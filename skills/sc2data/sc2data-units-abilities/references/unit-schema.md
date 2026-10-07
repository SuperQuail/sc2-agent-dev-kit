# 单位 schema（UnitData.xml）

Catalog XML 条目基础，加上 CUnit / CUnitHero 字段、工作示例与单位关键字段表。

数据编辑器把所有游戏数据以 XML 存放在 `Base.SC2Data/GameData/` 下。每个条目都是 `<Catalog>` 的子元素。你通过新增或覆写条目来编辑；你的模组/地图未定义的一切都继承暴雪基础数据。

## XML 基础

```xml
<?xml version="1.0" encoding="utf-8"?>
<Catalog>
    <!-- Each entry has an id and optionally a parent to inherit from -->
    <CUnit id="MyHero" parent="Zergling">
        <!-- Fields that differ from the parent only -->
        <LifeMax value="500"/>
        <ShieldsMax value="0"/>
        <Speed value="4.13"/>
    </CUnit>
</Catalog>
```

**规则：**
- `id` 属性是数据键，数据与 Galaxy 中其他地方都用它引用。
- 省略 `parent` 时，条目没有显式父条目（继承默认基础）。
- 只需写出与父条目**不同**的字段——其余全部继承。
- `default="1"` 把条目标记为该类型的默认覆写。
- 被注释的 XML（`<!-- ... -->`）是暴雪留下的活文档；可以参考但不生效。
- 数组用 `index` 定位具体槽位：`<WeaponArray index="0" Link="MyWeapon"/>`。

## 单位（`UnitData.xml`）

文件：`Base.SC2Data/GameData/UnitData.xml`  
类型：`CUnit`、`CUnitHero`

### 最小单位覆写

```xml
<CUnit id="MyUnit" parent="Marine">
    <Name value="Unit/Name/MyUnit"/>          <!-- references GameStrings.txt -->
    <LifeMax value="200"/>
    <LifeArmor value="1"/>
    <ShieldsMax value="0"/>
    <ShieldsArmor value="0"/>
    <EnergyMax value="0"/>
    <EnergyStart value="0"/>
    <Speed value="2.25"/>
    <Sight value="9"/>
    <Radius value="0.375"/>
    <EditorCategories value="Race:Terran"/>
    <AbilArray index="Move" Link="Move"/>
    <AbilArray index="Attack" Link="Attack"/>
    <AbilArray index="Stop" Link="Stop"/>
    <WeaponArray index="0" Link="MarineWeapon"/>
    <Attributes index="Light" value="1"/>
    <Attributes index="Biological" value="1"/>
    <CargoSize value="1"/>
</CUnit>
```

### 英雄单位

```xml
<CUnitHero id="MyHeroUnit" parent="Zergling">
    <LifeMax value="600"/>
    <XP index="0" value="0"/>     <!-- XP thresholds per level -->
    <XP index="1" value="300"/>
    <XP index="2" value="600"/>
    <!-- Hero-specific fields -->
</CUnitHero>
```

### 单位关键字段

| 字段 | 用途 |
|---|---|
| `LifeMax` | 最大生命值 |
| `LifeArmor` | 护甲值 |
| `ShieldsMax` | 星灵护盾 |
| `EnergyMax` / `EnergyStart` | 能量池 |
| `Speed` | 移动速度 |
| `Sight` | 视野半径 |
| `Radius` | 碰撞半径 |
| `Height` | 飞行高度 |
| `AbilArray` | 挂在该单位上的技能 |
| `WeaponArray` | 挂在该单位上的武器 |
| `BehaviorArray` | 生成时的默认行为 |
| `CargoSize` | 占用的运输位 |
| `FoodCost` | 人口花费 |
| `Attributes` | Light/Heavy/Biological/Mechanical/Psionic/Massive/Structure/Hover |
| `MovementType` | Ground/Fly/Burrowed/Cliff |
| `PlacementRadius` | 放置用的占地半径 |
| `EditorCategories` | 组织性标签（Race、AbilityorEffectType 等） |
