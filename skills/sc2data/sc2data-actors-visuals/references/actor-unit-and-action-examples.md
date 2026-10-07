# Actor 文件、CActorUnit 与 CActorAction

Actor 条目放在哪里、带完整注释的 CActorUnit、武器动作序列，以及可继承的 Actor 父类型。

> **文件放置：** 为新单位创建 Actor 时，放进该单位自己的独立 XML 文件（`GameData/Faction/UnitName/UnitName.xml`），不要塞进单体式的 `ActorData.xml`。在 `GameData.xml` 中用 `<Catalog path="..."/>` 注册该文件。完整文件模板见 sc2data-units-abilities 技能。

## Actor 文件

文件：`Base.SC2Data/GameData/ActorData.xml`

所有 Actor 条目都是 `<Catalog>` 的子元素：

```xml
<?xml version="1.0" encoding="utf-8"?>
<Catalog>
    <CActorUnit id="MyUnitActor" parent="GenericUnitStandard" unitName="MyUnit">
        ...
    </CActorUnit>
</Catalog>
```

## CActorUnit — 完整示例

```xml
<CActorUnit id="MyUnitActor" parent="GenericUnitStandard" unitName="MyUnit">
    <!-- Cross-actor messaging alias (e.g. _UnitMedium for medium-sized units) -->
    <Aliases value="_UnitMedium"/>

    <!-- Reusable event macro (defined elsewhere, expands into multiple On entries) -->
    <Macros value="StandardDeathMacro"/>

    <!-- Primary model -->
    <Model value="MyUnitModel"/>
    <AnimBlendTime value="0.300000"/>

    <!-- Construction model shown while building -->
    <BuildModel value="MyUnitBuildModel"/>

    <!-- Portrait model for unit info panel -->
    <PortraitModel value="MyUnitPortrait"/>

    <!-- Placement ghost model -->
    <PlacementModel value="MyUnitPlacement"/>

    <!-- Death variations -->
    <DeathArray index="Normal" ModelLink="MyDeathModel" SoundLink="MyDeathSound" VoiceLink=""/>
    <DeathArray index="Blast" ModelLink="MyBlastDeathModel"/>

    <!-- Unit sounds -->
    <SoundArray index="Ready" value="MyUnit_Ready"/>
    <SoundArray index="What" value="MyUnit_What"/>
    <SoundArray index="Yes" value="MyUnit_Yes"/>
    <SoundArray index="Attack" value="MyUnit_Attack"/>
    <SoundArray index="Pissed" value="MyUnit_Pissed"/>

    <!-- UI icons -->
    <GroupIcon><Image value="Assets\Textures\wireframe-myunit.dds"/></GroupIcon>
    <UnitIcon value="Assets\Textures\btn-unit-myunit.dds"/>
    <HighlightTooltip value="Unit/Tooltip/MyUnit"/>

    <!-- Health bar positioning -->
    <BarOffset value="25"/>
    <BarWidth value="90"/>

    <!-- Actor events -->
    <On Terms="ActorCreation" Send="AnimPlay Birth"/>
    <On Terms="UnitBirth.MyUnit" Send="Create MyBirthEffectActor"/>
    <On Terms="WeaponStart.MyWeapon.AttackStart" Send="AnimBracketStart Attack Attack"/>
    <On Terms="WeaponStop.MyWeapon.AttackStop" Send="AnimBracketStop Attack"/>
    <On Terms="Abil.MyAbility.SourcePrepStart" Send="AnimPlay Spell"/>
    <On Terms="Upgrade.MyUpgrade.Add" Send="AnimGroupApply Superior"/>
    <On Terms="UnitDeathCustomize; IsStatus DeathSuicide 0" Send="Create MyDeathSound"/>
    <On Terms="ActorOrphan" Send="Destroy"/>
</CActorUnit>
```

## CActorAction — 武器攻击序列

配合 `GenericAttack` 父类型使用。三个 token 是 **Attack**、**Launch**、**Impact**：

```xml
<CActorAction id="MyWeaponAction" parent="GenericAttack">
    <!-- Attack: play windup animation on the unit actor -->
    <On Terms="ActionStart" Send="AnimPlay Attack ::Creator"/>

    <!-- Launch: spawn projectile model at weapon launch point -->
    <On Terms="ActionLaunch" Send="Create MyProjectileModel ::Creator"/>

    <!-- Impact: spawn explosion model at target -->
    <On Terms="ActionImpact" Send="Create MyImpactModel ::Target"/>
</CActorAction>
```

## Actor 父类型参考

| 父类型 | 提供什么 |
|---|---|
| `GenericUnitStandard` | 默认单位 Actor——选中圈、血条、标准事件 |
| `GenericAttack` | 武器动作 Actor——Attack/Launch/Impact 事件 token |
| `ModelAddition` | 把模型依附到另一个 Actor（增益视觉、挂件） |
| `ModelAnimationStyleContinuous` | 循环模型（灵能风暴这类持续范围效果） |
| `ModelAnimationStyleOneShot` | 一次性模型（爆炸、出生特效） |
| `BehaviorGlaze` | 行为生效期间施加釉层（叠加层）；使用 `buff` token |
| `SoundOneShot` | 播一次声音后自动销毁 |
| `SoundContinuous` | 持续播放直到被销毁 |
| `Range Abil` | 技能的射程指示圈 |
| `Range Behavior` | 行为/光环的射程指示圈 |
| `Range Weapon` | 武器的射程指示圈 |
| `Cursor Splat` | 技能瞄准时显示的 AoE 目标光标 |

> **警告：** 通过编辑器界面使用 `Range *` 或 `Cursor Splat` 父类型时，编辑器会自动生成默认 `<On` 事件。创建后请在 Events 字段上执行 **Reset To Parent Value**，或者在 XML 视图里手工删除生成的 `<On` 行，以免事件接线重复或损坏。
