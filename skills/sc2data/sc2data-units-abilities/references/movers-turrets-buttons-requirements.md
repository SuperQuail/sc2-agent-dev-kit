# Mover、Turret、Button 与 Requirement

CMover、CTurret、CButton、CRequirement / CRequirementNode 的 schema 与示例。

## Mover（`MoverData.xml`）

文件：`Base.SC2Data/GameData/MoverData.xml`

| XML 类型 | 用途 |
|---|---|
| `CMoverMissile` | 弹体运动（追踪、弹道、直线） |
| `CMoverAvoid` | 地面单位避让移动 |
| `CMoverBallistic` | 抛物线弹体 |

```xml
<CMoverMissile id="MyMissile">
    <MotionPhases index="0" MaxSpeed="10"/>   <!-- max speed of the missile -->
</CMoverMissile>

<CMoverAvoid id="Ground2">
    <!-- Ground movement with collision avoidance -->
</CMoverAvoid>
```

## Turret（`TurretData.xml`）

```xml
<CTurret id="MyTurret">
    <YawArc value="360"/>      <!-- rotation arc in degrees (360 = any direction) -->
    <YawRate value="999"/>     <!-- rotation speed (degrees/sec) -->
</CTurret>
```

## Button（`ButtonData.xml`）

文件：`Base.SC2Data/GameData/ButtonData.xml`  
按钮就是指令面板的图标 + 提示。每个技能的 `CmdButtonArray` 按 id 引用按钮。

```xml
<CButton id="MyAbilityButton">
    <Icon value="Assets\Textures\btn-ability-myability.dds"/>   <!-- 76×76 .dds -->
    <Tooltip value="Abil/Tooltip/MyAbilityButton"/>              <!-- GameStrings key -->
    <Name value="Abil/Name/MyAbilityButton"/>
    <Hotkey value="W"/>
</CButton>
```

按钮图标命名约定：`btn-` 前缀，76×76 像素，`.dds` 格式。

## Requirement（`RequirementData.xml` / `RequirementNodeData.xml`）

需求把技能、单位或研究挡在条件之后。

```xml
<!-- A compound AND requirement -->
<CRequirement id="HaveBarracksAndArmory">
    <NodeArray index="0" value="CountUnitBarracksCompleteOnly"/>
    <NodeArray index="1" value="CountUnitArmoryCompleteOnly"/>
</CRequirement>

<!-- A node that counts completed buildings -->
<CRequirementCountUnit id="CountUnitBarracksCompleteOnly">
    <Link value="Barracks"/>
    <State value="Complete"/>
</CRequirementCountUnit>
```
