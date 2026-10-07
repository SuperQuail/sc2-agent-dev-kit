---
name: sc2-actor-system
description: StarCraft II actor system (events, terms, messages, parents) for the AeonOfIhanrii campaign. Use when creating or modifying CActorUnit/CActorAction/CActorModel/CActorBeam/CActorSound actors, troubleshooting VFX/audio/visibility issues, or extracting actors from inactive XML. Covers actor event-driven model, parent actor selection, missile defaults, texture swaps, indexed construction events, and editor round-trip pitfalls.
---
# SC2 Actor 系统

## 何时使用

任务涉及 `ActorData.xml` 中的 Actor XML（或任何 `CActor*` catalog 对象）时加载本技能：新建或修改单位 Actor（`CActorUnit`）、动作/攻击 Actor（`CActorAction`）、模型 Actor（`CActorModel`）、光束、音效、射程、溅射，或变形过渡视觉。调试单位不可见、VFX/音效缺失、贴图切换失效，或「同一单位作用域内存在多个 CActorUnit」告警时也加载，以及从非活动依赖 XML 中提取 Actor 时。

## 加载边界

- **本技能拥有：** Actor 事件模型、Actor 字段与继承语义、父 Actor 选择、导弹/贴图/建造事件约定、从非活动 XML 提取 Actor，以及 Actor 往返陷阱。
- **父技能拥有：** `sc2-project-entry` 拥有项目身份、路径与路由。`sc2-catalog-xml` 拥有 Actor 行同样要遵守的项目级 schema、继承与有效值规则。
- **兄弟技能拥有：** `sc2data-actors-visuals` 拥有逐字段的 `CActor*` schema 参考；`sc2-localization` 拥有字符串锚点；`sc2-map-triggers` 拥有 `Send Actor Message` 接线；`sc2-tools-validation` 拥有审计 CLI；`sc2-editor-handoff` 拥有保存/接受闸门。

## 核心规则

1. **Actor 是事件驱动且客户端本地的**——每个 Actor 自己声明监听什么；状态从不在客户端之间同步，在低画质设置下可能不存在。
2. **优先用规范父级**（`GenericUnitStandard`、`GenericAttack`、`ModelAddition`、`SoundOneShot`、`Range Abil`、`Cursor Splat`），而不是手搓它已经提供的东西。
3. **绝不要在基于 `GenericAttack` 的动作 Actor 上同时设 `Attack` 和 `Launch`+`Impact`**；光束/瞬发用 `Attack`，导弹用 `Launch`+`Impact`。
4. **省略 `<Missile>` 字段是正常的**，但仅当父级 token 解析为 `##id##Missile`、导弹 Actor 恰好是 `<ActionActorId>Missile`，且预载数据库一致。
5. **改子 Actor 的 `unitName` 不会重定向父级事件**——要在继承的索引处覆盖与单位相关的事件。
6. **建造事件的索引固定：** `4` = `UnitConstruction.<unit>.Start`，`5` = `.Finish`；只写一个裸 term 或挪位的 `.Start` 会留下重复模型或回退成折跃球。
7. **纯视觉的变形过渡是 `CActorModel parent="ModelAdditionNoAnims"`**，变形开始时创建、结束时销毁——绝不要第二个常驻 `CActorUnit`。
8. **`TextureDeclares` 只做槽位映射**——切换还需要 `TextureSelect*` 消息，所以删声明前先追继承事件。
9. **只有针对某个具体单位实例时才动用触发器**；触发器代码必须驱动 Actor 状态时用 `Send Actor Message`。
10. **任何编辑器保存之后都要审计**重复的 `(catalog type, id)` 行、带索引的命令卡覆盖，以及继承的状态栏 UI 钩子，然后才信任该文件。
11. **模型或音效 ID 存在不等于资源存在**——精简行可能只有 `scale`/`radius`，依赖一个未导出的父级。

## 参考文件

- [references/how-actors-work.md](references/how-actors-work.md) — 事件/term/消息模型、客户端本地异步执行与编辑器事件配色。
- [references/common-actor-fields.md](references/common-actor-fields.md) — 每个常用 `CActor*` 字段及其控制的东西。
- [references/parent-actors.md](references/parent-actors.md) — 每种 Actor 该选哪个父级，外加 Range/Splat token 缺陷与继承注意事项。
- [references/action-actors-and-missile-defaults.md](references/action-actors-and-missile-defaults.md) — 编辑器何时可能丢弃显式 `<Missile>` 值，何时不会。
- [references/construction-events-and-morph-transitions.md](references/construction-events-and-morph-transitions.md) — 带索引的 `UnitConstruction` 事件与安全的变形过渡 Actor。
- [references/texture-declarations-and-swaps.md](references/texture-declarations-and-swaps.md) — 继承的贴图切换与何时可以安全删除声明。
- [references/actor-extraction-from-inactive-xml.md](references/actor-extraction-from-inactive-xml.md) — 从 `XMLFromDependenciesWeDontUse/` 提取单位的表现链。
- [references/actors-vs-triggers.md](references/actors-vs-triggers.md) — 为视觉效果在 Actor 事件与触发器之间做选择。
- [references/roundtrip-and-presentation-faults.md](references/roundtrip-and-presentation-faults.md) — 保存后陷阱、要剥掉的状态栏钩子，以及症状→原因追踪表。
- [references/actors.md](references/actors.md) — Actor 系统完整参考；[references/editor-roundtrip-and-actors.md](references/editor-roundtrip-and-actors.md) — 往返检查单与 Actor XML 模式。
- `skills/sc2-catalog-xml/references/production.md` — 用于现场折跃生产的 Terran 空投舱 Actor 用法。
- `skills/sc2data/sc2data-units-reference/references/unit-extraction-from-inactive-xml.md` — 完整精简提取流程。
