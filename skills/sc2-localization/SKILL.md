---
name: sc2-localization
description: StarCraft II localization files (GameStrings.txt, ObjectStrings.txt, TriggerStrings.txt) for the AeonOfIhanrii campaign. Use when adding or fixing player-facing text, Data Editor display names, trigger display names, or string anchors. Covers file formats, key conventions, XML anchor rules, and automated restoration via audit-gamestrings-anchors.py.
---
# SC2 本地化与字符串锚点

## 何时使用

任务触及 `<ModName>.SC2Mod/<locale>.SC2Data/LocalizedData/` 下的 `GameStrings.txt`（游戏内可见文本）、`ObjectStrings.txt`（数据编辑器显示文本）或 `TriggerStrings.txt`（触发器编辑器显示名）时加载本技能。

数据编辑器显示空白名、错误前缀或描述缺失时也加载，以及新增可读的 catalog 对象、修已上报的 UI 文本问题时。

## 加载边界

- **本技能拥有：** 三类（加一类）本地化文件的格式与键约定、面向玩家文本的 XML 锚点规则、`ObjectStrings` 覆盖率、锚点恢复流程、语言区域纪律与 KSP CLI 诊断。
- **父技能拥有：** `sc2-project-entry` 拥有项目身份、语言区域选择与路由；`sc2-catalog-xml` 拥有产生这些锚点的 XML 编写规则。
- **兄弟技能拥有：** `sc2-map-triggers` 拥有触发器工作里的 `TriggerStrings.txt` 转义；`sc2-tools-validation` 拥有 `audit-gamestrings-anchors.py` 的调用与作用域；`sc2-editor-handoff` 拥有触发恢复的保存闸门。

## 核心规则

1. **面向玩家的文本放 `GameStrings.txt`，编辑器显示文本放 `ObjectStrings.txt`**——不要写进原始数据 XML。数据编辑器文本空白或错误，通常是缺字符串键，而不是字段坏了。
2. **每个新 catalog `id` 都要在同一次改动里加上 `ObjectStrings.txt` 的 `Type/Name/ID` 与 `Type/EditorPrefix/ID`**，否则对象在数据编辑器里显示为空白。
3. **玩家可见的 catalog 字段需要显式 XML 锚点**（`<Description value="Unit/Tooltip/MyUnit"/>`）——没有它，编辑器保存时可能把文本剪掉。
4. **改任何字符串文件前，先把锚点对准确切的 UI 面**（世界悬停、选择面板、命令卡、提示框、编辑器文本）。
5. **每次触及 catalog XML 的编辑器保存之后都跑 `python tools/audit-gamestrings-anchors.py --fill`**，然后审查 diff——它会重写文件。
6. **先确认运行时语言区域**——查看模组既有的语言区域文件夹并沿用它；不要按用户所说的语言去推断。
7. **零错误的静态锚点报告不等于打包后运行时覆盖**——间接的运行时引用必须单独复核。
8. **字段消费方式各不相同：** 对照当前依赖确认某个 `Name`/`Tooltip`/`Description` 字段实际读的是哪个 `GameStrings` 键。
9. **`TriggerStrings.txt` 是纯文本**——绝不做 HTML 转义；`Triggers` XML 本身走常规 XML 转义。
10. **全新（非翻译）文本先要模板**；没有模板就把结果标为尽力草稿，并保持中性、简短。

## 参考文件

- [references/localization-files-and-anchors.md](references/localization-files-and-anchors.md) — 文件/键格式表、覆盖的类型、必需的 `ObjectStrings` 条目与 XML 锚点示例。
- [references/intake-and-anchor-restoration.md](references/intake-and-anchor-restoration.md) — UI 面受理规则、`--fill` 恢复与本地化硬规则。
- [references/locale-and-coverage-caveats.md](references/locale-and-coverage-caveats.md) — 语言区域纪律、`--locale`/`--mod-dir` 用法与 `ObjectStrings` 迁移陷阱。
- [references/hotkeys-and-ksp-cli.md](references/hotkeys-and-ksp-cli.md) — `GameHotkeys.txt`、KSP `sc2loc` 诊断、类别、退出码与检查/修复循环。
- [references/writing-guidance.md](references/writing-guidance.md) — 翻译与新写文本的规则、模板与术语一致性。
- [references/localization.md](references/localization.md) — 原始本地化文件全量参考。
- `skills/sc2-catalog-xml/references/xml-patterns.md` — 会产生这些锚点引用的 XML 模式。
