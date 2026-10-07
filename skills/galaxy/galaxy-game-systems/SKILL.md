---
name: galaxy-game-systems
description: Bank save/load, spawner and wave systems, jungle/camp respawn, resource rewards, hero revive/death, tech tree upgrades, and game attribute lobby options in Galaxy script. Use when implementing persistent data storage, enemy wave spawners, neutral camp respawn timers, kill resource rewards, or player revive logic. Do not use for AI wave behavior (use galaxy-ai-and-techtree) or UI (use galaxy-ui-and-dialogs). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 – 游戏系统

持久化数据、刷兵器、重生计时器、奖励、复活与大厅选项。

## When to Use（何时使用）

- 跨会话持久化玩家数据（bank），或从 UserData 表读设计数值。
- 搭建敌方刷兵 / 路径点波次系统，或中立野怪营地的重生计时器。
- 击杀时发放资源，或让英雄与普通单位死亡后复活。
- 读取大厅游戏属性（game attribute）或任务时间。

先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md)，了解项目级 Galaxy 语法约束（局部变量提升（hoisting）、没有 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、唯一真源规则、模块化脚本边界与命名约定（如 `libMy_` 函数前缀、`libMy_g_` 全局前缀）。本子技能只拥有深层 API 参考；项目约定在路由技能里。

## Loading Boundary（加载边界）

- **本子技能拥有：** bank 存读档、刷兵与波次系统、野怪营地重生、资源奖励、英雄复活/死亡、科技树升级、游戏属性大厅选项
- **父技能（`sc2-galaxy-scripting`）拥有：** 项目约定、schema/语法规则、唯一真源、命名前缀
- **兄弟子技能拥有：** `galaxy-ai-and-techtree`（AI 波次行为）、`galaxy-ui-and-dialogs`（UI）、`galaxy-debug-data-catalog`（Data Table 跨触发器状态）、`galaxy-players-and-alliances`（玩家资源）

## Core Rules（核心规则）

1. `BankLoad` 是异步的——若读取发生在游戏开始时，读任何值之前先调用 `BankWait(bank)`。
2. `BankSetOptionSignature(bank, true)` 要在 `BankLoad` 之后、`BankWait` 之前立刻调用；签署过的 bank 一旦在磁盘上被手工改动，引擎就当作空档/损坏。
3. 不调用 `BankSave(bank)` 什么都不会落盘；一个 bank = 每个玩家槽位一个文件，因此新玩家槽位从空档开始。
4. bank 名应当用地图/模组名——改名会让所有已有存档变成孤儿。
5. 用 `c_orderQueueAddToEnd` 排队路径点命令；`c_orderReplace` 会取消排在它之前的路径点。
6. 只有整组单位都死了营地才重生：先把死亡单位移出组，再在 `UnitGroupCount(group, c_unitCountAlive) > 0` 时直接返回。
7. 环境击杀 / 触发器击杀时 `EventUnitKillingPlayer()` 返回 -1——发奖励前先判断 `lv_killer < 1`。
8. 把 `EventUnit()` 及由它派生的每个值（`UnitGetType`、`UnitGetOwner`、`UnitGetPosition`、`UnitGetFacing`）作为处理函数最先执行的动作捕获，早于任何 `Wait`。
9. `UnitRevive` 只对 `CUnitHero` 单位有效，并保留经验与等级；其他单位一律用 `UnitCreate` 重建。
10. `TechTreeUpgradeAddLevel(player, name, -1)` 就是降级路径——没有单独的移除函数。
11. `TechTreeRestrictionsEnable(player, unit, true)` 是限制（禁止），`false` 是允许；这个布尔值的含义与查询函数名的字面意思相反。
12. `UserDataGet*` 第 4 个参数是从 1 开始的实例索引——在那里传 0 或玩家号会静默返回错误的值。

## References（参考文件）

- [references/bank-save-load.md](references/bank-save-load.md) — bank 的打开/读取/写入/保存、段与键的删除、签名、磁盘文件位置，以及 段→键→值 层级。
- [references/userdata-reward-tables.md](references/userdata-reward-tables.md) — 把奖励与配置值放进 Galaxy Data 表，而不是硬编码在脚本里。
- [references/spawner-wave-system.md](references/spawner-wave-system.md) — 兵线注册、路径点数组、周期性刷兵处理函数，以及刷兵的开关。
- [references/jungle-camp-respawn.md](references/jungle-camp-respawn.md) — 营地注册、共享死亡处理函数，以及「清空营地才重生」规则。
- [references/kill-rewards.md](references/kill-rewards.md) — 按单位类型发放击杀奖励（`PlayerModifyPropertyInt`），以及击杀者不是玩家的判断。
- [references/melee-init-and-tech-tree.md](references/melee-init-and-tech-tree.md) — 近战经济初始化，以及游戏模式用到的科技树升级与限制调用。
- [references/death-and-revive.md](references/death-and-revive.md) — 英雄 `UnitRevive`，以及 `Wait` 前先捕获状态的重生模式。
- [references/game-attributes-and-time.md](references/game-attributes-and-time.md) — 读取大厅选项与任务/真实时间基准。
- [references/sources-and-codebases.md](references/sources-and-codebases.md) — 前置说明段，以及 bank/Custom Values 指南、外部代码库与 wiki。
