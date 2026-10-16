# sc2cat 设计文档

> 状态：**设计定稿，尚未实现**。本文只定决策，不含实现。

## 0. 两条目标，分远近

**远期**：把套件的**全部 Python 换成 Rust**，使用者**完全不需要配置 Python 环境**。
现在的 `python tools/sc2.py …` 是使用者必须迈过的第一道门槛，而它只为工具链存在，不为功能存在。

**近期**：sc2cat 是这条迁移的**第一个模块**——它先拿下「编写 GameData XML」这块，
验证整套做法（单 exe、零运行时、schema 驱动）能不能成立。所以本设计不只写 sc2cat 自己，
还要写**它在这条迁移路线上的位置**。

## 1. 现有 Python 面盘点（实测）

| 项 | 实测 |
|---|---|
| `tools/*.py` 脚本 | 约 40 个（不含 18 个测试） |
| **第三方依赖** | **只有 `lxml` 一个**，1 处，就在 `validate-mod.py` 的 XSD 校验 |
| 其余依赖 | 全是标准库（pathlib / json / re / xml / sqlite3 / zipfile / hashlib / subprocess …）或仓库内同级模块 |

**这个结论很重要：迁移的第三方依赖面几乎为零。** 标准库里需要 Rust 替代的几项都有成熟 crate：

| Python | Rust |
|---|---|
| `xml.etree` / `lxml` | `quick-xml`（流式）、`roxmltree`（带字节范围的只读树）、`xsd-schema`（XSD） |
| `sqlite3` | `rusqlite` |
| `zipfile` | `zip`（**安装器已经在用**） |
| `hashlib` | `sha2`（**安装器已经在用**） |
| `argparse` | `clap` |
| `json` / `tomllib` | `serde_json` |
| `subprocess` / `tempfile` / `ctypes`（Win32） | `std` + `windows-sys` |

## 2. XSD 问题：已解决（实测）

这是迁移里唯一的硬骨头——Rust 生态长期没有可用的 XSD 校验器。**它现在有了，而且拿真实数据验过。**

### 选型

| crate | 版本 | 说明 |
|---|---|---|
| **`xsd-schema`** | 0.2.3 | 纯 Rust，XSD 1.0/1.1 + PSVI + 内建 XPath 2.0；W3C 一致性测试 39,458/39,510（99.95%） |
| `xmlschema` | 0.0.9 | 纯 Rust，zero unsafe，但 0.0.x 太早 |
| `libxml` | 0.3.22 | libxml2 绑定（lxml 用的就是它），但引入 C 依赖，破坏「单 exe 零环境」 |

选 `xsd-schema`，且用 **`default-features = false`（XSD 1.0 模式）**：
我们这份 schema 里 `xs:assert` 出现 0 次，不需要 XPath 引擎。

### 实测结果（本机，真实文件）

```text
编译 Catalog.xsd          8,587,953 字节  1.35 s   峰值内存 113 MB   零错误
  types_resolved: 23752   complex_type_inline_derivations: 4096

校验 Core Units.txt          16,696 字节   12.4 ms   错误 0
校验 Core Actors.txt        255,048 字节   67.6 ms   错误 0      ← 约 3.8 MB/s

负例 broken.xml                 232 字节    5.3 ms   错误 2
  cvc-complex-type.2.4  Element 'SpeedBoost' is not allowed at this position in the content model
  cvc-elt.1             Element 'SpeedBoost' is not declared    path=/Catalog/CUnit/SpeedBoost
```

### 依赖面（`default-features = false`）

```text
ahash base64 bitflags chrono hex num-bigint once_cell
quick-xml regex roxmltree rust_decimal slotmap thiserror
```

**13 个纯 Rust crate，无 `cc`、无 libxml2、无 C 工具链。**
（`build.rs` 只在 `xsd11` feature 下跑 LALRPOP，我们关掉了。）

### 一个必须记住的缺口：XSD 抓不到「属性写成子元素」

负例里 `<CUnit id="Bad1" LifeMax="100"/>` **没有**被报错——因为 `CBase` 上有
`<xs:anyAttribute processContents="skip"/>`，**schema 本身就放行任意属性**。同理，字段顺序因 `xs:choice` 也不参与校验。

**所以：XSD 能抓「不存在的元素名、类型错误、枚举越界、基数、唯一性约束」；**
**抓不到「把取值写成属性、写得乱七八糟但合法」。后者只能靠写入器从构造上杜绝。**
这条直接决定了 sc2cat 必须是**写入器**，不能只是「写完之后跑一次校验」。

## 3. sc2cat 在这条迁移里的位置

```text
现在      python tools/sc2.py …                  ← 使用者要装 Python
第 1 步    + tools/bin/sc2cat.exe                 ← 新增 Rust 工具，Python 仍在
第 2 步    sc2cat.exe 吸收只读动词（rules / check / query / …）
第 3 步    sc2cat.exe 吸收写动词与 validate（含 XSD）→ Python 只剩构建/发布
终点      devkit.exe 单一前门，零 Python
```

**每一期都必须保持 `sc2.py` 可用**，否则套件在迁移中途不能用。

### 缓存决策：从「不建索引」翻转为「建」

早期设计写的是「不建索引缓存，每次流式扫 XSD」。实测推翻了它：
**编译一次 `Catalog.xsd` 要 1.35 秒**，而 agent 一个任务里会调十几次 `sc2cat`。

| 用途 | 走哪条路 |
|---|---|
| `fields` / `set` / `new`（结构判断） | 读**生成的紧凑字段表** `catalog-schema.json`（毫秒级） |
| `verify`（真 XSD 校验整项目） | 用 `xsd-schema` 编译一次（1.35 s），一次调用校验整个项目 |

字段表由 `sc2cat schema build` 生成，并把 `Catalog.xsd` 的 sha256 写进去；加载时比对，
schema 换了就要求重建。**这让「工具的答案过期」变成可发现的事件，而不是静默错误。**

## 4. sc2cat 要解决什么

agent 今天写 GameData XML 是**盲写加事后纠正**：先读 40–90 KB 技能文档知道形状，凭记忆手写片段，
再由 `sc2.py check`（9 条静态规则，**不查字段是否存在**）和 XSD 事后抓错。

失败模式全在「写」这一步：写错文件、属性写成子元素、数组漏 `index=`（**静默错位**）、
字段名拼错、非 ASCII、重复 `(type, id)`、漏字符串锚点。**目标：把「事后检查」前移成「事前只能这么写」。**

## 5. 边界

**做**：GameData XML 的编写与查询——新建行、改值、加数组项、删继承项、查字段表、查生效值出处、XSD 校验。

**不做**：

- 不做模板向导生成整个兵种——那是 `sc2data-wizards` 的活
- 不做 XML 美化器 / 重排器
- **不做通用 XPath CLI**：那等于让 agent 在更高一层继续手写，等于没解决
- 不做 Galaxy 脚本编辑（第 3 步再议）

## 6. 数据源

| 源 | 用途 | 必需 |
|---|---|---|
| `tools/schemas/sc2-xsd/Catalog.xsd` | 唯一 schema 真源 | 是 |
| `catalog-schema.json`（生成） | 毫秒级字段表 | 是（缺则要求先 build） |
| 活动项目的 `Base.SC2Data/GameData/*.xml` | 定位对象、父级链、重名、插入点 | 写操作必需 |
| `agent-config.json` | 找活动项目 | 否（`fields`/`types` 不需要） |

实测规模：`Catalog.xsd` 8.4 MB / 269,375 行 / 854 complexType，**598 个可作 catalog 根**；
`DataEditorXML` 157 个顶层转储 / 32 MB。

## 7. 命令面

```text
sc2cat types   [--filter <s>]                     列出可写族（598 个类型）
sc2cat fields  <Type> [--filter <s>] [--json]     字段表：名称 / 标量或数组 / index 枚举 / 说明
sc2cat get     <Type> <Id> [<Field>] [--json]     对象是否存在；字段的生效值出处
sc2cat new     <Type> <Id> [--parent P] [--set …] [--file F] [--dry-run]
sc2cat set     <Type> <Id> <F=V>… [--remove] [--file F] [--dry-run]
sc2cat rm      <Type> <Id> [--file F] [--dry-run]
sc2cat verify  [--file F] [--json]                用真 XSD 校验（编译一次，校验整个项目）
sc2cat schema  build|status                       生成/检查字段表与它的 schema 摘要
```

赋值语法照抄 XML，不发明新语法：

```bash
sc2cat set CUnit AeonMarine LifeMax=250                       # 标量
sc2cat set CUnit AeonMarine 'CostResource[Minerals]=50'       # 数组项
sc2cat set CUnit AeonMarine 'CostResource[Vespene]' --remove  # 删继承项（removed="1"）
```

### 退出码

| 码 | 含义 |
|---|---|
| 0 | 成功 |
| 1 | **拒绝**：字段不存在、重名、结果无法解析。**必须保证文件未被改动** |
| 2 | 用法错误 |

## 8. 写语义

**文本拼接，不做 DOM 往返。** 用 `roxmltree` 拿目标节点的**字节范围**，在原文本上按字节插入。
lxml/ElementTree 写回会重排缩进与属性顺序，而这个项目明确在意编辑器往返安全。
拼接后**无关部分逐字节不变**，git diff 一眼可读。

> 仓库里已有的 `repair_duplicate_trigger_ids.py` 就是这个路子（正则 + 文本替换），
> 这里只是把「找位置」从正则换成解析器的字节范围。

| 插什么 | 插到哪 |
|---|---|
| 新对象 | 同 `Type` 的最后一个元素之后（同族聚拢） |
| 新字段 | 按 XSD 声明顺序插到最近邻居之间（编辑器规范顺序） |
| 数组项 | 同名字段的最后一个之后 |

缩进与换行**取自文件自身**：探测同族首个元素的缩进宽度与字符、文件主导换行符。不写死 4 空格。

**默认落盘，`--dry-run` 预览。** 这次改动可审阅（文本拼接、git diff 可见）且正确性来自构造，
所以默认落盘；每次都必须打印改了什么。若实测误写率高，翻转成默认预演。

## 9. 拒绝契约（收益最大的部分）

字段名拼错时，今天 agent 要：check 不报 → XSD 报一句不告诉你正确名字 → grep 25.8 KB 规则 → grep 32 MB 转储。

```text
$ sc2cat set CUnit AeonMarine SpeedBoost=1.5
error: CUnit 没有字段 SpeedBoost
  最接近：Speed / SpeedMultiplier / SpeedCreep
  文件未改动
```

把「猜 → 报错 → 大海捞针」换成「问 → 得到确切名字」。

## 10. 类型 → 目标文件映射

这是动工前必须先补的缺口：映射今天只在 `audit-actor-and-card-integrity.py` 里硬编码了两处。
**实测可推**：从转储抽族表，得到 **18 族**：

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

归族靠**显式族前缀表**（`CAbilEffectTarget` 属 Abil，`CValidatorCombine` 属 Validator），不靠机械规则。

**未归族约 20+ 个**（`CModel`、`CTexture`、`CCommander`、`CMap`、`CDataCollection*`、`CGameUI`、`CTactical*` …）。
**实施时必须逐个确认**它们是「另一个 Data 文件」还是「根本不是 GameData 行」
（`CModel`/`CTexture` 很可能是编辑器资产目录）。确认前一律拒绝并要求 `--file`。

## 11. 分发与版本

| 项 | 决定 |
|---|---|
| 源码 | `tools/devkit/`（Cargo crate，见 [ARCHITECTURE.md](ARCHITECTURE.md)） |
| 产物 | `tools/bin/{sc2cat,devkit-lsp}.exe`（不进版本库，随 release 分发） |
| 版本 | 独立于套件版本；`--version` 同时打印 `Catalog.xsd` 与 `catalog-schema.json` 的 sha256 |
| 发布 | `release.py` 打 core 包时带上 `tools/bin/` |

## 12. 分期（迁移视角）

| 期 | 内容 | 风险 | 前置 |
|---|---|---|---|
| 1 | `types` / `fields`（纯读，走生成的字段表） | 无 | 无 |
| 2 | 族表确认（含未归族 20+ 个逐一定性） | 无 | 无 |
| 3 | `schema build` + `get`（本地 + 同 mod 父级链） | 无 | 1 |
| 4 | `set` / `new` / `rm`（写） | 中 | 2、3 |
| 5 | `verify`（用 `xsd-schema` 替换 `validate-mod.py` 的 XSD 那一半） | 中 | 4 |
| 6 | `sc2.py` 转发 + release 携带 exe | 低 | 4 |
| 7 | 吸收只读动词（`rules` / `check` / `query` / `strings` …） | 中 | 6 |
| 8 | 吸收 Galaxy 语法校验与其余动词 → `devkit.exe` | 高 | 7 |

## 13. 未决

1. **`get` 的跨依赖继承链**：本期只走同 mod 的 `parent`。跨依赖需要依赖图，Python 侧有 `sc2_dependencies.py`。
   倾向：本期返回「父级在依赖 X 里，未解析」，明确标注而不是假装知道。
2. **名字**：`sc2cat` 是提案；若终点是单一前门，`devkit.exe` 更合适（与安装器同前缀）。
3. **字段表格式**：JSON 简单但大；考虑 `bincode` / `postcard`。等实测体积再定。
4. **字符串锚点**：新 catalog ID 需要 `GameStrings.txt` + `ObjectStrings.txt` 的键。本期 `new` 只写 XML 锚点值并**打印期望的键**。
5. **XSD 报错格式**：`xsd-schema` 报的是 W3C 错误码与元素路径，而今天 `validate-mod.py` 的
   `XSD Schema Error in <file>:<line>: <msg>` 有既有测试与文档引用。迁移时要么保持这个格式，要么改文档与测试——不能默默换掉。