# 单位 ID 的使用

单位 ID 从哪来，以及在 Galaxy 与 XML 中如何使用，含变形/切换 ID 配对与单位属性清单。

本文件列出全部星际争霸 II 多人单位与建筑及其**编辑器目录 ID**——Galaxy 脚本（如 `UnitCreate(1, "Marine", ...)`）与 XML 数据（如 `<CUnit id="Marine"/>`）中使用的确切字符串。

> **唯一真源：** [ShadowDragon Base.SC2Data — UnitData.xml](https://github.com/ShadowDragonSC2/Base.SC2Data/blob/main/GameData/UnitData.xml)。有疑问时到那里核对确切 ID。

> **资料片缩写：** WoL = 自由之翼，HotS = 虫群之心，LotV = 虚空之遗

> **建造槽位映射：** 通过 `CAbilBuild`（`TerranBuild`、`ProtossBuild`、`ZergBuild`）修改工人建造时间时，InfoArray 槽位是数字（`Build1`、`Build2`、……）——**不是**建筑的目录 ID。三个种族的完整已核实槽位到建筑映射与标准建造时间见 `sc2data-units-abilities` 技能。

## 使用单位 ID

### 在 Galaxy 脚本中

```galaxy
// Unit type ID is always the exact catalog ID string
unit lv_marine = UnitCreate(1, "Marine", c_unitCreateIgnorePlacement, 1, lv_point, 270.0);
bool lv_isZealot = (UnitGetType(lv_unit) == "Zealot");

// Siege tank has two IDs — check both for unsieged + sieged  
bool lv_isTank = (UnitGetType(lv_unit) == "SiegeTank") || (UnitGetType(lv_unit) == "SiegeTankSieged");
```

### 在 XML 数据中

```xml
<!-- The id attribute must match the catalog entry exactly -->
<CUnit id="MyCustomUnit" parent="Marine">
    ...
</CUnit>

<!-- Referencing a Blizzard-defined unit as parent -->
<CUnit id="StrongerZergling" parent="Zergling">
    <LifeMax value="75"/>
</CUnit>
```

### 变形/切换 ID 配对

有些单位的两种形态各有一个 ID——两者都要处理：

| 单位 | 常规 ID | 交替 ID |
|---|---|---|
| Siege Tank | `SiegeTank` | `SiegeTankSieged` |
| Viking | `Viking` | `VikingAssault` |
| Hellion | `Hellion` | `Hellbat` |
| Warp Prism | `WarpPrism` | `WarpPrismPhasing` |
| Gateway | `Gateway` | `WarpGate` |
| Widow Mine | `WidowMine` | `WidowMineBurrowed` |
| Lurker | `Lurker` | `LurkerBurrowed` |
| Swarm Host | `SwarmHost` | `SwarmHostBurrowed` |
| Disruptor | `Disruptor` | `DisruptorPhased` |
| Supply Depot | `SupplyDepot` | `SupplyDepotLowered` |

## 单位属性速查

这些是 XML 中使用的 `Attributes` 取值，对应 Galaxy 中的 `c_unitAttribute*` 常量。

| 属性 | 典型单位 | Galaxy 常量 |
|---|---|---|
| Light | 陆战队员、跳虫、多数基础单位 | `c_unitAttributeLight` |
| Armored | 蟑螂、劫掠者、追猎者 | `c_unitAttributeArmored` |
| Biological | 所有有机单位（人类步兵、全部虫族、多数星灵战士） | `c_unitAttributeBiological` |
| Mechanical | 全部人类载具/舰船、星灵机械 | `c_unitAttributeMechanical` |
| Massive | 雷兽、雷神、航母、风暴战舰、巢虫领主 | `c_unitAttributeMassive` |
| Psionic | 幽灵、高阶/黑暗圣堂武士、监管者、感染虫、多数星灵 | `c_unitAttributePsionic` |
| Structure | 所有建筑 | `c_unitAttributeStructure` |
| Hover | 恶火（可越过部分地形） | `c_unitAttributeHover` |
