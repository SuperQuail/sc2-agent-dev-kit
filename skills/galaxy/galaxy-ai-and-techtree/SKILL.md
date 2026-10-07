---
name: galaxy-ai-and-techtree
description: AI behavior, melee AI initialization, tech tree upgrades, wave difficulty scaling, AI waves, unit restrictions, and tactical AI helpers in Galaxy script. Use when setting up computer-controlled players, scaling enemy difficulty per wave, managing tech tree restrictions, or scripting AI attack waves. Do not use for player-controlled unit behavior (use galaxy-units-and-groups). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 – AI 与科技树

电脑玩家设置、AI 建造/波次控制、科技树状态，以及按波次缩放难度。

## When to Use（何时使用）

- 设置电脑玩家：近战经济、起始单位、启动 AI 性格（personality）。
- 驱动 AI 的建造/训练/研究队列、库存量（stock）、城镇与目标评估。
- 指挥 AI 攻击波，或逐波缩放敌方难度。
- 读取或修改科技树升级、单位计数、限制与生产上限。
- 战役空投舱与剧情状态（story state）的跨关携带。

先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md)，了解项目级 Galaxy 语法约束（局部变量提升（hoisting）、没有 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、唯一真源规则、模块化脚本边界与命名约定（如 `libMy_` 函数前缀、`libMy_g_` 全局前缀）。本子技能只拥有深层 API 参考；项目约定在路由技能里。

## Loading Boundary（加载边界）

- **本子技能拥有：** AI 行为、近战 AI 初始化、科技树升级、波次难度缩放、AI 波次、单位限制、战术 AI 辅助函数
- **父技能（`sc2-galaxy-scripting`）拥有：** 项目约定、schema/语法规则、唯一真源、命名前缀
- **兄弟子技能拥有：** `galaxy-units-and-groups`（玩家操控的单位行为）、`galaxy-game-systems`（刷兵/波次系统）、`galaxy-players-and-alliances`（玩家种族/科技）

## Core Rules（核心规则）

1. `MeleeInitAI()` 一次性设置所有电脑玩家；只有单个玩家需要单独设置时才用 `MeleeInitUnitsForPlayer` + `MeleeInitResourcesForPlayer`。
2. `AIStart(player, "AI\Terran.SC2AIData", ...)` 才启动 AI 性格——只做近战初始化只给单位和资源，不会有 AI 行为。
3. `AISetStock` / `AISetStockEx` 维护某种单位的常备数量，它们不下达单次建造命令——单次命令用 `AIBuild` / `AITrain`。
4. `AIAttackWaveUseGroup` 会消耗一个预先编好的单位组并绕过波次构建器，所以要在调用前先把组编好。
5. `wavetarget` 与 `point` 是不同类型：`AIWaveTargetGatherMelee` 返回 `wavetarget`，手上只有 `point` 时要用 `AIAttackWaveSetTargetPoint`。
6. 波次升级要在每次波次切换时施放，而不是任务开始时施放一次——用计数器记录波次，递增时加一级。
7. `TechTreeUpgradeAddLevel(player, name, -1)` 可以移除一级；没有单独的降级或移除函数。
8. `TechTreeRestrictionsEnable(player, entry, true)` 是限制（禁止），`false` 是允许——这个布尔值与查询函数 `TechTreeRestrictionsEnabled` 的返回值含义相反。
9. `TechTreeRequirementsEnable(player, false)` 会关掉该玩家**全部**前置条件检查；记得恢复，否则该玩家会无视整棵科技树。
10. `c_techCount*` 常量不能混用——`Any` / `Both` / `Made` / `Lost` 要按题意挑，因为「累计总数」和「当前数量」回答的是不同问题。
11. `libNtve_gf_*` 辅助函数只有在依赖链里有 NativeLib 时才存在；依赖之前先在 `TriggerLibs/NativeLib.galaxy` 里确认该函数。
12. `libNtve_gf_DifficultyValue*` 的参数从左到右依次是 简单 / 普通 / 高级 / 专家，所以第一个参数是最弱的档位，不是最强的。

## References（参考文件）

- [references/melee-ai-initialization.md](references/melee-ai-initialization.md) — `MeleeInitUnitsForPlayer`、`MeleeInitResourcesForPlayer`、`AIStart` 与 `MeleeInitAI`。
- [references/ai-build-train-research.md](references/ai-build-train-research.md) — 脚本级的建造/训练/研究命令、库存控制，以及清空队列。
- [references/ai-towns-and-evaluation.md](references/ai-towns-and-evaluation.md) — 城镇资源点、采集/防守位置、默认经济/扩张，以及强度比较。
- [references/ai-waves-and-attack.md](references/ai-waves-and-attack.md) — `AIWaveGet` / 波次单位 / 波次目标 / 启停开关，以及脚本化的 `AIAttackWave*` 调用。
- [references/wave-upgrade-pattern.md](references/wave-upgrade-pattern.md) — 逐波升级辅助函数与计数器驱动的缩放模式。
- [references/tech-tree-upgrades.md](references/tech-tree-upgrades.md) — 升级等级的增/设/读，以及升级变化事件的取值函数。
- [references/tech-tree-counts-restrictions-production.md](references/tech-tree-counts-restrictions-production.md) — `TechTreeUnitCount` 常量、限制与前置条件开关，以及生产上限。
- [references/campaign-drop-pods-and-story-state.md](references/campaign-drop-pods-and-story-state.md) — `libCamp_gf_CreateDropPod`、战役科技单位，以及剧情状态的读/写。
- [references/native-lib-ai-helpers.md](references/native-lib-ai-helpers.md) — NativeLib 的战术 AI 设置函数与难度取值辅助函数。
- [references/sources-and-codebases.md](references/sources-and-codebases.md) — 前置说明段，以及外部代码库、文档与 wiki 链接。
