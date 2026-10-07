# 校验流水线

## `init-project.py`（默认的项目设置入口）

传入主 Components `.SC2Mod` 文件夹路径。该工具推断标准 SC2 布局，递归校验本地组件依赖，并且只在检查通过之后才原子地写 `agent-config.json`。用 `--dry-run` 预览；只有非标准布局才用手工目录选项。不要让用户手工编辑 JSON 或列举依赖。

## `test-suite.py`（选择受影响的作用域）

统一预检套件。它读 `agent-config.json`，解析配置的主模组，打印配置与确切目标，找不到模组就失败。它遵循 `validation.include_dependencies` 与 `validation.exclude_mods`；单次运行可用 `--primary-only`、`--include-dependencies` 或可重复的 `--exclude-mod` 覆盖。它跑：

- 工具单元测试
- `validate-agent-config.py` — 可移植配置、主模组与递归组件依赖检查
- `validate-mod.py` — 主 Mod 及所选依赖的 XML schema + Galaxy 语法
- `audit-actor-and-card-integrity.py` — 主 Mod 的命令卡槽位冲突（文件名是历史遗留；不做 Actor 绑定断言）
- `audit-gamestrings-anchors.py` — 本地化锚点检查
- `check-doc-links.py` — 断裂的 Markdown 链接

本地改动可用 `--scope tools|docs|mod` 或聚焦回归。编辑器交接前要校验真正的组件改动。跨模块改动与完整验收遵循 `AGENTS.md` 里的执行层级。只读查询不需要这套件。

## `validate-mod.py`

全面的静态检查：

- 对 GameData XML 做正式的 W3C XSD schema 校验（`tools/schemas/sc2-xsd/Catalog.xsd`），用 `lxml` 或 `xmllint`；缺后端会让校验失败
- UI 布局 XML 解析，加上引擎兼容的根元素、顶层子元素与 ASCII 检查；`SC2Layout.xsd` 仅供参考，因为它覆盖不了编辑器的全部输出
- XML 注释/属性仅 ASCII 检查，以及拒绝纯注释的 catalog
- Galaxy 脚本 AST/语法校验（局部变量提升（hoisting）、include 路径、禁用运算符、非 ASCII 字符）

## `inspect-mod-dependencies.py`

从配置的主模组出发，用 `ComponentList.SC2Components` 定位 info 组件（通常是 `DocumentInfo`），解析它的依赖声明，并对每个本地解析到的 `.SC2Mod` 重复该过程。共享依赖只展开一次，环会被标出，缺失的纯本地 `file:Mods` 依赖会让校验失败。引擎与战网依赖仍然可见，但不需要解包的组件文件夹。

## `audit-actor-and-card-integrity.py`

检测命令卡槽位冲突与攻击按钮错位。文件名为了兼容性而保留。在依赖感知的 Actor linter 实现之前，Actor 链校验必须靠 catalog 查询与聚焦的 Actor 复核。

## `audit-gamestrings-anchors.py`

把当前 catalog 的 `Name`/`Tooltip`/`Description` 字符串引用对照 `GameStrings.txt` 检查。编辑器保存后用 `--fill` 从 `ObjectStrings.txt` 恢复缺失的键。

## XSD schema

打包在 `tools/schemas/sc2-xsd/`：

- `Catalog.xsd` — GameData XML schema
- `SC2Layout.xsd` — UI 布局 IDE/参考 schema；`validate-mod.py` 不强制它

VS Code 工作区设置（`.vscode/settings.json`）把 `Base.SC2Data/GameData/*.xml` 绑到 `Catalog.xsd` 以获得实时诊断。
