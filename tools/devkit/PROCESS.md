# devkit 开发流程

> 配套 [DESIGN.md](DESIGN.md)（做什么）与 [ARCHITECTURE.md](ARCHITECTURE.md)（怎么搭）。本文讲**按什么顺序做、每步怎么单独验收**。

## 1. 三条原则

**① 一个切片 = 一条能跑的命令 + 一个测试 + 一行文档。** 三样缺一不合并。

**② 切片之间不共享未完成的地基。** 不许出现「A 期写了半截、B 期先依赖它」。

**③ 每期结束套件仍然可用。** 新 exe 独立演进，`sc2.py` 全程不动，直到第 10 期才接转发。

## 1.5 优先级修正：先做「读」，再做「写」

基准里有一条**快速通道**（`sc2agent-records/benchmark/BENCHMARK.md` §2.9），目标是最快拿到一张**能进游戏玩的自由日替换图**。

对照下来，这条通道要的四项能力，**原计划第 1–5 期一项都没有**：

| 快速通道需要 | 原计划里有没有 |
|---|---|
| 读 `.SC2Map` 目录结构 | ❌ 计划只覆盖 `Base.SC2Data/GameData` |
| 读 `Triggers` XML 找单位生成调用 | ❌ 套件只有 2 条触发器静态规则 |
| 读**第三方 `.SC2Mod`**（别的作者做的） | ❌ 完全没有 |
| 读**原始组件**取单位表 | ❌ 计划只读派生转储 |
| 有效值 ×2 | ✅ 第 4 期 |

**原计划第 5 期之前全在给「写入器」打地基，而玩上游戏不需要写入器**——补丁可以输出到 stdout，由人粘贴。

### 关键优化：开发不阻塞于人工步骤

自由日那张地图只能由人在编辑器里另存为 Components（M1），但**参考 mod `泽拉图之遗` 自带 21 张完整地图**，
每张都有真正的 `Triggers` XML（`‹TriggerData›` 根元素）和 `Base.SC2Data/GameData/*.xml`。

**所以 W1–W4 全部可以在参考 mod 上开发并测试，一行都不依赖 M1。** 只有 W5（对真自由日出补丁）需要 M1。

### 开工计划

| 周 | 做什么 | 验收（可自己敲） | 阻塞 |
|---|---|---|---|
| **W1** ✅ | `.SC2Mod` 读取器：全树扫描建 `(类型, id) → 文件` 索引 | `sc2cat find <mod> ZeratulZealot` 命中 1 条 `CUnit` | 无 |
| **W2** | `.SC2Map` 读取器：目录结构 / `Triggers` XML / 地图本地 GameData | 对 `paiur01.SC2Map` 打印文件结构与触发器树 | 无 |
| **W3** | 触发器扫描：列出单位生成点（动作 / 单位 ID / 玩家） | 对 `paiur01` 列出全部生成调用 | 无 |
| **W4** | 补丁生成器：替换单位 ID + 生成 catalog 覆盖行 | 对 `paiur01` 输出可直接粘贴的补丁 | 无 |
| — | **人**：编辑器另存自由日，拿到真地图 | — | 用户 |
| **W5** | 对真自由日跑 W3/W4 | 定位到运输机下来那 5 个 Marine 的生成点 | **M1** |
| — | **人**：应用补丁 → 编辑器验收 → 进游戏 | **L2 / L3** | 用户 |

**W1–W4 是纯读 + 文本生成，不写文件、不碰 XSD、不碰 LSP。**

### 与 13 期原计划的关系

```text
第 0 期   骨架 + 夹具（不变，W1 起需要它）
W1–W4     ★ 新增：第三方 mod / 地图 / 触发器 / 补丁生成
          → 人应用 + 进游戏   ← 到这里就能玩上了
--------------------------------------------------
第 1 期起  回到原计划（字段表 → 族表 → 拼接 → get → set → …）
```

原计划的第 1、2、9 期与 W 系列**无冲突**，可以并行。
但 DESIGN §10 已按 W1 的实测结论修正：**读用扫描索引，不用类型→文件映射**。

---
### 施工日志

**W1 完成（2026-10-17）**

```text
sc2cat types <root>            列类型统计
sc2cat find  <root> <id>       按 id 找 catalog 行（可 --type 过滤）
```

实现要点与实测：

| 项 | 值 |
|---|---|
| 扫描方式 | 遍历所有 `*.xml`，**看根元素是不是 `<Catalog>`**，不假设文件名 |
| 对 `泽拉图之遗` | 45,706 条 catalog 行 / 240 种类型，**0.11 秒** |
| 独立对账 | 与 Python `ElementTree` 计数**逐文件一致**（`AbilData.xml` 1096 = 1096） |
| 测试 | 5 个单元测试（自闭合行、非 Catalog 根、空 Catalog、注释、随意命名文件） |

**开发中抓到的两个真 bug**（都是测试/对账抓的，不是猜的）：

1. **自闭合行被漏掉**——`<CUnit id="X"/>` 走的是 `Event::Empty` 而非 `Start`，第一版只处理了 `Start`。
   这个 bug 让统计少了 92%（3646 vs 45706）。
2. **改了代码没重编二进制**——`cargo test` 只更新测试二进制，`sc2cat.exe` 还是旧的，
   导致修复看起来「没生效」。**每次改完要 `cargo build --release` 再验收。**

---
**W2 完成 + W3/W4 关键设计确认（2026-10-17）**

### 读哪里：Galaxy 而不是触发器图

`Triggers` 是**编译后的图**：`<FunctionCall Id="C03DAB8C"/>` 只是引用，真正身份是
`<FunctionDef Library="Ntve" Id="00000056"/>`——**没有可读名字**。直接解析它找「生成谁」成本很高。

而生成的 `MapScript.galaxy` 完全可读：

```galaxy
UnitCreate(8, "Zergling", 0, gv_pLAYER_07_ZERG_CINE, RegionRandomPoint(RegionFromId(29)), 30.0);
```

**单位是字面量字符串，玩家变量也可读。** 所以：**用 Galaxy 找，不用 Galaxy 改。**
（改它会被编辑器覆盖——见 `skills/sc2-map-triggers` 的红线。）

### 改哪里：Triggers 里的 Param 字面量

Triggers 里单位 id **是字面量**，形态固定：

```xml
<Element Type="Param" Id="4A9B7C33">
    <ParameterDef Type="ParamDef" Library="Ntve" Id="00000691"/>
    <Value>Zergling</Value>
    <ValueType Type="gamelink"/>
    <ValueGameType Type="Unit"/>
</Element>
```

**只有 `ValueGameType` 为 `Unit` 的 `<Value>` 才是单位引用。** 替换只改 `<Value>` 的文本，
按字节范围拼接，其余一字不动。

### 因此 W3/W4 重新定义

| 期 | 原定 | 改为 |
|---|---|---|
| W2 | 读 `.SC2Map` 结构 | ✅ 完成：`sc2cat layout` |
| W3 | 扫 Triggers 找生成点 | **改为扫 `MapScript.galaxy`**：`sc2cat spawns` |
| W4 | 生成补丁给人粘贴 | **改为直接精确替换**：`sc2cat set-unit`（符合「全程 CLI」纪律） |

### 新增命令

```text
sc2cat layout   <dir>                              看 .SC2Map/.SC2Mod 里有什么
sc2cat spawns   <map>                              列 UnitCreate 生成点（读 MapScript.galaxy）
sc2cat find-unit <map> <unit>                      列 Triggers 里的单位引用（只读）
sc2cat set-unit <map> --from A --to B [--dry-run]  精确替换，只改 <Value> 文本
```

### 参考素材（仓库外）

```text
sc2agent-records/benchmark/refs/kit_lotv_zeratul.SC2Mod/   65 文件 29.9 MB，含 ZeratulZealot
sc2agent-records/benchmark/refs/paiur01.SC2Map/            18 文件，含 Triggers 2.5 MB
sc2agent-records/benchmark/work/accept-01.ps1              人工验收脚本
```

---
**W3/W4 完成，第一个可人工验收的部分就绪（2026-10-17）**

### 派发结果（诚实记录）

| 子 agent | 任务 | 结果 |
|---|---|---|
| A | `galaxy.rs` 解析 `UnitCreate` | ✅ **一次成功**。7 个测试、11 处 UnitCreate 与独立 Python 计数一致 |
| B | `triggers.rs` + `edit.rs` | ❌ **退回重写** |

B 的产出有两个性质不同的问题：

1. **结构性错误**：它在 `ValueGameType` 处往上找**仍在栈上**的 `<Value>`，
   但 `<Value>Zergling</Value>` 在那之前**已经闭合出栈**了；而且 `<ValueGameType Type="Unit"/>` 是自闭合的，
   代码只处理非自闭合标签。结果是对任何输入都返回空。**它不是一行能修的。**
2. **契约与实现不符**：`splice` 的重叠检查没实现，有测试还断言了与契约相反的期望。

另外 B 写的两个中文测试函数名带了空格（`fn 空_edits 返回原文()`），编译不过。

**教训**：子 agent 交付后必须自己跑一遍它的验收命令。B 从未报告成功（它的消息没发出来），
而它的文件看起来「写完了」——`TODO` 标记没了、行数够了。**「看起来完成」不等于完成。**

### 端到端验收（`accept-01.ps1`，退出码 0）

用参考地图 `paiur01.SC2Map` 当替身：

```text
1 layout              地图结构：Triggers / MapScript / 13 个 GameData
2 spawns              11 处 UnitCreate（Zergling/Roach/Hydralisk/Mutalisk × 2 波）
3 find-unit Zergling  193 处 Unit 引用，Zergling 占 17 处（含字节范围）
4 set-unit --dry-run  17 处，且文件 sha256 不变
5 set-unit            真的落盘
6 改动相称            字节 -51 = 17×3；变化行数 17；每行差异恰好是那处替换
```

### 验收判据的一个坑（值得记下来）

第一版判据是「同一索引逐字节比对」——**错的**。替换让后面所有字节平移 3 位，
同索引比对报了 2,301,165 处「不同」。改成**行级比对**才对：行数不变、变化行数等于处数、
每行差异恰好等于那处替换。

### 新的独立对账结果

| 项 | 工具 | 独立 Python |
|---|---|---|
| `spawns` UnitCreate | 11 | 11 ✓ |
| `find-unit` 全部 Unit 引用 | 193 | 193 ✓ |
| `find-unit Zergling` | 17 | — |
| 替换后字节差 | -51 | 17 × (5-8) = -51 ✓ |

> 一个反直觉的事实：`<Value>Zealot</Value>` 这张图里只有 1 处，**且不属于 Unit 类型的 Param**，
> 所以 `find-unit Zealot` 报 0 是**正确的**。第一版验收脚本选错了演示单位，
> 换成 `Zergling` 才对——**工具报 0 不一定是工具错，可能是题目错。**

---
## 2. 每期的完成定义（Definition of Done）

| # | 要求 | 怎么证明 |
|---|---|---|
| 1 | 那条命令能跑，输出对 | 期表里的「你可以自己敲这条」 |
| 2 | 有测试，且测试能在**没装游戏、没编辑器**的机器上跑 | `cargo test` |
| 3 | 旧东西没坏 | `cargo test` + 现有 pytest 98 用例 + `sc2.py check` |
| 4 | 文档同步 | 那一期改了行为，就改 DESIGN/ARCHITECTURE 的对应段 |

## 3. 先建测试基座（第 0 期，不产生功能）

没有基座，后面每一期都只能「装个游戏试试」，那就不是「一点点测试」了。

```text
tools/devkit/
  tests/
    fixtures/
      Minimal.SC2Mod/                      ← 最小可运行项目
        ComponentList.SC2Components
        Base.SC2Data/GameData/UnitData.xml  ← 3 个 CUnit（1 个带父级）
        Base.SC2Data/GameData/AbilData.xml  ← 1 个 CAbilEffectTarget
    golden/
      set-lifemax.expected.xml             ← 文本拼接后的**完整文件**，逐字节比对
      new-cunit.expected.xml
    data/
      broken.xml                           ← 故意错，验 XSD 会报
```

**测试基座的三条硬要求：**

1. 夹具是**真实可编译的** XML（能被 `Catalog.xsd` 通过），不是随便写的片段
2. `edit.rs` 的断言是**整文本**，不是 `assert!(contains(...))`——拼接类工具最大的风险是悄悄多改一个空格
3. 不读本机路径，不联网，不依赖 `DataEditorXML`（那个 166 MB 不进测试）

> 唯一例外：`fields` 那一期要读真实 `Catalog.xsd`。它 8.4 MB、在仓库里，可以直接当测试输入。

## 4. 分期总表

| 期 | 产出 | 你可以自己敲这条验收 | 风险 |
|---|---|---|---|
| 0 | 骨架 + 夹具 + CI | `cargo test` 全绿、`cargo clippy` 无警告 | 无 |
| 1 | `sc2cat fields` / `types` | `sc2cat fields CUnit --filter life` | 无 |
| 2 | 族表 + 类型→文件 | `sc2cat types --filter abil` | 无 |
| 3 | `edit.rs` 文本拼接 | `cargo test edit`（golden 逐字节） | 低 |
| 4 | 项目定位 + `sc2cat get` | `sc2cat get CUnit Marine LifeMax` | 低 |
| 5 | `sc2cat set` | 对夹具跑 set，`git diff` 恰好一行 | **中** |
| 6 | `engine` + 旧 9 条规则 | `sc2cat check` 与 Python 版同结果 | 中 |
| 7 | 新 7 条检查器 | `sc2cat check` 报出夹具里的人为错误 | 低 |
| 8 | `sc2cat new` / `rm` | 对夹具跑 new，逐字节符合 golden | **中** |
| 9 | `sc2cat verify`（XSD） | 真实转储 0 错、broken.xml 必报 | 低 |
| 10 | `sc2.py catalog` 转发 + release 携带 | `python tools/sc2.py catalog fields CUnit` | 低 |
| 11 | `devkit-lsp` | 喂一帧 JSON-RPC，stdout 有诊断 | 中 |
| 12 | 跨依赖解析（搬 `sc2_dependencies`） | 对含依赖的夹具解析出父级 | **高** |

## 5. 每期展开

### 第 0 期：骨架与基座（不产生功能）

**做**：`cargo new`、`clap` 骨架（只有一个 `--version`）、夹具、golden 空目录、CI 里加 `cargo test`。
**不做**：任何业务逻辑。这一期唯一的判据是「空跑全绿」。
**验收**：`cargo test`（0 个测试也算绿）、`cargo clippy -- -D warnings` 零警告。
**可暂停**：✅ 完全可暂停。

### 第 1 期：字段表（纯读）

**做**：解析 `Catalog.xsd` → 内存字段表 → `fields` / `types` 两个子命令。
**关键点**：先用 `quick-xml` 流式扫，命中目标 complexType 就停；先**不**做 JSON 缓存（第 1 期还不知道要不要）。
**测试**：断言 `CUnit` 有 `LifeMax`（标量）、`CostResource`（数组，index 枚举含 Minerals/Vespene）、
`CAbilEffectTarget` 存在、总数 598 个可作根的族。
**验收**：`sc2cat fields CUnit --filter life` 打印 `LifeMax` 与说明。
**不做**：缓存、搜索、JSON 输出、`set`。

### 第 2 期：族表与类型→文件

**做**：18 族前缀表；`CAbilEffectTarget` → `Abil` → `AbilData.xml`；未归族的类型明确标为「未知」。
**测试**：18 族各抽一个类型断言映射对；未归族的 20+ 个断言返回「未知」而**不是猜一个**。
**验收**：`sc2cat types --filter abil` 列出 38 个 Abil 类型且都指向 `AbilData.xml`。
**不做**：把未归族硬凑成一个表——那 20+ 个要人工逐个定性，是独立的一件小事。

### 第 3 期：文本拼接（最该单独测的一期）

**做**：`roxmltree` 拿字节范围 → 插入一行/一段 → 输出新文本。**这一期不碰文件系统写**，纯函数：`(&str, 插入指令) -> String`。
**测试**：golden 逐字节比对三个场景——缩进是制表符的文件、缩进是 2 空格的文件、CRLF 文件。
**验收**：`cargo test edit` 全绿；故意改 golden 一个空格，测试必须红。
**可暂停**：✅

### 第 4 期：项目定位与 `get`

**做**：读 `agent-config.json` 找活动项目；在 `Base.SC2Data/GameData/*.xml` 里找 `(Type, Id)`；沿同 mod 的 `parent` 链解析有效值。
**测试**：夹具里 `Child` 继承 `Parent`，断言 `get` 报「Child 未定义，Parent 提供 120」。
**验收**：`sc2cat get CUnit Child LifeMax`。
**不做**：跨依赖（留第 12 期）。

### 第 5 期：`set`（第一次写文件）

**做**：串起 2+3+4：定位 → 校验字段 → 拼接 → 落盘。此时**才**引入 `--dry-run` 与拒绝契约。
**测试**：对夹具跑 set 后，**与 golden 整文件比对**；跑一个不存在的字段，断言退出码 1 且**文件字节未变**。
**验收**：对夹具 set 一次，`git diff` 恰好一行。
**这是第一个「会改用户文件」的期，所以拒绝契约的测试必须和写成功的测试一样多。**

### 第 6 期：`engine` + 移植旧 9 条规则

**做**：`Check` trait + 注册表；把 `sc2_checks.py` 的 9 条逐条移植；`sc2cat check`。
**测试**：把 Python 版的**同一批违规样例**拿来，断言两边都报、且 code 相同。
**验收**：`sc2cat check .` 与 `python tools/sc2.py check .` 对夹具输出等价。
**不做**：新规则（下一期）。

### 第 7 期：新 7 条检查器

**做**：`catalog.unknown-field` / `attribute-form` / `array-needs-index` / `index-value` / `duplicate-id` / `wrong-file` / `missing-anchor`。
**测试**：每条一个违规夹具 + 一个合规夹具。**特别测 `attribute-form`**——XSD 抓不到它，这里必须抓到。
**验收**：`sc2cat check` 在夹具上能报出人为埋的错。

### 第 8 期：`new` / `rm`

**做**：`new` 要决定插到哪个文件、插在同族哪个位置、缩进多少；`rm` 要处理「继承来的字段用 `removed="1"` 删」。
**测试**：golden 整文件比对；重名断言拒绝；跨文件重名也拒绝。

### 第 9 期：`verify`（XSD）

**做**：包装 `xsd-schema`；编译一次、校验整项目；报错格式保持 `XSD Schema Error in <file>:<line>: <msg>`（DESIGN 第 13.5 节）。
**测试**：真实转储 `Core Units.txt` 0 错、`Core Actors.txt` 0 错、自造 `broken.xml` 必报。
**验收**：`sc2cat verify` —— 这三条的耗时与结果应与我实测的一致（1.35 s 编译、255 KB/68 ms）。

### 第 10 期：接进 `sc2.py`

**做**：`_delegate` 支持直接执行 exe；`DELEGATES["catalog"]`；`release.py` 带上 `tools/bin/`。
**测试**：`python tools/sc2.py catalog fields CUnit` 与直接调 exe 输出一致。
**验收**：两种入口。

### 第 11 期：LSP

**做**：`lsp-server` 骨架 + 4 个消息（initialize / didOpen / didChange / didSave）+ publishDiagnostics；FULL 同步；150ms 防抖。
**测试**：往 stdin 喂 JSON-RPC 帧，断言 stdout 的 publishDiagnostics 内容。**不需要真编辑器。**
**验收**：`devkit-lsp` 对着夹具目录跑一轮。

### 第 12 期：跨依赖解析（风险最高，放最后）

**做**：搬 `sc2_dependencies.py` 的 `read_mod_dependencies` / `build_dependency_graph` / `_resolve_local_mod`；
让 `get` 能沿依赖链解析有效值。
**为什么最后**：它依赖前面所有的东西，而且**风险最高**——这是唯一一个「可能要读游戏数据文件」的能力。
**测试**：夹具里造一个 `Main.SC2Mod` 依赖 `Shared.SC2Mod`，断言跨 mod 的父级能解析。
**验收**：对夹具解析出跨依赖父级。

## 6. 里程碑 → 对应基准的哪个子任务

| 达成 | 就能开始试基准的 |
|---|---|
| 第 5 期 | 任务 1/2 的「血量 ×2」「改名」这类单字段改动 |
| 第 7 期 | 任务 1 的「写错文件/属性写法」被工具提前拦住 |
| 第 9 期 | 所有任务的 L1 判据（XSD 全过） |
| 第 12 期 | 任务 1/2/3/4 的**实体解析**这一步（基准的第一道墙） |

**注意：到第 12 期为止，四条基准任务仍然完不成。** 缺的是跨族事务性编辑（任务 3）、
Actor 写入（任务 1D/2B）、面板接线（任务 4B/4C）——那三块在 DESIGN 第 12 节里是第 7、8 期的事。

## 7. 回退与并行

* 每期一个独立 commit，前缀 `feat(devkit): 第 N 期 …`，可单独 revert
* 第 1、2、3、9 期**互不依赖**，可以并行（三个 agent 各做一期，只要第 0 期先落）
* 第 4 期起开始串行，因为都要 `project.rs` + `edit.rs`

## 8. 不做的事（避免流程膨胀）

* 不写「每周进度会」之类的流程文档——期表就是进度
* 不建 workspace、不建 feature flag、不做兼容层
* 不在第 0 期就把 13 期的目录都建出来——建了也是空的