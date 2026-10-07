# 单位状态、工具函数与选中

最近单位查找、所有权转移、状态标志、libNtve 包装、皮肤变体与选中控制。

## 单位工具函数

```galaxy
// Find nearest unit
unit lv_near = UnitNearestUnit(lv_point, lv_group);

// Default facing from one position to another
fixed lv_angle = AngleBetweenPoints(lv_from, lv_to);

// Give unit to a different player
UnitSetOwner(lv_unit, lv_newPlayer, true);   // true = change color

// Unit state flags
UnitSetState(lv_unit, c_unitStateInvulnerable,  true);
UnitSetState(lv_unit, c_unitStateHidden,        true);
UnitSetState(lv_unit, c_unitStateSelectable,    false);  // prevent selection by players
UnitSetState(lv_unit, c_unitStateStatusBar,     false);  // hide HP bar
UnitSetState(lv_unit, c_unitStateTooltipable,   true);   // allow tooltip on hover

// Common state constants
c_unitStateInvulnerable
c_unitStateHidden
c_unitStatePaused
c_unitStateIsHallucination
c_unitStateSelectable
c_unitStateStatusBar
c_unitStateTooltipable

// libNtve helper wrappers (shortcuts for common state combos)
libNtve_gf_MakeUnitInvulnerable(lv_unit, true);    // sets invulnerable
libNtve_gf_MakeUnitUncommandable(lv_unit, true);   // prevents player from commanding the unit
libNtve_gf_ShowHideUnit(lv_unit, false);           // hide (sets hidden + removes from selection)

// Change unit skin / variation (e.g. aged model, damaged state)
libNtve_gf_UnitSetVariation(lv_unit, "Marine", 1, "Battle");
// (unit, typeName, variationIndex, variationTag)

// Selection control
UnitFlashSelection(lv_unit, 2.0);      // flash selection ring for duration (seconds)
UnitSelect(lv_unit, lv_player, true);  // add to player's current selection
UnitClearSelection(lv_player);         // deselect all units for player
libNtve_gf_StoreUnitSelection(lv_player, libNtve_ge_UnitSelectionStoreOption_ClearUnitSelection);
libNtve_gf_RestoreUnitSelection(lv_player);  // restore previously stored selection
```
