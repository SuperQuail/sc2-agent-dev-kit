# 单位创建、检测与生命周期

创建标志、存活检测、移除语义，以及位置/朝向控制。

## 创建单位

```galaxy
// Basic create
UnitCreate(1, "Marine", c_unitCreateIgnorePlacement, 1, lv_point, 270.0);
unit lv_marine = UnitLastCreated();

// Via library helpers (common variants)
libNtve_gf_CreateUnitsAtPoint2(1, "Marine", 1, lv_point);
libNtve_gf_UnitCreateFacingPoint(1, "Marine", 0, 1, lv_point, lv_facingPoint);

// UnitCreate flags
c_unitCreateIgnorePlacement    // ignore placement rules (most common)
c_unitCreateUsePreferences     // use placement preferences
```

## 检测单位

```galaxy
bool lv_alive   = UnitIsAlive(lv_unit);
bool lv_valid   = UnitIsValid(lv_unit);
bool lv_dead    = !UnitIsAlive(lv_unit);
bool lv_type    = (UnitGetType(lv_unit) == "Marine");
int  lv_owner   = UnitGetOwner(lv_unit);
```

## 销毁 / 移除单位

```galaxy
UnitKill(lv_unit);          // kill instantly (no kill-credit/XP awarded to any attacker)
UnitRemove(lv_unit);        // remove instantly with no death animation or events
UnitPauseAll(bool);         // pause or unpause ALL units on the map simultaneously
```

## 单位位置与朝向

```galaxy
point lv_pos = UnitGetPosition(lv_unit);
UnitSetPosition(lv_unit, lv_newPos, false);          // false = no smooth move
UnitSetFacing(lv_unit, 90.0, 0.0);                   // angle in degrees, duration 0
fixed lv_facing = UnitGetFacing(lv_unit);
```
