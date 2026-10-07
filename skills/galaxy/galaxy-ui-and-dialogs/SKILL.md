---
name: galaxy-ui-and-dialogs
description: Dialog and dialog control creation, XML frame hookup, hero/upgrade selection panels, scoreboard panels, dialog events, HUD messages, localized text, minimap pings, and the SSF hooked-frame UI pattern in Galaxy script. Use when building or updating any in-game UI such as dialogs, buttons, labels, images, portraits, click handlers, or player-visible messages. Do not use for actor-based visuals (use galaxy-actor-and-visuals). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 —— UI 与对话框

## 何时使用

- 构建或更新任何游戏内 UI：对话框、按钮、文本标签、图片、头像、列表框。
- 挂接已存在于 SC2Layout XML 里的控件，或注册它们的点击处理函数。
- 英雄/升级选择面板、计分板/玩家板，以及 HUD 模式或框架可见性。
- 玩家可见的消息：HUD 消息、错误提示条、本地化文本、提示（tip）。
- 小地图 ping、目标信标、警报类型，以及战役帮助面板。

## 加载边界

- **本子技能负责：** 对话框/dialog control 创建、XML 框架挂接、英雄/升级面板、计分板面板、对话框事件、HUD 消息、本地化文本、小地图 ping，以及 SSF 的挂接框架 UI 范式
- **父根技能（`sc2-galaxy-scripting`）负责：** 项目约定、schema/语法规则、真源、命名前缀
- **兄弟子技能负责：** `galaxy-actor-and-visuals`（actor 视觉）、`galaxy-triggers-and-functions`（通过触发器的对话框事件）、`galaxy-players-and-alliances`（按玩家的 UI）、`sc2data-actors-visuals`（SC2Layout XML）

## 核心规则

1. 先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md)：它负责语法约束、真源规则和命名前缀。本技能只是 API 参考。
2. 优先挂接布局 XML 里已存在的框架：根框架用 `DialogControlHookupStandard(type, path)`，其子控件用 `DialogControlHookup(parent, type, childPath)`。用代码新建 dialog control 只是退路。
3. 挂接路径相对于 UI 布局根，且必须与 SC2Layout XML 完全一致——编译器不校验该字符串，所以要自己验证挂上的控件（`c_invalidDialogControlId` 表示失败）。
4. `c_invalidDialogControlId` 就是 `0`；每次读取/更新都要用 `if (ctrl != c_invalidDialogControlId)` 保护，因为未初始化或挂接失败的控件会一直停在该值。
5. 挂接类调用返回 dialog-control 句柄（SC2-IngameDevTools 范式把它存进 `int`），而 `libNtve_gf_CreateDialogItem*` 系列返回的是取自 `DialogControlLastCreated()` 的 `dialogcontrol`。
6. 触发器处理函数**按函数名字符串**注册（`TriggerCreate("HeroPanel_Click")`），且必须使用 `bool Name(bool, bool)` 签名——字符串与函数名必须完全一致。
7. `TriggerAddEventDialogControl(t, c_playerAny, c_invalidDialogControlId, c_triggerControlEventTypeClick)` 会为该对话框内*任意*控件触发；只想要某个控件时传该控件句柄。
8. 处理函数内部读 `EventDialogControl()` 和 `EventPlayer()`——绝不硬编码控件或玩家。
9. 对话框几何使用内部像素，高度固定 1200 像素，所以把对话框宽度控制在 1600 px 以内（最窄的 4:3 宽度），在任何显示器比例下都不会被裁掉。
10. 控件锚定到 `c_anchor*` 边缘，而不是绝对坐标，这样在各种宽高比下都留在屏幕内。
11. `UISetMode(players, c_uiModeFullscreen, duration)` 隐藏整个默认 HUD；`UISetFrameVisible(players, c_syncFrameType*, false)` 隐藏单个框架。它们是不同的开关，常量也不同。
12. 玩家可见文本来自 `StringExternal("Trig/Key")`（本地化的 `Trig/` 命名空间）；`StringToText` 只用于字面量或运行时拼出的字符串。

## 参考文档

- [references/dialogs-and-frames.md](references/dialogs-and-frames.md) — 挂接 XML 框架、`DialogCreate`、可见性、锚点。
- [references/dialog-controls.md](references/dialog-controls.md) — 挂接函数表、控件创建与更新。
- [references/dialog-events.md](references/dialog-events.md) — 注册与读取对话框点击事件。
- [references/hero-and-scoreboard-panels.md](references/hero-and-scoreboard-panels.md) — 英雄/升级面板、玩家板、UI 模式。
- [references/messages-and-localized-text.md](references/messages-and-localized-text.md) — HUD 消息、错误提示条、本地化文本。
- [references/alerts-and-minimap-pings.md](references/alerts-and-minimap-pings.md) — 警报类型、小地图 ping 与目标信标。
- [references/help-panel.md](references/help-panel.md) — 战役帮助面板与教程条目。
- [references/dialog-layout-and-resolution.md](references/dialog-layout-and-resolution.md) — 内部分辨率、安全宽度、模板创建。
- [references/sources-and-guides.md](references/sources-and-guides.md) — 自己造 API 之前先看的上游代码库与编辑器指南。
