---
name: sc2-galaxy-scripting
description: StarCraft II Galaxy scripting rules and gotchas for the AeonOfIhanrii campaign. Use when writing, editing, or debugging .galaxy files, trigger custom scripts, bank logic, or UI/dialog code. Covers syntax constraints, source-of-truth, modular script boundaries, and deployment.
---
# SC2 Galaxy 脚本

## 何时使用

任务涉及 Galaxy 脚本（`.galaxy` 文件）时加载本技能：写触发器动作、Bank 持久化、对话框/UI 代码、单位事件，或任何自定义脚本块。调试「找不到函数」、解析错误或链接器丢库时也加载。

不要仅因为 SC2 会把 GUI 触发器编译成 Galaxy，就给新触发器选独立 Galaxy。GUI 优先的编写走 `sc2-map-triggers`；本技能用于明确要求 Galaxy 的实现、既有脚本维护，或必要的内嵌 Custom Script 块。

本技能是 **Galaxy 路由**。它掌管项目级语法约束、真源规则、模块化脚本边界与部署。深层 Galaxy API 工作路由到 `skills/galaxy/` 下对应的 `galaxy-*` 子技能。

## 加载边界

- **本技能拥有：** 项目级 Galaxy 语法约束、脚本真源与部署规则、模块化脚本边界规则、GUI include 模式与命名约定。
- **父技能拥有：** `sc2-project-entry` 拥有项目身份与路由；`AGENTS.md` 拥有仓库红线（绝不手改 `Lib*.galaxy` 或 `MapScript.galaxy`）。
- **兄弟技能拥有：** `galaxy-*` 子技能拥有按 API 的参考；`sc2-map-triggers` 拥有触发器 XML 与 GUI 优先规则；`sc2-bank-system` 拥有 Bank schema 与持久化；`sc2-tools-validation` 拥有工具 CLI；`sc2-editor-handoff` 拥有编辑器保存闸门。

## 核心规则

1. **只编辑手写脚本**（`Epi_Main.galaxy` 或 `Base.SC2Data/Scripts/`）——绝不碰 `Lib*.galaxy` 或 `MapScript.galaxy`；编辑器保存时会重新生成这两者。
2. **局部变量提升（hoisting）：局部变量在函数体顶部声明且不带内联初始化**——在 `if`/`while` 内声明或带初始化器都是解析错误。
3. **数组写法是 `string[90] arr;`**，循环只有 `while`，自增用 `count += 1;`——`count++`、`--` 和 `for` 都不支持。
4. **全局初始化器只能是整数/定点字面量**——像 `c_invalidDialog` 这样的常量初始化器编译不过。
5. **用之前先确认常量存在**——例如是 `c_unitPropLifeMax` 而不是 `c_unitPropMax`；触发单位取自 `EventUnit()`，不存在 `UnitFromEvent()`。
6. **`include` 用不带扩展名的相对路径**（`include "Scripts/libMy_Globals"`）。
7. **绝不把函数体拆到多个文件**，也绝不重复定义同名函数；按编译顺序，被调用者必须出现在调用者之前。
8. **地图只通过 GUI action definition 链接模组库**——地图里的 Custom Script 不算数。
9. **不要猜 native、参数或常量**——查 `skills/sc2-map-triggers/references/triggers-native/`，并对照 `natives.galaxy` 或编辑器生成的输出交叉验证。
10. **只在 `workspace_copy` 模式下部署**（`python tools/deploy-mod.py`），然后让编辑器在保存时重新生成编译包装。

## 参考文件

- [references/critical-syntax-rules.md](references/critical-syntax-rules.md) — 已确认的编译错误规则与正/误 Galaxy 片段；写任何 `.galaxy` 行之前读它。
- [references/modular-script-boundaries.md](references/modular-script-boundaries.md) — include 文件形态、重复函数失败与前向引用。
- [references/gui-include-and-map-calls.md](references/gui-include-and-map-calls.md) — 常驻的 `Epi_Include` 注入模式与地图调用模组的 GUI action 规则。
- [references/error-triage.md](references/error-triage.md) — 社区整理的编译/链接失败「错误→原因→修法」表。
- [references/naming-and-source-of-truth.md](references/naming-and-source-of-truth.md) — 哪些文件可编辑、部署模式、前缀与操作者注意事项。
- [references/galaxy-sub-skill-router.md](references/galaxy-sub-skill-router.md) — `galaxy-*` 子技能表与 native 查询纪律；做深层 API 工作前读它。
- [references/galaxy-gotchas.md](references/galaxy-gotchas.md) — 原始陷阱源；`skills/sc2-bank-system/references/galaxy-bank.md` — 脚本中使用的 Bank 签名。
- `skills/galaxy/galaxy-language-fundamentals/references/galaxy-language.md` — 深层语法入门。
