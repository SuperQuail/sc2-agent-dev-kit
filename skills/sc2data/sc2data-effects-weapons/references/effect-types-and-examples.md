# 效果类型与示例

效果条目的存放位置、每个常用 CEffect* 的工作示例，以及完整的子类型清单。

> **文件放置：** 为新单位或技能创建效果时，放进该单位自己的独立 XML 文件（`GameData/Faction/UnitName/UnitName.xml`），不要塞进单体式的 `EffectData.xml`。在 `GameData.xml` 中用 `<Catalog path="..."/>` 注册该文件。完整文件模板见 sc2data-units-abilities 技能。

## 效果链架构

```
CWeapon
  └─ Effect ──► CEffectSet
                  ├─► CEffectDamage             (subtract HP)
                  ├─► CEffectApplyBehavior      (add buff/debuff)
                  ├─► CEffectCreateUnit         (spawn unit)
                  ├─► CEffectSearch / EnumArea  (AoE spread)
                  │     └─► CEffectDamage
                  └─► CEffectLaunchMissile ──► CEffectDamage
```

技能或武器只触发**一个根效果**。这个根几乎总是 `CEffectSet`，由它串起多个阶段。

## 武器（`WeaponData.xml`）

文件：`Base.SC2Data/GameData/WeaponData.xml`

```xml
<CWeapon id="MyWeapon">
    <EditorCategories value="Race:Terran"/>
    <DisplayEffect value="MyWeaponDamage"/>     <!-- effect shown in tooltip for damage display -->
    <Effect value="MyWeaponEffect"/>            <!-- root effect to fire on attack -->
    <Range value="5"/>                          <!-- attack range -->
    <Period value="0.8608"/>                    <!-- attack period (seconds between attacks) -->
    <DamagePoint value="0.0"/>                  <!-- % of Period before damage applies (0–1) -->
    <BackswingPoint value="0.5"/>              <!-- % of Period before unit can move/turn -->
    <Arc value="360"/>                          <!-- attack arc in degrees -->
    <Flags index="AttackTargetMover" value="1"/>
    <TargetFilters value="Visible;Self,Ally,Neutral,Dead,Invulnerable"/>
</CWeapon>
```

### 武器关键字段

| 字段 | 用途 |
|---|---|
| `Effect` | 攻击时触发的根效果 |
| `DisplayEffect` | 用于提示条伤害显示的效果（可以与前者不同） |
| `Range` | 攻击距离 |
| `Period` | 攻击速度——每次攻击的秒数 |
| `DamagePoint` | 在 Period 动画的哪个位置结算命中（0.0–1.0） |
| `BackswingPoint` | 攻击后单位何时可以再次移动 |
| `Arc` | 方向性武器的锥形角度 |
| `TargetFilters` | 哪些目标可被攻击 |

## 效果类型

文件：`Base.SC2Data/GameData/EffectData.xml`

### CEffectSet — 串联多个效果

```xml
<CEffectSet id="MyAbilityEffect">
    <EffectArray index="0" value="MyAbilityDamage"/>
    <EffectArray index="1" value="MyAbilityApplyBuff"/>
    <EffectArray index="2" value="MyAbilitySpawnUnit"/>
</CEffectSet>
```

### CEffectDamage — 造成伤害

```xml
<CEffectDamage id="MyWeaponDamage">
    <Amount value="25"/>
    <Kind value="Ranged"/>              <!-- Melee, Ranged, Spell, Splash -->
    <DamageModifierSource value="Weapon"/>
    <ValidatorArray index="0" value="IsTargetNotInvulnerable"/>
    <AttributeBonus index="Armored" value="10"/>  <!-- bonus vs Armored: 25+10=35 -->
    <AttributeBonus index="Light" value="5"/>
</CEffectDamage>
```

### CEffectApplyBehavior — 施加增益/减益

```xml
<CEffectApplyBehavior id="MyStunApply">
    <Behavior value="MyStunBehavior"/>
    <ValidatorArray index="0" value="IsTargetAlive"/>
</CEffectApplyBehavior>
```

### CEffectRemoveBehavior — 剥离增益

```xml
<CEffectRemoveBehavior id="MyBuffRemove">
    <Behavior value="MyBuff"/>
</CEffectRemoveBehavior>
```

### CEffectSearch — 按半径做 AoE

```xml
<CEffectSearch id="MyAoeSearch">
    <AreaArray index="0" Radius="2.5" Effect="MyAoeDamage"
               TargetFilters="Visible,Alive;Self,Ally,Neutral,Dead"/>
    <SearchFlags index="SameCliff" value="1"/>   <!-- ignore units on different cliff levels -->
</CEffectSearch>
```

### CEffectEnumArea — 自定义形状 AoE

```xml
<CEffectEnumArea id="MyLineAoe">
    <AreaArray index="0" Effect="MyAoeDamage" Radius="2" Arc="30" Distance="8"/>
</CEffectEnumArea>
```

### CEffectLaunchMissile — 生成弹体

```xml
<CEffectLaunchMissile id="MyMissileLaunch">
    <Mover value="MyMissile"/>
    <ImpactEffect value="MyMissileImpact"/>
    <LaunchMissileFlags index="DeleteOnImpact" value="1"/>
    <ImpactLocation><Effect value="TargetPoint"/></ImpactLocation>
</CEffectLaunchMissile>
```

### CEffectCreateUnit — 生成单位

```xml
<CEffectCreateUnit id="SpawnMyUnit">
    <UnitType value="MySpawnUnit"/>
    <Owner value="Caster"/>              <!-- Caster, Neutral, etc. -->
    <Count value="1"/>
</CEffectCreateUnit>
```

### CEffectIssueOrder — 命令单位

```xml
<CEffectIssueOrder id="ForceAttack">
    <Ability value="Attack"/>
    <AbilityCmd value="Attack"/>
</CEffectIssueOrder>
```

### CEffectModifyPlayer — 修改玩家资源

```xml
<CEffectModifyPlayer id="GrantMinerals">
    <Owner value="Caster"/>
    <Resource index="Minerals" value="50"/>   <!-- add 50 minerals to caster's player -->
</CEffectModifyPlayer>
```

## 效果子类型参考

全部 `CEffect*` 子类型清单，便于建立整体印象：

| 子类型 | 用途 |
|---|---|
| `CEffectSet` | 顺序串联多个效果 |
| `CEffectDamage` | 造成生命值伤害 |
| `CEffectApplyBehavior` | 施加增益/减益行为 |
| `CEffectRemoveBehavior` | 剥离行为 |
| `CEffectSearch` | 按半径做 AoE——对所有命中目标运行子效果 |
| `CEffectEnumArea` | 自定义形状 AoE（扇形、直线） |
| `CEffectLaunchMissile` | 生成弹体，由其投送命中效果 |
| `CEffectCreateUnit` | 生成单位 |
| `CEffectIssueOrder` | 命令单位执行某技能 |
| `CEffectModifyPlayer` | 修改玩家资源（晶体矿、瓦斯、人口） |
| `CEffectModifyUnit` | 直接修改单位命值或属性 |
| `CEffectCreatePersistent` | 创建限时持续区域（用于灵能风暴这类技能） |
| `CEffectDestroyPersistent` | 提前结束持续效果 |
| `CEffectSwitch` | 条件分支——按索引选效果 |
| `CEffectRandom` | 随机分支——从效果数组中随机取一个 |
| `CEffectTeleport` | 把单位瞬移到新位置 |
| `CEffectMorph` | 把单位变形为另一单位类型 |
| `CEffectTransferBehavior` | 把行为从一个单位转移到另一个 |
| `CEffectCancelOrder` | 取消单位当前的命令 |
| `CEffectUserData` | 执行由用户数据驱动的分支（自定义数据查表） |
| `CEffectLastTarget` | 复用前一个效果的最终目标 |
| `CEffectUseCalldown` | 触发一次呼叫支援技能 |
| `CEffectLoadContainer` | 把单位装入运输载具 |
| `CEffectRedirectMissile` | 飞行途中改变弹体目标 |
| `CEffectApplyForce` | 推动单位（物理力——例如冲击波使用） |
