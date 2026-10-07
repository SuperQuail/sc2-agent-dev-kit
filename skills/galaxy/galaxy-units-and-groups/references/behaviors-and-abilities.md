# 行为与技能

套用升级、增益与减益，以及控制单位技能和技能命令。

## 行为（Behavior）

行为是套用升级、增益与减益的主要机制：

```galaxy
// Add a behavior (with stack count)
UnitBehaviorAdd(lv_unit, "PowerOverwhelming", lv_unit, 1);
UnitBehaviorAdd(lv_unit, "ArmorPlatesAura",   lv_unit, 1);

// Remove a behavior
UnitBehaviorRemove(lv_unit, "PowerOverwhelming", 1);

// Set behavior count
UnitBehaviorSetCount(lv_unit, "ShieldBoost", lv_unit, 2);

// Query
int  lv_count   = UnitBehaviorCount(lv_unit, "PowerOverwhelming");
bool lv_hasBhvr = UnitHasBehavior(lv_unit, "PowerOverwhelming");
```

## 技能（Ability）

```galaxy
// Reset cooldown of an ability
UnitAbilityReset(lv_unit, "Ability_Name", true);

// Check if unit can use ability
bool lv_can = UnitAbilityEnabled(lv_unit, "Ability_Name");

// Set ability enabled
UnitAbilityEnable(lv_unit, "Ability_Name", true);

// Order a unit to use ability at a point
UnitOrder(lv_unit, OrderTargetingPoint(AbilityCommand("Ability_Name", 0), lv_point));
```
