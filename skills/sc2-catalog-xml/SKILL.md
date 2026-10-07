---
name: sc2-catalog-xml
description: StarCraft II GameData XML authoring for the AeonOfIhanrii campaign. Use when creating or modifying units, abilities, actors, behaviors, effects, weapons, upgrades, requirements, validators, or footprints. Covers schema rules, parent inheritance, effective-value evidence, localization, and editor round-trip safety.
---
# SC2 Catalog XML 编写

## 何时使用

在编写或编辑 `*.SC2Mod/Base.SC2Data/GameData/*.xml` 下的 GameData XML 时加载本技能——单位、技能、Actor、行为（Behavior）、效果（Effect）、武器、升级、需求、验证器、足迹，或任何 catalog 对象。

本技能是 **Catalog XML 路由**。它掌管项目级 schema 规则、父级继承、有效值证据、本地化锚点与编辑器往返安全。针对某个 catalog 族的深层 XML 工作，路由到 `skills/sc2data/` 下对应的 `sc2data-*` 子技能。

## 加载边界

- **本技能拥有：** 项目级 schema 规则、有效值证据流程、父级/继承约定、足迹与 `EditorCategories` 策略、XML→本地化锚点约定。
- **父技能拥有：** `sc2-project-entry` 拥有项目身份、路径与整体实现流程；AGENTS.md 拥有全仓库硬规则。
- **兄弟技能拥有：** `sc2data-*` 子技能拥有按族的 schema 与样例；`sc2-actor-system` 拥有 Actor 事件与提取；`sc2-localization` 拥有字符串文件；`sc2-tools-validation` 拥有工具 CLI；`sc2-editor-handoff` 拥有问题生命周期与验证闸门。

### Catalog 子技能路由

针对某个 catalog 族的深层 XML 编写，加载对应的 `sc2data-*` 子技能。本技能保留所有子技能都遵守的项目级 **schema 规则** 与 **有效值流程**。

| 任务 | 子技能 |
|---|---|
| `CUnit` / `CAbil*` / `CMover` / `CTurret` XML | `skills/sc2data/sc2data-units-abilities/SKILL.md` |
| `CBehavior*` / `CValidator*` XML | `skills/sc2data/sc2data-behaviors-validators/SKILL.md` |
| `CEffect*` / `CWeapon` / `CUpgrade` / 伤害链 XML | `skills/sc2data/sc2data-effects-weapons/SKILL.md` |
| `CActor*` XML（Unit/Action/Model/Beam/Sound） | `skills/sc2data/sc2data-actors-visuals/SKILL.md` |
| `.BlizWiz` XML 文件、向导模板 | `skills/sc2data/sc2data-wizards/SKILL.md` |
| 单位 catalog 参考（编辑器 ID、种族、18 个合作指挥官） | `skills/sc2data/sc2data-units-reference/SKILL.md` |

子技能主题与顶层技能重叠时，顶层技能拥有项目约定，子技能拥有 schema 参考。

## 核心规则

1. **先查询再落笔：** 先对目标 ID/族跑 `python tools/sc2-catalog-query.py`——字段名和 ID 区分大小写，绝不允许猜。
2. **改数值前先证明有效值：** 记录本地行、父级、精确字段/索引处的继承值，以及打算做的覆盖。子级缺字段也可能是对的。
3. **新行写进带类型的 catalog**（`AbilData.xml`、`UnitData.xml`、`ButtonData.xml` …），绝不写进 `GameData.xml`；绝不留两行相同的 `(catalog type, id)`。
4. **XML 只用 ASCII**，取值写成子元素（`<LifeMax value="100"/>`），数组字段必须显式写 `index=`；不许 `AbilAutoCmd`，不许 `CBehaviorBuff.InitEffect`，不许纯注释的 catalog。
5. **`parent=` 只是数据字段继承**——它不产生依赖也不产生解锁；绝不要把自定义 `CUpgrade` 挂到 `HotS*` 下。
6. **`CEffectDamage` 只接受 `Amount`**——`ImpactLocation`、`WhichUnit`、`KillType` 不存在；`DamageDealtFraction` 是累加的（`0.6` = +60%）。
7. **`CValidatorCombine` 默认是 `Or`**——显式写 `<Type value="And"/>`，用 `<Negate value="1"/>` 取反，绝不用 `Type="Not"`。
8. **异虫护甲升级：** `ZergGroundArmorsLevel1/2/3` 各触发一次——只有三者都带 `EffectArray` 条目时，单位才拿到 +3。
9. **折跃：** 同一建筑上的多个 `CAbilWarpTrain` 必须共用一个 `Location=Unit` 的充能链接；普通训练成功绝不证明折跃成功。
10. **面向玩家的文本需要显式 XML 锚点**（`<Description value="Unit/Tooltip/MyUnit"/>`）加上 `GameStrings.txt` 键；每个新 catalog ID 还需要 `ObjectStrings.txt` 的 `Type/Name/ID` 与 `Type/EditorPrefix/ID`。
11. **添加验证器或实体前先查重**，它可能已存在于当前依赖链中。
12. **静态通过不等于编辑器接受**——验证器通过并不证明编辑器或打包后运行时接受该改动。

## 参考文件

- [references/catalog-authoring-workflow.md](references/catalog-authoring-workflow.md) — 强制的编辑前查询/取证顺序与编辑后校验；任何 XML 改动前读它。
- [references/xml-schema-basics.md](references/xml-schema-basics.md) — 取值子元素、行为嵌套、命令卡布局、数组索引、ASCII 规则。
- [references/inheritance-and-parent-pitfalls.md](references/inheritance-and-parent-pitfalls.md) — `parent=` 语义、Actor 与变形变体、验证器、伤害与升级陷阱。
- [references/production-command-cards-and-costs.md](references/production-command-cards-and-costs.md) — 生产链核验、命令卡子菜单、费用显示、折跃与升级注意事项。
- [references/footprints-and-editor-categories.md](references/footprints-and-editor-categories.md) — 足迹层、菌毯放置、安全的 `EditorCategories` 收拢。
- [references/xml-patterns.md](references/xml-patterns.md) — 已验证 XML 模式页的枢纽索引（[references/core.md](references/core.md)、[references/catalog-rules.md](references/catalog-rules.md)、[references/production.md](references/production.md)）。
- [references/source-caveats.md](references/source-caveats.md) — 源文档中已记录的矛盾；照抄任何示例前读它。
