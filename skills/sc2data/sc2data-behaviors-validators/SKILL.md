---
name: sc2data-behaviors-validators
description: SC2 Data Editor — Behaviors (buffs, debuffs, auras, timers) and Validators (conditional tests) in XML. Use when creating or modifying CBehavior* (buff, attribute modifier, unit tracker, reveal) or CValidator* (unit type, unit order, comparison, combine) entries. Also covers behavior stacking, duration, Vitals modification, and how validators gate effects, abilities, and behaviors. Always consult the catalogsData.xsd schema for exact fields and structure — do not assume unsupported fields exist. Do not use for applying a behavior via an effect (use sc2data-effects-weapons) or actor visuals tied to a behavior (use sc2data-actors-visuals). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

## 何时使用

处理 `CBehavior*` 条目（增益、减益、光环、计时器、属性修改、单位追踪、揭示）和 `CValidator*` 条目（单位类型、单位命令、比较、组合）时加载本技能，包括行为叠加、持续时间、命值（Vitals）修改，以及校验器如何为效果、技能、行为和 Actor 把关。行为是单位身上持续存在的被动效果（增益、减益、光环、限时生命、属性修改）；校验器是数据中任何位置都能用来把关的布尔判定。

## 加载边界

- **本子技能拥有：** CBehavior*（增益、减益、光环、计时器）与 CValidator*（单位类型、命令、比较、组合）的 XML schema，行为叠加、持续时间和命值修改
- **父根技能（`sc2-catalog-xml`）拥有：** 项目约定、schema/语法规则、唯一真源、命名前缀
- **兄弟子技能拥有：** `sc2data-effects-weapons`（通过效果施加行为）、`sc2data-actors-visuals`（与行为绑定的 Actor 视觉表现）、`sc2data-units-abilities`（单位/技能容器）

## 核心规则

1. 先加载 [`sc2-catalog-xml`](../../sc2-catalog-xml/SKILL.md) 取得项目级 Catalog XML schema 规则；本子技能只负责行为与校验器的深度 schema。
2. 每次改动都用 Red Hat XML 对照 `catalogsData.xsd` 校验，绝不臆造字段：给每条诊断归类（非法元素、属性、枚举或字段路径/索引），到 schema 里核对允许的结构，只修成 schema 支持的字段，然后重新校验。被要求修 XML 报错时，把这条闭环跑完。
3. 为新单位或技能创建的行为，放进该单位自己的独立文件（`GameData/<Faction>/<Unit>/<Unit>.xml`）并在 `GameData.xml` 注册，不要塞进单体式的 `BehaviorData.xml`。
4. 行为由 `CEffectApplyBehavior` 施加，由 `CEffectRemoveBehavior` 或到期移除——行为条目本身从不决定自己何时被施加。
5. `Duration value="0"` 表示在被移除前永久存在。多数增益都要设 `MaxCount value="1"` 以防意外叠加；叠加模式有 `Duration`（重置计时器）、`None`（只计数）和 `Any`（两者各取最大）。
6. 校验器槽位要选得有意为之：`Disable` 校验器返回假会抑制行为但保留实例（可开关的光环），`Remove` 校验器返回假则永久剥离。
7. 校验器是无副作用的布尔判定：返回假意味着效果不施加、技能置灰，或行为被禁用/移除——具体看它挂在哪里。
8. 校验器按真条件命名——`TargetIsAlive`、`CasterNotBurrowed`，绝不要写成 `NotDead`。
9. 组合逻辑要在别处复用时，用 `CValidatorCombine` 配 `And`/`Or`/`Not`；单个校验器保持小而可组合。
10. 比较运算符是 `LT`、`LE`、`EQ`、`GE`、`GT`、`NE`；`Negate value="1"` 反转 `CValidatorUnitType`。
11. 光环的做法：源行为上的周期效果 + `CEffectSearch` + 对命中目标施加行为。绝不要从行为直接给每个单位施加光环。
12. 在 Actor 语境里，校验器通过 `ValidateUnit <UpgradeId>` 触达；事件接线见 `sc2data-actors-visuals`。

## References

- [references/behavior-types-and-fields.md](references/behavior-types-and-fields.md) — CBehavior* 类型清单、CBehaviorBuff 与 CBehaviorAttributeModifier 示例及其字段表。
- [references/behavior-stacking-and-auras.md](references/behavior-stacking-and-auras.md) — 叠加计数与叠加模式，以及「行为 + 搜索 + 施加」的光环模式。
- [references/validator-types-and-examples.md](references/validator-types-and-examples.md) — 全部 CValidator* 类型，每个常用类型都有一个工作示例。
- [references/validators-in-context.md](references/validators-in-context.md) — 校验器返回假时，在效果、技能、行为的 Disable/Remove 槽和 Actor 项中究竟发生什么。
- [references/external-references.md](references/external-references.md) — 行为与校验器的 wiki 页面和 catalogsData.xsd schema 地址。
