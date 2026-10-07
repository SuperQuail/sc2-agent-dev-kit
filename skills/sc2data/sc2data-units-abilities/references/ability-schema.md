# 技能 schema（AbilData.xml）

完整的 CAbil* 子类型清单、常用技能类型的工作示例，以及技能字段参考（Set ID、冷却、充能、标志）。

## 技能（`AbilData.xml`）

文件：`Base.SC2Data/GameData/AbilData.xml`

### 指定目标的技能模式

```xml
<CAbilEffectTarget id="MyBlast">
    <EditorCategories value="Race:Terran,AbilityorEffectType:Units"/>
    <Effect index="0" value="MyBlastEffect"/>       <!-- which effect to fire -->
    <Cost index="0">
        <Vital index="Energy" value="75"/>
        <Cooldown TimeStart="0" TimeUse="12"/>
    </Cost>
    <Range value="7"/>
    <Arc value="360"/>
    <CursorEffect value="MyBlastEffect"/>
    <PrepTime index="0" value="0.5"/>               <!-- cast time (seconds) -->
    <CmdButtonArray index="Execute" DefaultButtonFace="MyBlastButton"/>
    <TargetFilters index="0" value="Visible;Self,Ally,Neutral,Invulnerable,Dead"/>
</CAbilEffectTarget>
```

### 瞬发（无目标）

```xml
<CAbilEffectInstant id="MyShield">
    <Cost index="0">
        <Cooldown TimeUse="40"/>
    </Cost>
    <Effect index="0" value="MyShieldEffect"/>
    <CmdButtonArray index="Execute" DefaultButtonFace="MyShieldButton"/>
</CAbilEffectInstant>
```

### 研究技能

```xml
<CAbilResearch id="MyBuildingResearch">
    <InfoArray index="Research1" Time="45">
        <Effect value="ResearchMyTech"/>
    </InfoArray>
    <InfoArray index="Research2" Time="60">
        <Effect value="ResearchMyTech2"/>
    </InfoArray>
</CAbilResearch>
```

## 技能子类型参考

| XML 类型 | 使用场景 |
|---|---|
| `CAbilEffectTarget` | 指定目标——有距离与角度，对点或单位触发效果 |
| `CAbilEffectInstant` | 无目标瞬发（自我增益、被动触发） |
| `CAbilBehavior` | 开关一个行为 |
| `CAbilBuild` | 建造建筑（工人的按钮） |
| `CAbilTrain` | 在建筑处训练单位 |
| `CAbilResearch` | 在建筑处研究升级 |
| `CAbilMorph` | 把施法单位变形为另一单位类型 |
| `CAbilMorphPlacement` | 需要放置目标的变形（如兵营 → 升空） |
| `CAbilHarvest` | 采集资源（工人采矿） |
| `CAbilInteract` | 与目标交互（如修理） |
| `CAbilInventory` | 管理物品栏 |
| `CAbilLearn` | 花费经验/点数学习技能（英雄天赋） |
| `CAbilMerge` | 把多个单位合并为一个 |
| `CAbilMove` | 覆写移动命令 |
| `CAbilQueue` | 排队子技能（用于多阶段技能） |
| `CAbilRally` | 设置集结点 |
| `CAbilRevive` | 复活已死亡的英雄 |
| `CAbilSpecialize` | 英雄专精/天赋选择 |
| `CAbilStop` | 停止命令 |
| `CAbilTransport` | 运输载具的装卸 |
| `CAbilWarpTrain` | 通过折跃门机制训练 |
| `CAbilArmMagazine` | 先装填再发射（如核弹：先架设，再单独发射） |

## 技能字段参考

### Set ID — 共享指令按钮

`Set ID` 是跨多个技能共享的字符串。当玩家下达命令，或把单位加入带有某 `Set ID` 的选中集合时，该单位上**所有具有相同 Set ID 的技能**都会响应。用于 `BurrowDown`/`BurrowUp`，这样按住 `Burrow` 键会下达给正确的变形技能：

```xml
<CAbilMorph id="BurrowDown_Zergling">
    <CmdButtonArray index="Execute" DefaultButtonFace="Burrow" SetId="BrwD"/>
</CAbilMorph>
<CAbilMorph id="BurrowDown_Roach">
    <CmdButtonArray index="Execute" DefaultButtonFace="Burrow" SetId="BrwD"/>
</CAbilMorph>
```

### 冷却范围（Cooldown Location）

控制冷却在哪个范围内共享：

| 取值 | 含义 |
|---|---|
| `Ability` | 默认——冷却按本单位的该技能实例独立计算 |
| `Unit` | 同一单位的所有变形形态共享 |
| `Player` | 该玩家的所有单位共享一个冷却（按玩家全局） |
| `Global` | 所有玩家共享一个冷却（完全全局） |

### 冷却操作（Cooldown Operation）

动态的冷却修改（例如来自升级或行为）如何生效：

| 取值 | 效果 |
|---|---|
| `Add` | 加到剩余冷却上 |
| `Add if Not In Cooldown` | 仅当技能当前不在冷却中时才加 |
| `Max` | 取当前值与新值中的较大者 |
| `Min` | 取较小者 |
| `Multiply` | 乘以剩余冷却 |
| `Set` | 直接覆写剩余冷却 |

### 充能系统

带充能的技能可以在触发冷却前多次使用：

| 字段 | 用途 |
|---|---|
| `Count Max` | 最大充能层数 |
| `Count Start` | 游戏开始时的可用充能 |
| `Count Use` | 每次使用消耗的充能（通常为 1） |
| `Time Start` | 初始回复时间 |
| `Time Use` | 每回复一层充能的时间 |
| `Time Delay` | 充能耗尽后到开始回复之间的等待 |
| `Hide Count` | 在界面上隐藏充能计数 |

### State Behavior

`State Behavior` 把某个行为与技能的生命周期绑定——技能状态变化（创建/销毁/启用/禁用）时创建该行为。适合用来做镜像技能可用性的被动效果。

### 共享标志（Shared Flags）

| 标志 | 效果 |
|---|---|
| `Disable While Dead` | 单位死亡时自动禁用技能 |
| `Disabled` | 默认以禁用状态开始（用效果或脚本启用） |
| `Register Charge Event` | 允许充能状态变化触发 Actor 事件 |
| `Register Cooldown Event` | 允许冷却状态变化触发 Actor 事件 |
| `Skip Preload` | 不预载技能资源 |
| `Snap Target To Unit Radius` | 强制把目标吸附到目标单位半径的边缘 |

### Veterancy Level Min / Skip

用于英雄技能的门控或跳级：

- `Veterancy Level Min` — 该技能可用的最低等级（用于分级英雄技能）
- `Veterancy Level Skip` — 若升级跳过了该等级（例如来自额外经验），行为在此定义

### Refund Fraction

`-1` 表示：技能在其「Refundable Stage」之前被取消（例如建造中途取消）时，全额退还资源。
