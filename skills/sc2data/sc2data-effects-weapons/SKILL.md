---
name: sc2data-effects-weapons
description: SC2 Data Editor — Effects, Weapons, Upgrades, and the damage chain in XML. Use when creating or modifying CEffect* (damage, search, apply behavior, launch missile, set), CWeapon, CUpgrade, or the full chain from weapon through to damage application. Also covers TargetFind, TargetSort, and Footprints. Always consult the catalogsData.xsd schema for exact fields and structure — do not assume unsupported fields exist. Do not use for actors/visuals (sc2data-actors-visuals) or unit/ability containers (sc2data-units-abilities). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

## 何时使用

处理 `CEffect*` 条目（伤害、搜索、施加/移除行为、发射弹体、集合、持续、切换）、`CWeapon`、`CUpgrade`、TargetFind 与 Footprint 时加载本技能——也就是从武器或技能开火，一路到施加伤害、施加行为、生成单位的整条链路。效果与武器定义了游戏逻辑如何执行：造成多少伤害、施加哪些行为、执行哪些搜索、生成哪些单位。

## 加载边界

- **本子技能拥有：** CEffect*、CWeapon、CUpgrade、伤害链、TargetFind、TargetSort 与 Footprint 的 XML schema
- **父根技能（`sc2-catalog-xml`）拥有：** 项目约定、schema/语法规则、唯一真源、命名前缀
- **兄弟子技能拥有：** `sc2data-actors-visuals`（表现效果的 Actor）、`sc2data-units-abilities`（单位/技能容器）、`sc2data-behaviors-validators`（由效果施加的行为）

## 核心规则

1. 先加载 [`sc2-catalog-xml`](../../sc2-catalog-xml/SKILL.md) 取得项目级 Catalog XML schema 规则；本子技能只负责效果、武器与升级的深度 schema。
2. 每次改动都用 Red Hat XML 对照 `catalogsData.xsd` 校验，绝不臆造字段：给每条诊断归类（非法元素、属性、枚举或字段路径/索引），到 schema 里核对允许的结构，只修成 schema 支持的字段，然后重新校验。被要求修 XML 报错时，把这条闭环跑完。
3. 一个技能或武器只触发一个根效果，而根效果几乎总是 `CEffectSet`——哪怕只有一个效果也走集合，这样以后追加内容才不费事。
4. 链中第一个效果决定技能的目标选取类型。把 `CEffectIssueOrder` 放在最前会让技能无法指定目标；需要可指定目标的技能时就把 `CEffectSet` 放第一位。
5. 为新单位或技能创建的效果，放进该单位自己的独立文件（`GameData/<Faction>/<Unit>/<Unit>.xml`）并在 `GameData.xml` 注册，不要塞进单体式的 `EffectData.xml`。
6. `ValidatorArray` 是与门：任一校验器为假，效果就不施加，其关联的 Actor 事件也被抑制。条件性命中要门控在 `CEffectDamage` 上，而不是在武器层做过滤。
7. 伤害的 `Kind`：`Melee` 与 `Ranged` 受护甲减免，`Spell` 默认无视护甲，`Splash` 可命中多个单位。`AttributeBonus index="Armored" value="10"` 在 `Amount` 之外追加。
8. 把武器的 `DisplayEffect` 设为直接的 `CEffectDamage`，游戏内提示条才会显示真实数值；并让它与游戏性根效果 `Effect` 分开。
9. 所有效果 id 保持唯一并前置声明（技能 -> 效果 -> 子效果）。暴雪的数据很密；重名会静默覆盖，循环引用则永不触发。
10. 范围效果中使用标记（`MarkerArray`），让同一个实例无法对同一单位命中两次；`Chance`、`CasterHistory`、`CanBeBlocked` 和 `ResponseFlags`/`AINotifyFlags` 是其余几个横切字段。
11. 升级在研究完成时触发其效果链，最多叠加到 `MaxLevel`（`0` = 不限）；这是全局修改单位属性的受支持做法。
12. 效果与武器通过 TargetFind 类型（`TargetPoint`、`SourcePoint`、`BuffTarget`）解析目标——调试「弹体落在错误位置」之前先核对用的是哪一个。溅射/AoE 用 `CEffectSearch` + `CEffectDamage` 搭，而不是再加一把武器。

## References

- [references/effect-chain-architecture.md](references/effect-chain-architecture.md) — 根效果如何经由集合、搜索与弹体逐级展开。
- [references/weapon-schema.md](references/weapon-schema.md) — 带注释的 CWeapon 与武器字段表。
- [references/effect-types-and-examples.md](references/effect-types-and-examples.md) — 每个常用 CEffect* 都有工作示例，另附完整子类型清单。
- [references/effect-common-fields-and-damage.md](references/effect-common-fields-and-damage.md) — 所有效果共享的字段、标记、响应/AI 标志、伤害类型、属性加成、ValidatorArray 门控。
- [references/upgrades.md](references/upgrades.md) — CUpgrade 条目、等级，以及它们触发的效果。
- [references/targetfind-and-footprints.md](references/targetfind-and-footprints.md) — 效果如何找到目标点，以及 CFootprint 如何阻挡地形。
- [references/naming-conventions.md](references/naming-conventions.md) — 武器、根效果、伤害/搜索效果、升级的 ID 命名。
- [references/external-references.md](references/external-references.md) — 效果/武器 wiki 页面与 catalogsData.xsd schema 地址。
