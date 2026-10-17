# devkit 开发流程

> 配套 [DESIGN.md](DESIGN.md)（做什么）与 [ARCHITECTURE.md](ARCHITECTURE.md)（怎么搭）。本文讲**按什么顺序做、每步怎么单独验收**。

## 1. 三条原则

**① 一个切片 = 一条能跑的命令 + 一个测试 + 一行文档。** 三样缺一不合并。

**② 切片之间不共享未完成的地基。** 不许出现「A 期写了半截、B 期先依赖它」。

**③ 每期结束套件仍然可用。** 新 exe 独立演进，`sc2.py` 全程不动，直到第 10 期才接转发。

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