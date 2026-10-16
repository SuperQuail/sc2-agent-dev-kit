# devkit 代码架构

> 配套 [DESIGN.md](DESIGN.md)。那份讲**做什么、为什么**，这份讲**怎么搭**。

## 1. 中心思想：一个诊断引擎，三个门面

LSP 不是「另一个功能」，而是**同一个引擎的第二个门面**。三条消费路径共用一套规则：

```text
                       ┌──────────────────────┐
    写完之后立刻看 ────▶│                      │
    （agent，同一次调用）│   engine::diagnose   │
                       │   Document → Vec<Diag>│
    整个项目批量看 ────▶│                      │
    （sc2cat verify）   └──────────┬───────────┘
                                  │
    编辑器里实时看 ────────────────┘
    （devkit-lsp，publishDiagnostics）
```

**绝不能出现三份实现。** 规则只写在 `checks/` 里，三个门面都只负责「把文档喂进去、把诊断拿出来」。

## 2. 一个 crate，两个可执行

```text
tools/devkit/
  Cargo.toml            [package] name = "devkit"
                        [[bin]] sc2cat      → src/bin/sc2cat.rs
                        [[bin]] devkit-lsp  → src/bin/lsp.rs
  DESIGN.md             决策
  ARCHITECTURE.md       本文
  src/
    lib.rs              对外只导出 engine / diag / schema 三块

    diag.rs             ★ Diagnostic / Severity / Code —— LSP 形状的诊断模型
    document.rs         文档抽象：路径 + 文本 + 字节↔行列映射
    project.rs          活动项目、GameData 目录、依赖链
    xmldoc.rs           quick-xml 解析 + roxmltree 字节范围

    schema/
      mod.rs            字段表类型（Field / CatalogType / Family）
      build.rs          从 Catalog.xsd 生成 catalog-schema.json
      load.rs           加载 + sha256 校验 + 「过期就报错」

    catalog.rs          字段查询、类型→文件、有效值（父级链）
    edit.rs             ★ 文本拼接写入器（位置计算、缩进/换行探测、拒绝契约）
    validate.rs         XSD 校验（包装 xsd-schema）

    checks/
      mod.rs            Check trait + 注册表（rules 的唯一真源）
      xml.rs            xml.ascii / xml.banned / xml.catalog-not-empty / trigger.gui-first
      galaxy.rs         galaxy.include / utf8-no-bom / line-length / trigger-name
      catalog.rs        ★ 新增的 7 条（见 `5）
      guard.rs          guard.protected

    engine.rs           ★ 拱心石：diagnose(doc, ctx) -> Vec<Diagnostic>

    bin/
      sc2cat.rs         CLI（clap）
      lsp.rs            LSP server（lsp-server）
```

**为什么先是一个 crate 而不是 workspace**：现在只有两个 bin 共用一套库，拆 workspace 只会换来更长的构建命令。

> shortcut: 当「核心库改一行就要重编两个 bin + 全部测试」开始碍事时，按 `core / schema / checks / cli / lsp` 拆 workspace。

## 3. 引擎的签名

```rust
// engine.rs —— 三条路径唯一的交汇点
pub fn diagnose(doc: &Document, ctx: &Context) -> Vec<Diagnostic>;
```

* `Document`：路径 + 全文 + 字节↔行列映射（诊断要有 range，就得先有映射）
* `Context`：字段表、活动项目、已解析的依赖 —— **只读**，由调用方加载一次复用
* 返回 LSP 形状的 `Diagnostic`，即使调用方是 CLI

刻意**不传**`&mut`：诊断是纯函数，能并行、能缓存、能在测试里直接断言。

## 4. 诊断模型

```rust
pub struct Diagnostic {
    pub range: Range,          // 1-based 行列；LSP 要求这个形状
    pub severity: Severity,    // Error | Warning | Information | Hint
    pub code: &'static str,    // 规则 id，与 sc2_checks 一致，如 "catalog.unknown-field"
    pub source: &'static str,  // 固定 "sc2kit"
    pub message: String,
    pub suggestions: Vec<String>,   // 给 code action / CLI 的「最接近：Speed」
}
```

**严重度分工**（这条决定了 agent 会不会被噪声淹没）：

| 级别 | 什么算 | 例子 |
|---|---|---|
| Error | 写入器拒绝执行 / XSD 不通过 | 字段不存在、重复 `(type,id)`、非 ASCII |
| Warning | 语法合法但几乎肯定不是本意 | 数组漏 `index=`、取值写成属性、写错文件 |
| Information | 缺了通常要补的东西 | 面向玩家的字段没有字符串锚点 |

## 5. 新增的检查器（写入器才能防住的那批）

实测确认 XSD **抓不到**「属性写成子元素」（因为 `CBase` 上有 `xs:anyAttribute processContents="skip"`），
也抓不到字段顺序（`xs:choice`）。这批只能靠引擎：

| id | 抓什么 | 依据 |
|---|---|---|
| `catalog.unknown-field` | 字段不在该族的声明里 | 字段表 |
| `catalog.attribute-form` | ★ 该是子元素的取值写成了属性 | 字段表（XSD 抓不到） |
| `catalog.array-needs-index` | 数组字段漏 `index=` | 字段表 `maxOccurs` |
| `catalog.index-value` | `index` 不在枚举里 | 字段表的 `simple_E*` |
| `catalog.duplicate-id` | 同一 catalog 里 `(type,id)` 重复 | 既有规则 3 |
| `catalog.wrong-file` | `CUnit` 写进了 `AbilData.xml` | 族表 |
| `catalog.missing-anchor` | 面向玩家的字段缺字符串锚点 | 既有规则 10 |

## 6. 包清单

### 必选（10 个）

| 用途 | 包 | 版本 | 为什么是它 |
|---|---|---|---|
| CLI | `clap` | 4.x | 事实标准，derive 宏省掉手写解析 |
| 序列化 | `serde` + `serde_json` | 1.x | 字段表、诊断 JSON、LSP 消息都要 |
| XML 流式 | `quick-xml` | 0.42 | 最快；**且 `xsd-schema` 已经在用同一版本** |
| XML 只读树 | `roxmltree` | 0.21 | `node.range()` 给出字节范围，是文本拼接的前提 |
| XSD | `xsd-schema` | 0.2.3 | 唯一可用的纯 Rust XSD 校验器，已实测（见 DESIGN `2） |
| LSP 传输 | `lsp-server` | 0.10 | rust-analyzer 的脚手架，**同步、不拖 tokio**；2026-10 仍在更新 |
| 错误 | `anyhow` + `thiserror` | 1.x / 2.x | 库用 thiserror，bin 用 anyhow |
| 摘要 | `sha2` | 0.11 | schema 摘要 + 安装器已在用 |

### LSP 类型：不用 `lsp-types`，自己定义

| 候选 | 版本 | 更新 | 判断 |
|---|---|---|---|
| `lsp-types` | 0.97.0 | **2024-06-04** | 停更两年 |
| `tower-lsp` | 0.20.0 | **2023-08-11** | 停更三年，且拖 tokio |

我们只需要约 8 个消息形状（initialize / initialized / shutdown / exit / didOpen / didChange / didSave / didClose / publishDiagnostics / codeAction / hover），
而 `lsp-server` 是**刻意类型无关**的（载荷就是 `serde_json::Value`）。
所以自己写那几个 `serde` 结构（约 120 行），换掉一个停更两年、还会拖进旧 `url` 的依赖。

> shortit: 若后来要补全、签名帮助、语义高亮这类大块能力，再回头评估 `lsp-types`。

### 建议（2 个）

| 用途 | 包 | 为什么 |
|---|---|---|
| 日志 | `tracing` + `tracing-subscriber` | **LSP 的 stdout 是协议通道**，日志必须走 stderr 或文件；这是它存在的理由 |

### 现在不做（等实测）

| 包 | 为什么不现在上 |
|---|---|
| `rayon` | 项目里 GameData XML 通常是十几个文件，串行够。**并发诊断等实测有痛点再说** |
| `walkdir` / `globset` | 只要一个目录下的 `*.xml`，`read_dir` + 扩展名判断足够 |
| `camino` | 用 `std::path`；序列化时转字符串即可 |
| `rusqlite` | 那是 catalog 图数据库的事，属于后续替换 `sc2-catalog-query.py` 的期 |
| `notify` | LSP 由编辑器推 `didChange`，不需要自己监听文件 |

## 7. LSP 设计

### 能力集（最小可用）

```rust
ServerCapabilities {
    text_document_sync: TextDocumentSyncKind::FULL,   // 见下
    code_action_provider: true,                        // 「最接近：Speed」的一键修复
    hover_provider: true,                              // 悬停显示该字段的 XSD 说明
}
```

### 刻意不做增量同步

LSP 的增量同步要求维护一份带偏移映射的文本缓冲并处理 range 更新，而我们的文件都是 KB 级。
**用 `FULL` 同步**：每次收到全文，重建 `Document`。省掉一整类「偏移算错」的 bug。

> shortcut: 若出现 >1 MB 的 GameData XML 且编辑卡顿，再上增量同步。

### 一次请求的流

```text
initialize(rootUri)      → 加载字段表 + 活动项目 + 依赖，存进 Context（只做一次）
didOpen / didChange      → 合并进「待诊断」槽位，投递到工作线程
didSave                  → 立即诊断（不等待防抖）
工作线程（150ms 防抖）   → engine::diagnose → publishDiagnostics(uri, diags)
codeAction(range)        → 读该 range 上的诊断 → 把 suggestions 变成 quick-fix
hover(position)          → 命中字段 → 打印字段表的说明文本
```

**防抖是必需的**：编辑器每次击键都发 `didChange`，不防抖就是把 8.4 MB 的字段表查到冒烟。
用「槽位 + 工作线程」而不是「每事件一个任务」：只处理最新的那一份，旧的自然作废。

### 跨文件诊断暂不做

「这个 `CUnit` 引用了不存在的技能」需要项目级索引（今天是 `sc2-catalog-query.py unresolved` 从图数据库答的）。
本期 LSP 只做**单文件内可判定的**诊断。跨文件留到吸收 catalog 图之后。

## 8. 写入即诊断（对 agent 收益最大的一条）

CLI 的一次调用同时完成**写入**与**诊断**，agent 不需要第二次调用去校验：

```bash
$ sc2cat set CUnit AeonMarine SpeedBoost=1.5
error: CUnit 没有字段 SpeedBoost
  最接近：Speed / SpeedMultiplier / SpeedCreep
  文件未改动                       ← 退出码 1，一个字节没动

$ sc2cat set CUnit AeonMarine LifeMax=250
wrote UnitData.xml: +1 -0
  UnitData.xml:42:5  warning  catalog.attribute-form  该写成子元素：<LifeMax value="250"/>
```

第二条是重点：**写成功但引擎仍在同一份输出里给出警告**。
这就是「把事后检查前移」的最终形态——不是把校验提前，而是让它根本不需要单独跑。

## 9. 测试策略

| 层 | 测什么 | 怎么测 |
|---|---|---|
| `schema/` | 字段表生成：标量/数组、index 枚举、声明顺序 | 对真实 `Catalog.xsd` 生成一次，抽查 `CUnit` / `CAbilEffectTarget` / `CBehaviorBuff` |
| `edit.rs` | ★ 拼接后**逐字节**符合预期 | 最小 `UnitData.xml` → new/set/rm → 断言整个输出文本 |
| `checks/` | 每条规则各一个违规样例 + 一个合规样例 | 防止「永远不报」和「永远报」 |
| `engine.rs` | 多规则叠加时的顺序与去重 | 一个文件同时犯 3 个错 |
| `validate.rs` | XSD 包装：真实转储全过、故意错必报 | 用 `DataEditorXML/Core Units.txt` 与自造的 `broken.xml` |
| LSP | 协议往返 | 直接喂 JSON-RPC 帧给 `devkit-lsp` 的 stdin，断言 stdout |

**拼接类工具最大的风险是悄悄多改一个空格**，所以 `edit.rs` 的测试必须是整文本断言，不是「包含」断言。

## 10. 与既有 Python 的过渡

移植不是重写。每一条规则、每一个检查都有对照：

| 今天 | 将来 | 过渡办法 |
|---|---|---|
| `tools/sc2_checks.py` 的 9 条 | `checks/{xml,galaxy,guard}.rs` | 逐条移植，**用同一批违规样例**断言两边结果一致 |
| `tools/validate-mod.py` 的 XSD 半边 | `validate.rs` | 报错格式要保（见 DESIGN `13.5） |
| `tools/sc2.py rules` | `sc2cat rules` | 输出格式照旧，agent 的读法不变 |

`sc2.py` 在每个阶段都必须可用（DESIGN `3）。
