# sc2cat 设计文档

> 状态：**设计定稿，尚未实现**。本文只定决策，不含实现。

## 1. 要解决什么

agent 今天写 GameData XML 是**盲写加事后纠正**：先读 40–90 KB 技能文档知道形状，凭记忆手写片段，
再由 `sc2.py check`（9 条静态规则，**不查字段是否存在**）和 `validate-mod.py`（XSD）事后抓错。

失败模式全部集中在「写」这一步：写错文件、属性写成子元素、数组漏 `index=`（**静默错位**）、
字段名拼错、非 ASCII、重复 `(type, id)`、漏字符串锚点。

**目标：把「事后检查」前移成「事前只能这么写」。** 与 `sc2.py rules` 同一条原则，只是从「验证」往上游走一格到「生成」。

## 2. 边界

**做**：GameData XML 的编写与查询——新建行、改值、加数组项、删继承项、查某族有哪些字段、查某字段的生效值出处。

**不做**：

- 不复刻 `sc2.py` 的 14 个转发动作（校验、打包、部署仍走 Python）
- 不做模板向导生成整个兵种——那是 `sc2data-wizards` 的活
- 不做 XML 美化器 / 重排器
- **不做通用 XPath CLI**：那等于让 agent 在更高一层继续手写，等于没解决
- 不做 Galaxy 脚本编辑（另有考虑，见第 13 节）

## 3. 为什么是 Rust + 独立 exe

1. **不依赖 Python/lxml。** 套件里其他工具要 Python，但「写一行 XML」不该被这个绑架。
2. **启动快。** agent 会高频调用，每次都要付 Python 解释器的启动成本。
3. **单文件分发。** 编译产物直接进 release，与安装器同一套路子。

### 一个反直觉的点：不需要 XSD 校验器

Rust 生态没有成熟的 XSD 1.0 校验器，一开始看像拦路虎。但**写工具不需要通用校验器**——
它自己生成片段，正确性由**构造**保证，不由事后校验保证。

所以 `Catalog.xsd` 的用法是**当成数据读**（字段表、数组标记、index 枚举、声明顺序），
不是当成校验器跑。校验退化成几条 assert 式自检：字段在族里、数组必有 index、index 在枚举内、结果能被自己解析回来。

## 4. 数据源

| 源 | 用途 | 必需 |
|---|---|---|
| `tools/schemas/sc2-xsd/Catalog.xsd` | 字段表：存在性、标量/数组、`index` 枚举、声明顺序、字段说明 | 是 |
| 活动项目的 `Base.SC2Data/GameData/*.xml` | 定位对象、父级链、重名检查、插入点 | 写操作必需 |
| `agent-config.json` | 找活动项目 | 否（`fields`/`types` 不需要） |
| `DataEditorXML/*.txt` | 族表推导与校验 | 否（仅构建/校验期） |

实测规模：`Catalog.xsd` 8.4 MB / 269,375 行 / 854 complexType，**598 个可作 catalog 根**；
`DataEditorXML` 157 个顶层转储 / 32 MB。

### 4.1 不建索引缓存

每次调用用 `quick-xml` 流式扫 `Catalog.xsd`，命中目标 complexType 就停。
8.4 MB 的流式扫描按经验在百毫秒量级，比维护一份生成索引便宜。

> shortcut: 若实测超过 300 ms，改为生成 `catalog-schema.json` 紧凑索引，
> 并由 `refresh-sc2-reference.py` 那条线一起刷新。

## 5. 命令面

```text
sc2cat types   [--filter <s>]                    列出可写的 catalog 族（598 个类型）
sc2cat fields  <Type> [--filter <s>] [--json]    该族字段表：名称 / 标量或数组 / index 枚举 / 说明
sc2cat get     <Type> <Id> [<Field>] [--json]    对象是否存在；字段的生效值出处
sc2cat new     <Type> <Id> [--parent P] [--set ...] [--file F] [--dry-run]
sc2cat set     <Type> <Id> <F=V>... [--remove] [--file F] [--dry-run]
sc2cat rm      <Type> <Id> [--file F] [--dry-run]
```

### 赋值语法照抄 XML，不发明新语法

```bash
sc2cat set CUnit AeonMarine LifeMax=250                       # 标量
sc2cat set CUnit AeonMarine 'CostResource[Minerals]=50'       # 数组项
sc2cat set CUnit AeonMarine 'CostResource[Vespene]' --remove  # 删继承项（写 removed="1"）
```

### 退出码

| 码 | 含义 |
|---|---|
| 0 | 成功（查询返回结果，或写入完成） |
| 1 | **拒绝**：字段不存在、重名、文件不可写、结果无法解析。**必须保证文件未被改动** |
| 2 | 用法错误 |

### 输出契约

- 机器可读优先：默认简洁文本，末尾附一行 `wrote <file>: +<n> -<m>`；`--json` 给结构化形态
- 拒绝时 stdout 为空，stderr 首行以 `error: ` 开头，**第二行必须是「文件未改动」**
- `get` 的继承链用树形打印，明确标出生效来源（见第 7 节）

## 6. 写语义

### 6.1 文本拼接，不做 DOM 往返

用 `roxmltree` 解析拿到目标节点的**字节范围**，然后在原文本上按字节插入。
理由：lxml/ElementTree 写回会重排缩进与属性顺序，而这个项目明确在意编辑器往返安全。
拼接后**无关部分逐字节不变**，git diff 一眼可读。

> 仓库里已有的 `repair_duplicate_trigger_ids.py` 就是这个路子（正则 + 文本替换），
> 这里只是把「找位置」从正则换成解析器的字节范围。

### 6.2 插入位置

| 插什么 | 插到哪 |
|---|---|
| 新对象 | 同 `Type` 的最后一个元素之后（同族聚拢） |
| 新字段 | **按 `Catalog.xsd` 的声明顺序**（编辑器规范顺序）插到最近的邻居之间 |
| 数组项 | 同名字段的最后一个之后 |

### 6.3 缩进与换行

取自文件自身：探测同族首个元素的缩进宽度与字符（制表符还是空格）、文件主导换行符（CRLF/LF）。不写死 4 空格。

### 6.4 默认落盘，`--dry-run` 预览

权衡过「默认预演」，否掉了：那会让每次改动变成两次调用，而这次改动本身是**可审阅的**
（文本拼接、git diff 完整可见）且**正确性来自构造**。默认落盘，但每次都必须打印改了什么。

> shortcut: 若实测发现误写率高，改成默认预演 + `--write` 落盘。

## 7. 拒绝契约（收益最大的部分）

字段名拼错时，今天 agent 要：check 不报 → XSD 报一句 lxml 原始错误（不告诉你正确名字）→
grep `catalog-rules.md`（25.8 KB，那是规则不是字段表）→ grep 32 MB 转储。

```text
$ sc2cat set CUnit AeonMarine SpeedBoost=1.5
error: CUnit 没有字段 SpeedBoost
  最接近：Speed / SpeedMultiplier / SpeedCreep
  文件未改动
```

这条把「猜 → 报错 → 大海捞针」换成「问 → 得到确切名字」。

## 8. 类型 → 目标文件映射

这是动工前必须先补的缺口：映射今天只在 `audit-actor-and-card-integrity.py` 里硬编码了
`UnitData.xml`/`AbilData.xml` 两处，还带 `GameData.xml` 兜底。

**实测可推**：从 `DataEditorXML` 转储抽族表，得到 **18 族**：

| 族 | 类型数 | 目标文件 |
|---|---|---|
| Actor | 101 | `ActorData.xml` |
| Validator | 92 | `ValidatorData.xml` |
| Abil | 38 | `AbilData.xml` |
| Effect | 34 | `EffectData.xml` |
| Requirement | 26 | `RequirementData.xml` |
| Behavior | 16 | `BehaviorData.xml` |
| Mover | 4 | `MoverData.xml` |
| Weapon | 3 | `WeaponData.xml` |
| Unit、Button、Footprint、Location、Objective、Sound、Turret、Upgrade、ArmyCategory、ArmyUnit | 各 1 | `<族>Data.xml` |

归族靠**显式族前缀表**，不靠机械规则：`CAbilEffectTarget` 属 Abil 而非 AbilEffectTarget，
`CValidatorCombine` 属 Validator，用「取 `C` 之后的最长已知族前缀」匹配。

**未归族约 20+ 个**（`CModel`、`CTexture`、`CCommander`、`CMap`、`CDataCollection*`、`CGameUI`、
`CTactical*`、`CBankCondition*`、`CAttachMethod*` …）。**实施时必须逐个确认**它们是「另一个 Data 文件」
还是「根本不是 GameData 行」（`CModel`/`CTexture` 很可能是编辑器资产目录，写不进 GameData）。
确认前这些类型一律拒绝，要求 `--file` 显式指定。

## 9. 与 Python 侧的关系

单一门面不破：给 `sc2.py` 加一个转发动作，agent 在套件里仍然只记一条命令。

```python
# tools/sc2.py
DELEGATES["catalog"] = ("../bin/sc2cat.exe", "编写 GameData XML")
```

需要改 `sc2.py` 的 `_delegate`：它现在固定 `sys.executable <script>`，要支持「直接执行 exe」这一形态。
独立使用时 agent 直接调 `tools/bin/sc2cat.exe`；两种入口行为必须一致。

## 10. 分发与版本

| 项 | 决定 |
|---|---|
| 源码 | `tools/sc2cat/`（Cargo crate） |
| 产物 | `tools/bin/sc2cat.exe`（不进版本库，随 release 分发） |
| 版本 | 独立于套件版本；`--version` 打印自身版本与**所依赖 `Catalog.xsd` 的 sha256** |
| 发布 | `release.py` 打 core 包时带上 `tools/bin/` |

`Catalog.xsd` 是 SC2 版本的快照，暴雪改 schema 后不会自动更新。
把它的 sha256 打进 `--version` 输出，schema 换了能被发现。

## 11. 测试

- **单元**：schema 解析（字段表/数组标记/枚举）、族匹配、拼接位置计算、缩进探测
- **夹具**：一个最小 `UnitData.xml`，跑 new/set/rm 后断言**输出文本逐字节符合预期**
  （拼接类工具最大的风险是悄悄多改了一个空格）
- **回归**：把「属性写成子元素」「漏 index」「非 ASCII」「重复 id」四个历史失败模式各做成一个用例，断言被拒绝
- 现有 9 条静态规则 + pytest 继续跑；sc2cat 的输出必须过 `sc2.py check` 与 XSD 校验

## 12. 分期

| 期 | 内容 | 风险 | 前置 |
|---|---|---|---|
| 1 | `types` / `fields`（纯读） | 无 | 无 |
| 2 | 族表确认（含未归族 20+ 个逐一定性） | 无 | 无 |
| 3 | `get`（本地 + 同 mod 父级链） | 无 | 1 |
| 4 | `set` / `new` / `rm`（写） | 中 | 2、3 |
| 5 | `sc2.py catalog` 转发 + release 携带 exe | 低 | 4 |

## 13. 未决

1. **`get` 的跨依赖继承链**：本期只走同 mod 的 `parent`。跨依赖需要依赖图，Python 侧有 `sc2_dependencies.py`，
   Rust 侧要么复刻要么调用它。倾向：本期返回「父级在依赖 X 里，未解析」，明确标注而不是假装知道。
2. **名字**：`sc2cat` 是提案；备选 `sc2-catalog`（与 `sc2-catalog-query.py` 一致但更长）。
3. **是否吸收更多动词**：若这期成立，`sc2.exe` 作为统一前门是自然演进，但会重新引入 Python 依赖。
4. **字符串锚点**：新 catalog ID 需要 `GameStrings.txt` + `ObjectStrings.txt` 的键。本期 `new` 只写 XML 里的锚点值
   并**打印期望的键**，实际填字符串仍交给 `audit-gamestrings-anchors.py --fill`（它有既有测试覆盖）。