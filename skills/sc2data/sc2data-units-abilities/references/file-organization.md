# 文件组织 — 一个单位一个文件

如何组织 GameData，让一个单位的所有目录类型都放在同一个独立 XML 文件里，以及 GameData.xml 如何注册它。

## 文件组织模式 — 一个单位一个文件

不要把条目塞进按类型划分的单体文件（`UnitData.xml`、`ActorData.xml` 等），推荐做法是**每个单位或功能一个独立 XML 文件**。该单位的所有目录类型——单位、武器、效果、技能、行为、Actor、模型、声音——都放在这一个文件里。

**目录结构：**
```
Base.SC2Data/
  GameData.xml                             ← lists all included files
  GameData/
    Terran/NCO/Herc/
      Herc.xml                             ← all data for Herc in one file
    Zerg/MyUnit/
      MyUnit.xml                           ← all data for MyUnit in one file
```

模组根目录下的 **`GameData.xml`** 用 `<Includes>` 注册每个文件：
```xml
<?xml version="1.0" encoding="utf-8"?>
<Includes>
    <Catalog path="GameData/Terran/NCO/Herc/Herc.xml"/>
    <Catalog path="GameData/Zerg/MyUnit/MyUnit.xml"/>
</Includes>
```

**每个单位文件**使用单一 `<Catalog>` 根，按顺序包含该单位的所有数据类型：
```xml
<?xml version="1.0" encoding="UTF-8"?>
<Catalog>
    <!-- Button -->
    <CButton id="MyUnit" .../>  

    <!-- Unit -->
    <CUnit id="MyUnit" parent="Generic_Unit_Ground">...</CUnit>

    <!-- Weapon + its effects -->
    <CWeaponLegacy id="MyUnit_Weapon">...</CWeaponLegacy>
    <CEffectDamage id="MyUnit_Weapon@Damage">...</CEffectDamage>

    <!-- Ability + its effects + behaviors -->
    <CAbilEffectTarget id="MyUnit_Ability">...</CAbilEffectTarget>
    <CEffectSet id="MyUnit_Ability@Set">...</CEffectSet>
    <CBehaviorBuff id="MyUnit_Ability@Buff">...</CBehaviorBuff>

    <!-- Actor, Model, Sound -->
    <CActorUnit id="MyUnit" unitName="MyUnit">...</CActorUnit>
    <CModel id="MyUnit" parent="Unit">...</CModel>
    <CSound id="MyUnit@Death" parent="Zerg_ExplosionSmall"/>

    <!-- Requirement -->
    <CRequirement id="MyUnit@Use"/>

    <!-- Data collection — groups entries for the editor UI -->
    <CDataCollectionUnit id="MyUnit">
        <DataRecord Entry="Unit,MyUnit"/>
        <DataRecord Entry="Actor,MyUnit"/>
        <DataRecord Entry="Model,MyUnit"/>
        <DataRecord Entry="Weapon,MyUnit_Weapon"/>
    </CDataCollectionUnit>
</Catalog>
```

**新增一个单位的步骤：**
1. 创建 `GameData/YourFaction/UnitName/UnitName.xml`，根为 `<Catalog>`。
2. 把 `<Catalog path="GameData/YourFaction/UnitName/UnitName.xml"/>` 加进 `GameData.xml`。

> 参见 [ShadowDragon Base.SC2Data](https://github.com/ShadowDragonSC2/Base.SC2Data) —— `GameData.xml` 看 include 清单，任意单位子目录（例如 `GameData/Terran/Campaign/NCO/Herc/Herc.xml`）看一个完整、独立、真实世界的范例。
