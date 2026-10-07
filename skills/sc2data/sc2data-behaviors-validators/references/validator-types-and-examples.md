# 校验器类型与示例

校验器的存放位置、命名约定、全部 CValidator* 类型，以及每个常用类型的工作示例。

## 校验器（`ValidatorData.xml`）

文件：`Base.SC2Data/GameData/ValidatorData.xml`

校验器返回 **真或假**。使用位置：

- **效果** — 假 = 效果不施加
- **行为** `Disable` — 假 = 条件不满足期间行为被禁用（休眠）
- **行为** `Remove` — 假 = 行为被永久移除
- **技能** — 假 = 技能不可用 / 置灰
- **Actor** `Terms` — 通过 `ValidateUnit UpgradeId` 项

### 命名约定

命名为描述**真条件**的句子片段：

```
"CasterNotBurrowed"       — true when caster is not burrowed
"TargetIsAlive"           — true when target is alive
"IsPhoenix"               — true when unit type is Phoenix
"PlayerHasBarracks"       — true when player owns at least one Barracks
```

### 校验器类型

| XML 类型 | 检测内容 |
|---|---|
| `CValidatorUnitType` | 单位是否为指定类型？ |
| `CValidatorUnitOrder` | 单位是否正在执行指定命令/技能？ |
| `CValidatorUnitComparison` | 把单位命值/数值与阈值比较 |
| `CValidatorPlayerComparison` | 把玩家资源与阈值比较 |
| `CValidatorCombine` | 其他校验器的 AND / OR / NOT |
| `CValidatorPlayerRequirement` | 玩家是否满足科技需求？ |
| `CValidatorLocationPathable` | 某位置是否可通行？ |
| `CValidatorConditionCustom` | Galaxy 脚本自定义条件（高级） |

### CValidatorUnitType — 检查单位类型

```xml
<CValidatorUnitType id="IsZergling">
    <UnitType value="Zergling"/>
    <!-- Negate field not set = true when IS Zergling -->
</CValidatorUnitType>

<CValidatorUnitType id="IsNotHero">
    <UnitType value="HeroBase"/>
    <Negate value="1"/>   <!-- true when unit is NOT a hero -->
</CValidatorUnitType>
```

### CValidatorUnitComparison — 比较单位数值

```xml
<CValidatorUnitComparison id="TargetBelowHalfHP">
    <WhichUnit value="Target"/>
    <Value index="0" value="Life"/>
    <Value index="1" value="LifeMax"/>
    <Fraction value="0.5"/>         <!-- 50% threshold -->
    <Compare value="LT"/>           <!-- Less Than: life < 50% max -->
</CValidatorUnitComparison>
```

比较运算符：`LT`（小于）、`LE`（≤）、`EQ`（等于）、`GE`（≥）、`GT`（大于）、`NE`（不等于）。

### CValidatorPlayerComparison — 检查玩家资源

```xml
<CValidatorPlayerComparison id="PlayerHasEnoughMinerals">
    <Value index="0" value="Minerals"/>
    <Compare value="GE"/>
    <Threshold value="100"/>
</CValidatorPlayerComparison>
```

### CValidatorCombine — 布尔逻辑

```xml
<!-- AND: both must be true -->
<CValidatorCombine id="TargetAliveAndNotShielded">
    <Type value="And"/>
    <ValidatorArray index="0" value="TargetIsAlive"/>
    <ValidatorArray index="1" value="IsNotShielded"/>
</CValidatorCombine>

<!-- OR: at least one must be true -->
<CValidatorCombine id="IsGroundOrStructure">
    <Type value="Or"/>
    <ValidatorArray index="0" value="IsGround"/>
    <ValidatorArray index="1" value="IsStructure"/>
</CValidatorCombine>

<!-- NOT: negate a validator -->
<CValidatorCombine id="NotCloaked">
    <Type value="Not"/>
    <ValidatorArray index="0" value="IsCloaked"/>
</CValidatorCombine>
```

### CValidatorUnitOrder — 检查单位是否正在执行某命令

```xml
<CValidatorUnitOrder id="TargetIsAttacking">
    <WhichUnit value="Target"/>
    <Abil value="Attack"/>
</CValidatorUnitOrder>
```
