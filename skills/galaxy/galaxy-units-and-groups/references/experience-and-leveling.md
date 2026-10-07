# 经验与升级

经验授予与查询、等级读取，以及升级事件。

## 经验 / 等级系统

```galaxy
// Add XP
UnitXPAddXP(lv_unit, 200.0);

// Query XP
fixed lv_xp    = UnitXPGetCurrentXP(lv_unit);
int   lv_level = UnitXPGetCurrentLevel(lv_unit);
int   lv_level2 = UnitLevel(lv_unit);   // same result, shorter

// Event: listen for level-up
TriggerAddEventUnitGainLevel(myTrigger, null);   // null = any unit
// Inside handler:
unit lv_leveledUnit = EventUnit();
int  lv_newLevel    = UnitXPGetCurrentLevel(lv_leveledUnit);
```
