---
name: sc2data-units-reference
description: StarCraft II unit catalog reference — editor IDs, races, attributes, and roles for all multiplayer units and structures across Wings of Liberty, Heart of the Swarm, and Legacy of the Void. Also covers campaign-only units and all 18 co-op commanders with their race, hero unit IDs, and signature units. Use when you need to know the correct catalog/Galaxy type ID for a unit (e.g. "SiegeTank", "HighTemplar", "BroodLord"), or when checking a unit's race, classification, expansion of origin, or which commander a unit belongs to. Referenced by sc2data-units-abilities, galaxy-units-and-groups, sc2data-actors-visuals, and sc2data-effects-weapons. Start by running "python tools/sc2.py start" to read this project's previous task records.
---
## 何时使用

要查某个单位或建筑的确切目录 / Galaxy 类型 ID（例如 `SiegeTank`、`HighTemplar`、`BroodLord`），要确认单位的种族、属性、定位或所属资料片，或者要找仍可作为 parent 或 `UnitCreate` 目标的战役专属与合作指挥官单位时，加载本技能。

## 加载边界

- **本子技能拥有：** SC2 单位目录参考——多人单位的编辑器 ID、种族、属性、定位，战役专属单位，以及 18 位合作指挥官
- **父根技能（`sc2-catalog-xml`）拥有：** 项目约定、schema/语法规则、唯一真源、命名前缀
- **兄弟子技能拥有：** `sc2data-units-abilities`（CUnit XML 编写）、`galaxy-units-and-groups`（Galaxy UnitCreate/GetType）、`sc2data-actors-visuals`（Actor 的单位引用）

## 核心规则

1. 先加载 [`sc2-catalog-xml`](../../sc2-catalog-xml/SKILL.md) 取得项目级 Catalog XML schema 规则；本子技能是查阅参考，不负责任何编写约定。
2. ID 区分大小写，必须与目录条目完全一致。写错 ID 在 Galaxy 里静默无效果，在 XML 里则创建死条目。
3. 关键 ID 到 [ShadowDragon Base.SC2Data UnitData.xml](https://github.com/ShadowDragonSC2/Base.SC2Data/blob/main/GameData/UnitData.xml) 核实。下文的战役与合作表为尽力整理；多人表是权威的。
4. 可变形的单位带两个 ID——脚本与数据里都要处理：`SiegeTank`/`SiegeTankSieged`、`Viking`/`VikingAssault`、`Hellion`/`Hellbat`、`WarpPrism`/`WarpPrismPhasing`、`Gateway`/`WarpGate`、`WidowMine`/`WidowMineBurrowed`、`Lurker`/`LurkerBurrowed`、`SwarmHost`/`SwarmHostBurrowed`、`Disruptor`/`DisruptorPhased`、`SupplyDepot`/`SupplyDepotLowered`。
5. 潜地、拔根与相位形态是独立的 `CUnit` 条目，各自继承花费——写任何花费覆写之前先确认有无变体（见 `sc2data-units-abilities`）。
6. 显示名不等于 ID：坑道虫是 `NydusCanal`；可建造的虫群虫后是 `Queen`，而 `SwarmQueen` 是另一个英雄单位。
7. 工人的建造/训练时间通过数字槽位 `BuildN`/`TrainN` 覆写，绝不用建筑或单位的 ID（见 `sc2data-units-abilities`）。
8. 属性可以叠加（一个单位可同时是 Light + Mechanical + Structure），并驱动 `AttributeBonus` 伤害与目标过滤；XML 的 `Attributes` 名称与 Galaxy 的 `c_unitAttribute*` 常量是同一套。
9. 多人、战役与合作数据各不相同：编辑器里存在的 ID 未必可建造，且多数合作数据位于 `Mods/CoopCampaign.SC2Mod`。

## References

- [references/using-unit-ids.md](references/using-unit-ids.md) — Galaxy 与 XML 用法示例、变形/切换 ID 配对、完整属性清单。
- [references/terran-multiplayer-units.md](references/terran-multiplayer-units.md) — 人类多人单位与建筑 ID（资料片、类型、属性、定位）。
- [references/protoss-multiplayer-units.md](references/protoss-multiplayer-units.md) — 星灵多人单位与建筑 ID。
- [references/zerg-multiplayer-units.md](references/zerg-multiplayer-units.md) — 虫族多人单位与建筑 ID。
- [references/campaign-and-coop-units.md](references/campaign-and-coop-units.md) — 可作为 parent 或生成目标的战役专属人类/虫族/星灵 ID。
- [references/coop-commanders.md](references/coop-commanders.md) — 全部 18 位合作指挥官，含英雄单位 ID 与标志性单位。
- [references/neutral-and-critter-units.md](references/neutral-and-critter-units.md) — 用于地图脚本的矿、瓦斯泉、塔与可破坏物 ID。
- [references/zerg-units.md](references/zerg-units.md) — HotS 虫族示例指南：变种、潜地机制，以及虫群虫后命名陷阱。
- [references/zerg-structures.md](references/zerg-structures.md) — HotS 虫族示例指南：建造时间真正由什么控制，以及 CAbilMorph 升级路径。
- [references/unit-extraction-from-inactive-xml.md](references/unit-extraction-from-inactive-xml.md) — 从不激活依赖导出的数据中重建单位的工作流。
