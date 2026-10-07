---
name: galaxy-triggers-and-functions
description: Trigger declaration, event registration, async execution via TriggerExecute, the static parameter pattern for functions that use Wait, trigger management, cinematic sequencer queue, and common event types in Galaxy script. Use when creating triggers, attaching events, executing async functions, or building the trigger init chain. Do not use for unit-specific events (use galaxy-units-and-groups) or dialog events (use galaxy-ui-and-dialogs). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 – 触发器与函数

触发器（Trigger）声明、事件注册、异步执行，以及触发器初始化链。

## When to Use（何时使用）

- 创建触发器、挂接事件（`TriggerCreate`、`TriggerAddEvent*`、`TriggerSendEvent`），或搭建地图初始化 / 模块初始化链。
- 让函数异步执行、在独立线程中运行，或向使用 `Wait()` 的函数传参。
- 编写函数，或判断 `static` 让什么变成文件私有。
- 把过场动画纳入触发器队列。

先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md)，了解项目级 Galaxy 语法约束（局部变量提升（hoisting）、没有 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、唯一真源规则、模块化脚本边界与命名约定（如 `libMy_` 函数前缀、`libMy_g_` 全局前缀）。本子技能只拥有深层 API 参考；项目约定在路由技能里。

## Loading Boundary（加载边界）

- **本子技能拥有：** 触发器声明、事件注册、异步 `TriggerExecute`、使用 Wait 的函数的静态参数模式、触发器管理、过场动画队列、常见事件类型
- **父技能（`sc2-galaxy-scripting`）拥有：** 项目约定、schema/语法规则、唯一真源、命名前缀
- **兄弟子技能拥有：** `galaxy-units-and-groups`（单位专属事件）、`galaxy-ui-and-dialogs`（对话框事件）、`galaxy-game-systems`（初始化链 / 刷兵器）

## Core Rules（核心规则）

1. 所有局部变量声明在函数最顶部，位于任何语句或调用之前；写 `var += 1` 而不是 `var++`。
2. `TriggerCreate("Name")` 的字符串必须是准确的函数名，目标函数签名必须是 `bool Name(bool testConds, bool runActions)`。
3. 线程函数第一行就把静态/全局参数寄存器拷贝进局部变量，早于任何 `Wait()`——否则并发调用会覆盖该寄存器。
4. `Wait()` 之后触发器事件上下文可能已被复用；等待前先捕获 `EventUnit()` / `EventPlayer()` 及由它们派生的一切。
5. `TriggerExecute(t, checkConds, runActions)`——第三个参数决定是否另开线程：`(t, false, true)` 立即返回，`(t, false, false)` 阻塞当前线程。
6. 普通函数不能等待；任何需要 `Wait()` 的逻辑都要包进触发器，参数通过文件作用域静态变量传递。
7. Galaxy 线程在 `Wait()` 处按时间片切换，并非真正并行，因此线程化代码比顺序代码更慢——只在确实需要并发时间线时才用。
8. `TriggerAddEventUnit*` 的单位参数传 `null` 表示「任意单位」；传具体单位则事件只对该单位触发。
9. 注册聊天命令要用玩家实际输入的字符串，一条命令一个事件：`TriggerAddEventChatMessage(t, c_playerAny, "!cmd", false)`。
10. `TriggerQueueEnter()` / `TriggerQueueExit()` 必须成对包住整个过场触发器——不成对会卡住排在它后面的所有触发器。
11. 每个模块在各自的 `_Init()` / `_TriggerCreate()` 里注册自己的触发器；`main()` 只接地图初始化触发器。
12. 单行不得超过 2048 字符，且没有 `/* */` 块注释——只能用 `//`。

## References（参考文件）

- [references/functions-and-scope.md](references/functions-and-scope.md) — 编写函数、局部变量置顶、`static` 可见性、native/typedef 声明。
- [references/trigger-declaration-and-init.md](references/trigger-declaration-and-init.md) — 声明触发器全局变量、SC2-IngameDevTools 的字符串名注册模式、编辑器生成的触发器形态。
- [references/trigger-management-functions.md](references/trigger-management-functions.md) — `TriggerCreate` / `TriggerExecute` / `TriggerEnable` / `TriggerDestroy` 等管理型 native 的签名表。
- [references/async-execution-and-multithreading.md](references/async-execution-and-multithreading.md) — 静态参数包装模式、`TriggerExecute` 的线程模型、编辑器生成的线程化 action 定义、线程开销。
- [references/wait-and-timers.md](references/wait-and-timers.md) — `Wait` 的时间基准与 `TimerCreate` / `TimerStart` 用法。
- [references/common-events.md](references/common-events.md) — 时间、单位、玩家/UI 与通用事件的完整注册清单。
- [references/map-init-and-registration-patterns.md](references/map-init-and-registration-patterns.md) — SSF 地图初始化链与库模组的 `InitTriggers` 变体。
- [references/cinematic-action-queue.md](references/cinematic-action-queue.md) — `TriggerQueue` 的进入/退出/暂停/清空与空队列检查。
- [references/sources-and-codebases.md](references/sources-and-codebases.md) — 前置说明段，以及外部代码库、文档与 wiki 链接。
