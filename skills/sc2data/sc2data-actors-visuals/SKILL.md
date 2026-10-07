---
name: sc2data-actors-visuals
description: SC2 Data Editor — Actors, visual effects, animations, sounds, and the actor event system in XML. Use when creating or modifying CActorUnit, CActorModel, CActorAction, CActorSound, CActorBeam, CActorSite, or any <On Terms="..." Send="..."/> event wiring. Covers actor creation, parent types, aliases, macros, death arrays, sound arrays, and all common actor messages. Always consult the catalogsData.xsd schema for exact fields and structure — do not assume unsupported fields exist. Do not use for game logic/damage (use sc2data-effects-weapons) or unit/ability data (use sc2data-units-abilities). Actors are client-side only — they cannot affect gameplay. Start by running "python tools/sc2.py start" to read this project's previous task records.
---

## 何时使用

处理 `CActorUnit`、`CActorModel`、`CActorAction`、`CActorSound`、`CActorBeam`、`CActorSite`、其他任意 `CActor*` 条目，以及任意 `<On Terms="..." Send="..."/>` 事件接线时加载本技能——包括 Actor 创建、父类型、别名、宏、死亡与声音数组，以及常用 Actor 消息。

Actor 是客户端侧的视觉与音频层：它们在每个客户端本地运行，不做网络同步，也无法影响游戏性状态。

## 加载边界

- **本子技能拥有：** CActor*（Unit、Model、Action、Sound、Beam、Site）、Actor 事件系统、父类型、别名、宏、死亡/声音数组，以及 Actor 消息
- **父根技能（`sc2-catalog-xml`）拥有：** 项目约定、schema/语法规则、唯一真源、命名前缀
- **兄弟子技能拥有：** `sc2data-units-abilities`（CUnit/CAbil XML）、`galaxy-actor-and-visuals`（Galaxy Actor API）、`sc2data-effects-weapons`（Actor 所表现的那些 CEffect/CWeapon）

## 核心规则

1. 先加载 [`sc2-catalog-xml`](../../sc2-catalog-xml/SKILL.md) 取得项目级 Catalog XML schema 规则；本子技能只负责 Actor 的深度 schema。
2. 每次改动都用 Red Hat XML 对照 `catalogsData.xsd` 校验，绝不臆造字段：给每条诊断归类（非法元素、属性、枚举或字段路径/索引），到 schema 里核对允许的结构，只修成 schema 支持的字段，然后重新校验。被要求修 XML 报错时，把这条闭环跑完。
3. Actor 是视觉与音频层：它们响应游戏事件、播放动画、生成特效模型、播放声音，但在每个客户端本地运行、不做网络同步，对游戏性状态零影响——绝不要用它追踪游戏状态。
4. Actor 事件接线是反向且自声明的：只有 Actor 自己的 `<On Terms="..." Send="..."/>` 条目能激活它。Terms 之间用 `;` 表示 AND，用 `!` 取反，`*` 是匹配单个片段的通配符。
5. 为新单位创建的 Actor 放进该单位自己的独立文件（`GameData/<Faction>/<Unit>/<Unit>.xml`）并在 `GameData.xml` 注册，不要塞进单体式的 `ActorData.xml`。
6. id 与单位 id 保持一致：单位 `MyUnit` -> Actor `MyUnitActor`。
7. 只要该种族或类型存在匹配的宏，就优先用暴雪的可复用宏（`<Macros value="..."/>`），而不是手写 `<On>` 事件。
8. `CActorMissile` 完全没有 `<On>` 数组——它是纯视觉弹体；发射/命中事件接线放在 `CActorAction` 或其他 Actor 上。
9. 按生命周期选父类型：爆炸与出生特效用 `ModelAnimationStyleOneShot`，持续循环的区域用 `ModelAnimationStyleContinuous`，一次性音效用 `SoundOneShot`。
10. 必须自我清理的非单位 Actor 要加 `<On Terms="ActorOrphan" Send="Destroy"/>`；事件必须每个 Actor 生命周期只触发一次时用 `Cap 1`。
11. `Upgrade.<id>.Add` 只有在单位被列入该升级的 `Affected Unit Array` 时才会触发；充能与冷却 Actor 事件需要技能打开 `Register Charge Event` / `Register Cooldown Event` 共享标志。
12. 使用 `Range *` 或 `Cursor Splat` 父类型时，编辑器会在创建时自动生成默认 `<On` 事件——在 Events 字段上 Reset To Parent Value，或在 XML 视图里删除它们，以免接线重复。

## References

- [references/actor-unit-and-action-examples.md](references/actor-unit-and-action-examples.md) — Actor 文件、带完整注释的 CActorUnit、CActorAction，以及 Actor 父类型表。
- [references/actor-types-and-examples.md](references/actor-types-and-examples.md) — CActor* 类型清单，外加模型、声音、光束、飞弹示例。
- [references/actor-events-and-messages.md](references/actor-events-and-messages.md) — `<On Terms Send>` 语法、term 表，以及常用 Actor 消息。
- [references/actor-aliases-and-macros.md](references/actor-aliases-and-macros.md) — 引用别名（`::Creator`、`::Target`、`_UnitMedium`）与宏的用法。
- [references/advanced-actor-event-patterns.md](references/advanced-actor-event-patterns.md) — 计时器、信号、状态标志、次数上限、效果树隔离、升级与充能事件。
- [references/external-references.md](references/external-references.md) — Actor wiki 页面、教程与 catalogsData.xsd schema 地址。
