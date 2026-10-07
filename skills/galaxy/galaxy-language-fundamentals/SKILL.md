---
name: galaxy-language-fundamentals
description: Core Galaxy language syntax, all primitive and engine handle types (including funcref/structref/arrayref), naming conventions, include/file structure, structs, arrays, control flow, and map initialization patterns. Use for any question about Galaxy syntax, type system, variable declarations, or the MapScript bootstrap chain. Also documents which files are auto-generated (MapScript.galaxy, LibHASH.galaxy) and must never be edited. Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 —— 语言基础

Galaxy 是《星际争霸 II》地图/模组开发所用的脚本语言。它是静态类型、类 C 的语言，由 SC2 编辑器编译。

## 何时使用

- 任何关于 Galaxy 语法、类型系统、变量声明或 MapScript 引导链的问题。
- 编写或审查必须能编译的 `.galaxy` 代码：声明、结构体、数组、常量、控制流。
- 为新函数、结构体、全局或局部变量挑选命名规范。
- 确认某个文件是否自动生成、绝不能编辑（见规则 11）。

## 加载边界

- **本子技能拥有：** Galaxy 语言语法、基元/句柄类型、命名规范、include/文件结构、结构体、数组、控制流，以及 MapScript 引导链
- **父根技能（`sc2-galaxy-scripting`）拥有：** 项目约定、schema/语法规则、真源（source-of-truth）、命名前缀
- **兄弟子技能拥有：** `galaxy-code-organization`（文件布局/include 顺序）、`galaxy-math-strings-conversion`（数学/字符串 API）、`galaxy-triggers-and-functions`（触发器（Trigger）/事件声明）

## 核心规则

1. **前置条件：** 先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md) —— 项目级语法约束（局部变量提升（hoisting）、无 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、真源规则、模块脚本边界与命名规范（`libMy_` 函数前缀、`libMy_g_` 全局前缀）都在那里；本子技能只拥有语言深度参考。
2. 所有局部变量必须在函数最顶部声明，位于任何语句、条件或调用之前。
3. 没有 `++`/`--`；用 `var += 1` 和 `var -= 1`。用 `while`（父技能约定，`while (true)` + `break` 做迭代）；长格式 `for` 循环是 GUI 编译器产出的东西。
4. 只有 `//` 行注释；`/* */` 块注释不是合法 Galaxy。
5. `static` 表示文件私有：在自己的 `.galaxy` 文件之外不可见。
6. 行长度不得超过 2048 个字符（编译器硬限制）。
7. `TriggerCreate` 接收函数的**字符串名**；处理函数（handler）签名必须是 `bool Name(bool a, bool b)`。
8. `fixed` 是 20 位定点数，不是 IEEE 浮点 —— 精度约 0.0001，最大值约 524287。它就是 SC2 所称的 "Real"。
9. 没有动态内存：没有 `new`/`delete`。存储只有静态全局、栈上局部，或引擎管理的句柄。
10. `include` 路径相对于地图根目录，且不带 `.galaxy` 扩展名。
11. 绝不要把逻辑写进 `MapScript.galaxy`、`LibHASH.galaxy` 或 `LibHASH_h.galaxy` —— 编辑器保存时会覆盖它们。
12. 文件一旦共享 `MapScript.galaxy` 的编译单元，任一文件都能调用另一个，无需局部 `include`。

## 参考文档

- [references/file-structure-and-includes.md](references/file-structure-and-includes.md) —— 当把 `MapScript.galaxy` → `scripts/main.galaxy` 接起来时，或挑选战役 / 库模组 / 测试地图的 include 模式时
- [references/naming-conventions.md](references/naming-conventions.md) —— 当命名新函数、结构体、全局常量或局部变量时，或核对处理模块模式时
- [references/type-system.md](references/type-system.md) —— 当需要完整基元/句柄类型清单、`funcref`/`structref`/`arrayref` 语义，或某个内置 `c_` 常量时
- [references/structs-and-arrays.md](references/structs-and-arrays.md) —— 当声明结构体、定长数组，或按引用传递它们时
- [references/control-flow-and-gotchas.md](references/control-flow-and-gotchas.md) —— 写循环/分支之前，或看似正确的代码编译失败时
- [references/map-initialization.md](references/map-initialization.md) —— 当搭建 `main()`、地图初始化触发器，或幂等的库 `InitLib` 时
- [references/string-and-color-quick-reference.md](references/string-and-color-quick-reference.md) —— 当你只需要常用字符串/颜色调用时；完整 API 在 `galaxy-math-strings-conversion`
- [references/external-sources.md](references/external-sources.md) —— 当需要查 native 签名、wiki 上的类型清单，或上游指南时
- [references/galaxy-language.md](references/galaxy-language.md) —— 上游 GalaxyScript 使用指南：脚本与 GUI 的取舍规则、语法对比表、API 分层、catalog 反射、完整示例
