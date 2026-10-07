---
name: galaxy-sound-camera-environment
description: Sound playback, music, camera movement and snapshots, cinematic mode, weather, lighting, and environment effects in Galaxy script. Use when playing sounds or music, moving or configuring the camera, entering cinematic mode, or controlling weather and lighting. Do not use for actor-level sound messages (use galaxy-actor-and-visuals). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 —— 声音、镜头与环境

## 何时使用

- 播放、停止或等待声音与音乐，或静音/压低某个声音通道。
- 移动、吸附、保存、恢复或锁定玩家镜头，或让镜头扫过地图。
- 进入过场模式，运行脚本化过场或叙事序列。
- 雾、地形高度、环境可见性、禁飞区、天空盒与昼夜时间。
- 排行榜、目标、头像传送（transmission）、简报录像与任务胜利。

## 加载边界

- **本子技能负责：** 声音播放、音乐、镜头运动/快照、过场模式、天气、光照与环境效果
- **父根技能（`sc2-galaxy-scripting`）负责：** 项目约定、schema/语法规则、真源、命名前缀
- **兄弟子技能负责：** `galaxy-actor-and-visuals`（actor 级声音消息）、`galaxy-ui-and-dialogs`（HUD 消息）、`galaxy-players-and-alliances`（玩家镜头）

## 核心规则

1. 先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md)：它负责语法约束、真源规则和命名前缀。本技能只是 API 参考。
2. 声音播放有三条轴，参数表各不相同：`SoundPlayForPlayer(link, player)`、`SoundPlayAtPointForPlayer(link, player, point, minDist, maxDist, volume)`、`SoundPlayAtPoint(link, playergroup, point, minDist, volume, fadeIn)`——照签名抄，绝不猜。
3. `SoundPlayOnUnit` 接收的是 **playergroup**，不是单个玩家，且参数顺序与 `SoundPlayAtPointForPlayer` 不同。
4. `PlayerGroupAll()` 是战役/多人地图上安全的接收者；传单个玩家只会让一个玩家听到。
5. `SoundWait(sound)` 会阻塞触发线程——在过场线程里要刻意使用，绝不要用在同时驱动游戏逻辑的循环中。
6. 镜头调用接收单个 `player`，且 `CameraPan` / `CameraSetTarget` 共用同一个六参数形状 `(player, point, distance, yaw, pitch, sync)`。
7. `CameraSave(player)` / `CameraRestore(player, duration)` 用于在脚本化镜头运动后把视角放回去——用它，不要手工重建 yaw/distance。
8. `CameraShake` 没有 NativeLib 包装；只有当你的构建暴露了该原生函数时才直接调用（见 `references/camera.md`）。
9. `libNtve_gf_CinematicMode(onOff, playergroup, duration)` 是黑边/UI 开关；`UISetMode(players, c_uiModeFullscreen, 0.0)` 是另一次完整的 HUD 移除——真正的过场两个都要，并且都要有配对的退出调用。
10. 每个 `TriggerSkippableBegin` 必须由恰好一个 `TriggerSkippableEnd()` 关闭，且每条退出路径都要恢复 HUD/过场状态。
11. 战役传送管线有顺序依赖：先用 `libCamp_gf_SetAllSoundChannelVolumesCampaign` 压低音量，在**每次** `libCamp_gf_SendTransmissionCampaign(...)` **之前**调用 `libLbty_gf_PlayTransmissionCueSound(PlayerGroupAll())`，最后用 `_Game` 音量模式恢复。
12. `LeaderboardCreate` / `ObjectiveCreate` 只能通过 `LeaderboardLastCreated()` / `ObjectiveLastCreated()` 拿到句柄，而且两者都必须用 `BoardDestroy` / `ObjectiveDestroy` 释放。

## 参考文档

- [references/sound-playback-and-channels.md](references/sound-playback-and-channels.md) — 播放声音、通道静音/音量、音乐。
- [references/sound-advanced-and-soundtrack.md](references/sound-advanced-and-soundtrack.md) — 挂到单位上的声音、等待、声音长度、配乐轨。
- [references/camera.md](references/camera.md) — 平移/吸附/保存/恢复、掠过（swoosh）、camera object、输入锁定。
- [references/cinematic-mode-and-sequences.md](references/cinematic-mode-and-sequences.md) — 过场管线与任务触发队列。
- [references/environment-fog-terrain-and-skybox.md](references/environment-fog-terrain-and-skybox.md) — 雾、地形、环境可见性、天空盒、昼夜时间。
- [references/leaderboard-and-objectives.md](references/leaderboard-and-objectives.md) — 计分板与目标面板 API。
- [references/transmissions.md](references/transmissions.md) — `TransmissionSend` 与战役传送范式。
- [references/video-recording-and-mission-completion.md](references/video-recording-and-mission-completion.md) — 简报录制与任务胜利流程。
- [references/sources-and-guides.md](references/sources-and-guides.md) — 自己造 API 之前先看的上游代码库与编辑器指南。
