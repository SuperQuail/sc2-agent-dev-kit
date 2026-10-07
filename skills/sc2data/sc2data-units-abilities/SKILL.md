---
name: sc2data-units-abilities
description: SC2 Data Editor — Units, Abilities, Movers, Turrets, Requirements, and Races in XML. Use when creating or modifying units (CUnit/CUnitHero), abilities (CAbilEffectTarget, CAbilEffectInstant, CAbilResearch, etc.), movement (CMover), turrets (CTurret), or tech requirements in GameData XML files. Always consult the catalogsData.xsd schema for exact fields and structure — do not assume unsupported fields exist. Do not use for Actors/visuals (use sc2data-actors-visuals), damage/effects (use sc2data-effects-weapons), or Galaxy scripting (use the galaxy-* skills). Start by running "python tools/sc2.py start" to read this project's previous task records.
---
## 何时使用

处理 GameData XML 中的 `CUnit`/`CUnitHero`、`CAbil*`（EffectTarget、EffectInstant、Research、Train、Build、Morph）、`CMover`、`CTurret`、`CRequirement` 和 `CRace` 条目时加载本技能，也包括每个单位的文件布局，以及已核实的 Void.SC2Mod 建造/训练槽位映射。

## 加载边界

- **本子技能拥有：** CUnit/CUnitHero、CAbil*（EffectTarget/EffectInstant/Research）、CMover、CTurret、CRequirement 与 CRace 的 XML schema
- **父根技能（`sc2-catalog-xml`）拥有：** 项目约定、schema/语法规则、唯一真源、命名前缀
- **兄弟子技能拥有：** `sc2data-behaviors-validators`（CBehavior/CValidator）、`sc2data-effects-weapons`（CEffect/CWeapon）、`sc2data-actors-visuals`（CActor*）、`galaxy-units-and-groups`（Galaxy 单位 API）

## 核心规则

1. 先加载 [`sc2-catalog-xml`](../../sc2-catalog-xml/SKILL.md)：子值元素（而非裸属性）、纯 ASCII 文本、不用 `AbilAutoCmd`、不用 `CBehaviorBuff.InitEffect`、不用只有注释的目录、父条目继承陷阱、有效值取证流程、本地化锚点、编辑器往返安全，都由它负责。本子技能只负责深度 schema 参考。
2. 每次改动都用 Red Hat XML 对照 `catalogsData.xsd` 校验，绝不臆造字段：给每条诊断归类（非法元素、属性、枚举或字段路径/索引），到 schema 里核对允许的结构，只修成 schema 支持的字段，然后重新校验。被要求修 XML 报错时，把这条闭环跑完。
3. 只写与 `parent` 不同的字段；省略的一切都继承。`default="1"` 标记某类型的默认覆写，被注释掉的条目是不生效的文档。
4. 新单位的所有目录类型放进同一个自包含文件 `GameData/<Faction>/<Unit>/<Unit>.xml`，并在 `GameData.xml` 中用 `<Catalog path="..."/>` 注册——不要往单体式的 `UnitData.xml` 里追加。
5. 数组槽位是位置性的：一律写 `index="Train1"`、`index="Build1"`。在那里写单位或建筑的目录 ID（例如 `index="Marine"`）会静默创建一个死条目，什么也不改变。
6. 改动花费时，必须覆写每一个变体 CUnit：人类 `*Flying` 形态、`SupplyDepotLowered`、三个 TechLab 与 Reactor 挂件、以及虫族变形/潜地形态——否则 SC2 会在切换时扣费或退费。
7. 变形花费是累计的：变形目标的 `CUnit` 花费设为源花费 + 升级增量（`OrbitalCommand` = CC + 升级），绝不要只写增量。
8. 如果某个变体目录 ID 不在 [references/variant-cost-pitfalls.md](references/variant-cost-pitfalls.md) 的已确认清单里，先请用户在数据编辑器中核实，再写覆写。
9. `id` 是其他所有目录与 Galaxy 使用的数据键；保持 PascalCase 且稳定。数组用 `index` 定位；要禁用继承来的元素，就在同一索引上用空值或零值覆写。
10. 缺失的 `CRequirement`/`CRequirementNode` 会静默禁用它所门控的技能或按钮——找技能数据之前先查需求。
11. `CDataCollectionUnit` 只是把条目分组给编辑器界面看；它不创建也不覆写任何东西。真正起作用的是 `CUnit` 条目。
12. `CAbilMorph` 覆盖科技升级与交替形态（Lair、Hive、Greater Spire、Orbital Command、Planetary Fortress）——它们不是 `CAbilBuild`/训练槽位，且使用不同的 InfoArray 键格式。

## References

- [references/file-organization.md](references/file-organization.md) — 新增单位文件或改 `GameData.xml` include 之前先读。
- [references/unit-schema.md](references/unit-schema.md) — CUnit/CUnitHero 字段、可运行的覆写示例、单位关键字段表。
- [references/variant-cost-pitfalls.md](references/variant-cost-pitfalls.md) — 覆写任何人类或虫族单位花费之前先读；列出全部已知变体 CUnit。
- [references/ability-schema.md](references/ability-schema.md) — 完整的 CAbil* 子类型清单、工作示例，以及技能字段参考（Set ID、冷却、充能、标志）。
- [references/build-and-train-slots.md](references/build-and-train-slots.md) — 已核实的 Void.SC2Mod 训练/建造槽位表、标准时间，以及变形花费陷阱。
- [references/ability-cost-and-targeting.md](references/ability-cost-and-targeting.md) — 花费、冷却、充能字段，以及 TargetFilters 语法。
- [references/movers-turrets-buttons-requirements.md](references/movers-turrets-buttons-requirements.md) — CMover、CTurret、CButton 与 CRequirement 的 schema 与示例。
- [references/naming-and-editor-conventions.md](references/naming-and-editor-conventions.md) — ID 命名约定、推荐的数据编辑器视图设置、最佳实践。
- [references/button-images.md](references/button-images.md) — 挑选 `DefaultButtonFace`/`Icon` 时检索 806 条按钮图标路径。
- [references/external-references.md](references/external-references.md) — 上游 XML 来源、wiki 页面，以及 catalogsData.xsd schema 地址。
