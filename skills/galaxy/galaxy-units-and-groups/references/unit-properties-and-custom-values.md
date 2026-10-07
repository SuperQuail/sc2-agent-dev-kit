# 单位属性、自定义值、装载量与缩放

属性读写配对、四个自定义定点数据槽、装载量查询与模型缩放。

## 单位属性

```galaxy
// Reading
fixed lv_life    = UnitGetPropertyFixed(lv_unit, c_unitPropLife, true);       // true = current
fixed lv_maxLife = UnitGetPropertyFixed(lv_unit, c_unitPropLife, false);      // false = max
fixed lv_energy  = UnitGetPropertyFixed(lv_unit, c_unitPropEnergy, true);
fixed lv_shield  = UnitGetPropertyFixed(lv_unit, c_unitPropShields, true);

// Writing
UnitSetPropertyFixed(lv_unit, c_unitPropLife,     75.0);
UnitSetPropertyFixed(lv_unit, c_unitPropShields, 100.0);

// Property constants
c_unitPropLife           // current HP
c_unitPropLifeMax        // max HP
c_unitPropLifePercent    // HP as percent 0-100
c_unitPropShields        // current shields
c_unitPropShieldsMax     // max shields
c_unitPropEnergy         // current energy
c_unitPropEnergyMax      // max energy
c_unitPropEnergyPercent  // energy as percent 0-100 (writable)
c_unitPropKills          // kill count
c_unitPropMoveSpeed      // movement speed
c_unitPropArmorLevel     // armor rating
c_unitPropCurrent        // attack damage (general current)
```

## 单位自定义值（数据槽）

每个单位有 4 个自定义定点数据槽（下标 0–3），用于存任意数值：

```galaxy
// Store a float in slot 0 (e.g. spawn time, special flag, current state)
UnitSetCustomValue(lv_unit, 0, 2.5);

// Retrieve the stored value
fixed lv_val = UnitGetCustomValue(lv_unit, 0);

// Slots 0-3 are independent; default value is 0.0
```

## 装载量

```galaxy
// Count units currently loaded in a transport
int lv_count = UnitCargoValue(lv_transport, c_unitCargoUnitCount);
```

## 缩放与外观

```galaxy
UnitSetScale(lv_unit, 1.5, 1.5, 1.5);      // x, y, z scale multipliers
```
