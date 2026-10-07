---
name: galaxy-players-and-alliances
description: Player data patterns, playergroups, alliance setup, race helpers, player resources, camera control, game attributes, difficulty, player state flags, player color, and game-over calls in Galaxy script. Use when initializing alliances at map start, iterating active players, reading or modifying minerals/gas, checking player race, or ending the game for a player or group. Do not use for unit ownership queries (use galaxy-units-and-groups). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 —— 玩家与同盟

## 何时使用

- 在地图开始时初始化同盟与玩家组，或在任务中途改动它们。
- 遍历活动玩家，或在玩家槽位 ID 与 `playergroup` 之间转换。
- 读写晶体矿、瓦斯、人口或 AI  handicap。
- 查询玩家种族、名字、颜色、槽位状态或槽位类型，以及设置种族。
- 为某个玩家或玩家组结束游戏，或切换玩家状态标志与分数显示。

## 加载边界

- **本子技能负责：** 玩家数据、玩家组、同盟、种族工具函数、玩家资源、镜头控制、游戏属性、难度、玩家状态标志、结束游戏调用
- **父根技能（`sc2-galaxy-scripting`）负责：** 项目约定、schema/语法规则、真源、命名前缀
- **兄弟子技能负责：** `galaxy-units-and-groups`（单位所有权）、`galaxy-game-systems`（bank/复活）、`galaxy-sound-camera-environment`（镜头运动）

## 核心规则

1. 先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md)：它负责语法约束、真源规则和命名前缀。本技能只是 API 参考。
2. 单个玩家槽位是 `int`；多个玩家是 `playergroup`。`PlayerGroupPlayer()` 返回 `int`——把槽位 ID 声明成 `playergroup` 会引发一大片「Parameter type mismatch」错误。
3. 玩家槽位 ID 从 1 开始，而 `PlayerGroupNextPlayer` 的遍历从 `-1` 起步，返回负数时结束。
4. 用 `libNtve_gf_SetAlliance` 配合 `libNtve_ge_AllianceSetting_*` 预设设置同盟；数字预设为 0–9，清单见 `references/alliances.md`——绝不要猜序号。
5. `PlayerSetAlliance` 一次只处理一个通道（`c_allianceIdSharedVision`、`c_allianceIdControl` 等）；只有在预设无法表达该关系时才用它。
6. 用 `libNtve_gf_PlayerIsEnemy(me, target, libNtve_ge_PlayerRelation_*)` 来*查询*关系——关系常量是只读的，不能用它设置关系。
7. `PlayerGetPropertyInt(player, prop)` 读，`PlayerModifyPropertyInt(player, prop, operator, value)` 写——操作符常量（`c_playerPropOperSetTo`、`c_playerPropOperAdd`、`c_playerPropOperSubtract`）是独立参数。
8. `PlayerDifficulty(1)` 返回 1 休闲 / 2 普通 / 3 困难 / 4 残酷——按它分支，不要按大厅属性字符串分支。
9. `GameAttributeGameValue("attr")` 把大厅取值按字符串返回——用字符串相等去比较（例如 `== "Hard"`），队伍成员关系用 `GameAttributePlayersForTeam(n)` 读。
10. `GameOver` 为单个玩家结束游戏，`libNtve_gf_EndGameForPlayerGroup` 为玩家组结束游戏；两者都接收结果常量（`c_gameOverVictory`、`_Defeat`、`_Leave`、`_Tie`）。
11. `PlayerSetState(player, c_playerState*, bool)` 切换单个标志（leader panel、分数、经验获取、待机动作）；用 `PlayerGetState` 读回。文本用色可用 `libNtve_gf_ConvertPlayerColorToColor(PlayerGetColorIndex(p, false))` 转换。
12. 本技能中的镜头调用只有按玩家生效的 `CameraPan`/`CameraSetTarget` 这一对——完整的镜头运动、扫视和快照在 `galaxy-sound-camera-environment`。

## 参考文档

- [references/player-data-and-slots.md](references/player-data-and-slots.md) — `PlayerStruct` 布局，以及槽位 ID 与 playergroup 的类型差异。
- [references/playergroups.md](references/playergroups.md) — 玩家组的构建、查询与遍历。
- [references/player-info-and-race.md](references/player-info-and-race.md) — 名字/种族/颜色/状态读取、种族工具函数、`PlayerSetRace`。
- [references/alliances.md](references/alliances.md) — 同盟预设、通道、关系查询与开图初始化范式。
- [references/player-resources.md](references/player-resources.md) — 晶体矿、瓦斯、人口与 handicap。
- [references/camera-and-game-attributes.md](references/camera-and-game-attributes.md) — 按玩家镜头、大厅属性、难度。
- [references/player-state-color-and-game-over.md](references/player-state-color-and-game-over.md) — 状态标志与结束游戏。
- [references/sources-and-guides.md](references/sources-and-guides.md) — 自己造 API 之前先看的上游代码库与编辑器指南。
