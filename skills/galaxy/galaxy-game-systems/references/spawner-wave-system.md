# 刷兵 / 波次系统

按周期性计时器刷出单位，并用排队移动命令把它们依次送往一串路径点。

## 全局变量

```galaxy
// Declare in your GlobalVariables file
point[10] gv_laneSpawnPoint;       // spawn location per lane
point[10][11] gv_laneWaypoints;    // ordered waypoints per lane (up to 11)
int[10] gv_laneWaypointCount;      // number of registered waypoints per lane
int gv_laneCount = 0;
trigger gt_SpawnUnits;
```

## 地图初始化时注册兵线

```galaxy
void WaveSystem_RegisterLane(int lp_lane, point lp_spawn) {
    gv_laneSpawnPoint[lp_lane]    = lp_spawn;
    gv_laneWaypointCount[lp_lane] = 0;
    if (lp_lane > gv_laneCount) { gv_laneCount = lp_lane; }
}

void WaveSystem_AddWaypoint(int lp_lane, point lp_pt) {
    int lv_idx = gv_laneWaypointCount[lp_lane] + 1;
    gv_laneWaypoints[lp_lane][lv_idx] = lp_pt;
    gv_laneWaypointCount[lp_lane] = lv_idx;
}
```

## 示例配置

```galaxy
void MapInit_Waves() {
    WaveSystem_RegisterLane(1, PointCreate(10.0, 50.0));
    WaveSystem_AddWaypoint(1, PointCreate(35.0, 50.0));
    WaveSystem_AddWaypoint(1, PointCreate(70.0, 50.0));
    WaveSystem_AddWaypoint(1, PointCreate(100.0, 50.0));

    gt_SpawnUnits = TriggerCreate("WaveSpawn_Handler");
    TriggerAddEventTimePeriodic(gt_SpawnUnits, 30.0, c_timeGame);
}
```

## 波次刷兵处理函数

```galaxy
bool WaveSpawn_Handler(bool testConds, bool runActions) {
    int  lv_lane;
    int  lv_u;
    int  lv_wp;
    unit lv_unit;

    for (lv_lane = 1; lv_lane <= gv_laneCount; lv_lane += 1) {
        for (lv_u = 1; lv_u <= 5; lv_u += 1) {
            UnitCreate(1, "Zergling", c_unitCreateIgnorePlacement,
                2, gv_laneSpawnPoint[lv_lane], 270.0);
            lv_unit = UnitLastCreated();

            // Queue move orders through each waypoint in sequence
            for (lv_wp = 1; lv_wp <= gv_laneWaypointCount[lv_lane]; lv_wp += 1) {
                UnitIssueOrder(lv_unit,
                    OrderTargetingPoint(
                        AbilityCommand("move", 0),
                        gv_laneWaypoints[lv_lane][lv_wp]
                    ),
                    c_orderQueueAddToEnd  // MUST use AddToEnd — Replace cancels previous waypoints
                );
            }
        }
    }
    return true;
}
```

## 开关刷兵

```galaxy
TriggerEnable(gt_SpawnUnits, true);   // start spawning
TriggerEnable(gt_SpawnUnits, false);  // pause spawning
```
