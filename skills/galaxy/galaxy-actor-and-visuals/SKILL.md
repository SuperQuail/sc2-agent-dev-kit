---
name: galaxy-actor-and-visuals
description: Actors, visual effects, model attachments, animations, and tint/scale/opacity control in Galaxy script. Use when working with ActorSend, AttachModelToUnit, PlayAnimation, SetTintColor, SetScale, SetOpacity, doodad visibility, or any purely visual/audio actor operation. Do not use for unit gameplay logic or sound triggers (use their dedicated skills). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 —— Actor 与视觉效果

## 何时使用

- 生成效果、模型、光束或挂接物，或销毁它们。
- 把模型或数据定义的 actor 挂接到单位或另一个 actor 上。
- 播放、清除或计时动画，包括区域内 doodad 上的动画。
- 给 actor 上色、缩放、隐藏、换贴图、换模型或淡入淡出。
- doodad 的显示/移除、整区域 actor 消息、头像（portrait），以及 actor scope/ref 查找。

## 加载边界

- **本子技能负责：** Galaxy 脚本中的 actor、视觉效果、模型挂接、动画、上色/缩放/不透明度控制，以及 doodad 可见性
- **父根技能（`sc2-galaxy-scripting`）负责：** 项目约定、schema/语法规则、真源、命名前缀
- **兄弟子技能负责：** `galaxy-sound-camera-environment`（声音/镜头）、`galaxy-ui-and-dialogs`（UI）、`sc2data-actors-visuals`（CActor* XML schema）

## 核心规则

1. 先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md)：它负责语法约束、真源规则和命名前缀。本技能只是 API 参考。
2. actor 与模拟层（单位、区域）相互独立，只通过消息通信：`actor`/`actorscope` 句柄给你的是画面和声音，不是游戏逻辑状态。
3. 所有 `libNtve_gf_PlayAnimation*` 和 `libNtve_gf_ClearAnimation` 都接收 **actor**，不是单位——先用 `libNtve_gf_MainActorofUnit(lv_unit)` 取到它。
4. 挂接/创建类辅助函数不可互换：`AttachModelToUnit`、`AttachModelToUnitInheritVisibility`、`AttachActorToUnit`、`AttachActorToActor`、`AttachModelToActor2`、`CreateModelAtPoint` 各自参数表都不同——调用前先查 `references/actor-basics-and-creation.md`。
5. `ActorScopeCreate("")` 只接收一个可选的 actor 名（不是两个）；`ActorScopeKill(scope)` 销毁该 scope 内的全部 actor。
6. 库构造函数与裸消息字符串等价：`ActorSend(a, libNtve_gf_SetTintColor(1.0,0.0,0.0,1.0))` 与 `ActorSend(a, "SetTintColor {1,0,0,1}")` 相同。
7. `libNtve_gf_SendActorMessageToUnit(lv_unit, msg)` 不需要 actor 句柄——它直接作用于单位自己的 actor，是发单位消息最短的路径。
8. 想要单位可见的那个 actor 时，优先用 `libNtve_gf_MainActorofUnit(lv_unit)` 而不是 `ActorFrom(lv_unit)`。
9. 动画时长查询是异步的：`AnimLengthQueryByName(actor, anim, scaled)` 没有返回值，要紧接着调用 `AnimLengthQueryWait()`（无参数），再读 `AnimLengthQueryLastCreated()`。
10. `libNtve_gf_ShowHideDoodadsInRegion(showHide, region, doodadType)` **没有** playergroup 参数——它全局改变地图上的 doodad，`doodadType` 传空串表示全部 doodad。
11. `libNtve_gf_CinematicMode(onOff, playergroup, duration)` 和 `CinematicFade(...)` 是两个过场入口；参数顺序不是直觉顺序，照 `references/cinematic-fade-and-mode.md` 抄。
12. 具名 ref 用 `::` 前缀（`"::Main"`、`"::Target"`）；用 `ActorRefGet`/`ActorScopeRefGet` 读，用配套的 `ActorRefSet`/`ActorScopeRefSet` 写。

## 参考文档

- [references/actor-basics-and-creation.md](references/actor-basics-and-creation.md) — 创建 actor、挂接模型/actor、scope 与句柄。
- [references/actor-messages.md](references/actor-messages.md) — `ActorSend`、消息构造函数与常用消息表。
- [references/animations.md](references/animations.md) — 播放/清除/区间动画、动画属性与 doodad 动画。
- [references/actor-texture-and-model.md](references/actor-texture-and-model.md) — 贴图组、换模型、销毁模型、模型朝向。
- [references/cinematic-fade-and-mode.md](references/cinematic-fade-and-mode.md) — `CinematicFade` 与 `CinematicMode` 的用法和常量。
- [references/doodads-and-region-messages.md](references/doodads-and-region-messages.md) — doodad 显示/隐藏/移除与整区域 actor 消息。
- [references/portraits-and-actor-refs.md](references/portraits-and-actor-refs.md) — 头像句柄与 actor scope/ref 访问。
- [references/sources-and-guides.md](references/sources-and-guides.md) — 上游代码库、NativeLib actor 辅助函数与 wiki 页面。
