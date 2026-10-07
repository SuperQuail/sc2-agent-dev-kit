# 野怪 / 营地重生系统

中立营地的单位全部死亡后，等待固定时间再在刷新点重建它们。

## 全局变量

```galaxy
// Declare in your GlobalVariables file
point[20]     gv_campSpawn;
string[20]    gv_campUnitType;
int[20]       gv_campUnitCount;
fixed[20]     gv_campRespawnTime;
unitgroup[20] gv_campGroup;
int           gv_campCount = 0;
```

## 地图初始化时注册营地

```galaxy
void Camp_Register(point lp_spawn, string lp_type, int lp_count, fixed lp_respawn) {
    int lv_u;
    gv_campCount += 1;
    int lv_idx = gv_campCount;

    gv_campSpawn[lv_idx]       = lp_spawn;
    gv_campUnitType[lv_idx]    = lp_type;
    gv_campUnitCount[lv_idx]   = lp_count;
    gv_campRespawnTime[lv_idx] = lp_respawn;
    gv_campGroup[lv_idx]       = UnitGroupEmpty();

    // Spawn initial units
    for (lv_u = 1; lv_u <= lp_count; lv_u += 1) {
        UnitCreate(1, lp_type, c_unitCreateIgnorePlacement,
            c_playerNeutralHostile, lp_spawn, 270.0);
        UnitGroupAdd(gv_campGroup[lv_idx], UnitLastCreated());
    }

    trigger lv_t = TriggerCreate("Camp_OnDeath");
    TriggerAddEventUnitDied(lv_t, null);
}
```

## 共享的死亡 / 重生处理函数

```galaxy
bool Camp_OnDeath(bool testConds, bool runActions) {
    unit lv_dead = EventUnit();
    int  lv_camp;
    int  lv_u;

    for (lv_camp = 1; lv_camp <= gv_campCount; lv_camp += 1) {
        if (!UnitGroupHasUnit(gv_campGroup[lv_camp], lv_dead)) { continue; }

        UnitGroupRemove(gv_campGroup[lv_camp], lv_dead);

        // Only respawn once the whole camp is cleared
        if (UnitGroupCount(gv_campGroup[lv_camp], c_unitCountAlive) > 0) { return true; }

        Wait(gv_campRespawnTime[lv_camp], c_timeGame);

        gv_campGroup[lv_camp] = UnitGroupEmpty();
        for (lv_u = 1; lv_u <= gv_campUnitCount[lv_camp]; lv_u += 1) {
            UnitCreate(1, gv_campUnitType[lv_camp], c_unitCreateIgnorePlacement,
                c_playerNeutralHostile, gv_campSpawn[lv_camp], 270.0);
            UnitGroupAdd(gv_campGroup[lv_camp], UnitLastCreated());
        }
        return true;
    }
    return true;
}
```
