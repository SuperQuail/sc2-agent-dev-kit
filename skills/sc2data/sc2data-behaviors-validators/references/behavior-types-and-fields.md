# 行为类型与字段

行为条目的存放位置、CBehavior* 类型清单、CBehaviorBuff 与 CBehaviorAttributeModifier 示例，以及它们的字段表。

> **文件放置：** 为新单位或技能创建行为时，放进该单位自己的独立 XML 文件（`GameData/Faction/UnitName/UnitName.xml`），不要塞进单体式的 `BehaviorData.xml`。在 `GameData.xml` 中用 `<Catalog path="..."/>` 注册该文件。完整文件模板见 sc2data-units-abilities 技能。

## 行为（`BehaviorData.xml`）

文件：`Base.SC2Data/GameData/BehaviorData.xml`

行为经 `CEffectApplyBehavior` 施加到单位，经 `CEffectRemoveBehavior` 或到期移除。

### 行为类型

| XML 类型 | 用途 |
|---|---|
| `CBehaviorBuff` | 通用增益/减益；最常用的类型 |
| `CBehaviorAttributeModifier` | 修改单位属性（Speed、Armor、HP 等） |
| `CBehaviorUnitTracker` | 追踪单位，达到数量阈值时触发效果 |
| `CBehaviorReveal` | 揭示单位（破除潜行，或把友军暴露给敌方） |
| `CBehaviorAbilityModifier` | 动态修改技能参数 |
| `CBehaviorResource` | 资源回复（晶体矿、高能瓦斯） |

### CBehaviorBuff — 通用增益/减益

```xml
<CBehaviorBuff id="MyStunBehavior">
    <EditorCategories value="AbilityorEffectType:Targeted"/>
    <Duration value="3"/>                          <!-- seconds; 0 = permanent until removed -->
    <DurationBonusArray index="HeroKills" value="1"/>  <!-- duration bonus per hero kill (if applicable) -->
    <MaxCount value="1"/>                          <!-- max stacking instances on one unit -->

    <!-- While active: disable unit's movement and attack -->
    <DisableArray index="Move" value="1"/>
    <DisableArray index="Attack" value="1"/>

    <!-- Validators to keep behavior active; false = disable while condition fails -->
    <ValidatorArray type="Disable" index="0" value="IsTargetAlive"/>

    <!-- Validators to permanently remove behavior -->
    <ValidatorArray type="Remove" index="0" value="IsTargetAboveHalfHP"/>

    <!-- Periodic effect while behavior is active -->
    <PeriodicEffectArray index="0" value="MyDoTEffect" Period="1"/>

    <!-- Effects fired on apply/expire -->
    <OnUnitBirth value="MyBuffApplyEffect"/>
    <OnRemove value="MyBuffExpireEffect"/>
</CBehaviorBuff>
```

### CBehaviorBuff 关键字段

| 字段 | 用途 |
|---|---|
| `Duration` | 行为持续多久（0 = 永久） |
| `MaxCount` | 单个单位上的最大叠加层数 |
| `DisableArray` | 生效期间禁用某项单位命令（Move、Attack、Hold 等） |
| `PeriodicEffectArray` | 生效期间每隔 N 秒触发的效果 |
| `OnUnitBirth` | 行为首次施加时触发的效果 |
| `OnRemove` | 行为到期或被移除时触发的效果 |
| `InitVitalArray` | 施加时修改某项命值（例如施加即治疗） |
| `VitalMaxArray` | 生效期间修改命值上限 |

### CBehaviorAttributeModifier — 属性修改

```xml
<CBehaviorAttributeModifier id="SpeedBoost">
    <Duration value="10"/>
    <Modification>
        <SpeedMultiplier value="1.5"/>     <!-- 50% speed increase -->
    </Modification>
</CBehaviorAttributeModifier>

<CBehaviorAttributeModifier id="ArmorReduction">
    <Duration value="0"/>                  <!-- permanent until removed -->
    <Modification>
        <LifeArmorBonus value="-2"/>       <!-- reduce armor by 2 -->
    </Modification>
</CBehaviorAttributeModifier>
```

### 常用 Modification 字段

| 字段 | 含义 |
|---|---|
| `SpeedMultiplier` | 移动速度倍率（1.0 = 不变） |
| `LifeArmorBonus` | 增减生命护甲 |
| `ShieldArmorBonus` | 增减护盾护甲 |
| `KillBountyModifier` | 击杀赏金倍率 |
| `DamageDealtFraction` | 输出伤害倍率 |
| `DamageTakenFraction` | 承受伤害倍率 |
| `LifeRegenRate` | 每秒生命回复量 |
