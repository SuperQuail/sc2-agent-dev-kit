# UI 警报与小地图 Ping

开关内置警报类型、简单 ping，以及作用在单位与点位上的目标信标/ping。

## UI 警报与小地图

```galaxy
// Toggle built-in alert types
UISetAlertTypeVisible(PlayerGroupAll(), "AlertWorkerAttacked", false);

// Minimap ping
PingCreate(PlayerGroupAll(), lv_point, 10.0, ColorWithAlpha(255, 0, 0, 255), "");
```

## UI 警报与目标 Ping

```galaxy
// Beacon / alert for a map point (flashes minimap + compass)
UIAlertPoint("TriggerName", PlayerGroupSingle(lv_player), StringExternal("Param/Alert/..."), null, lv_point);

// Beacon / alert centered on a specific unit
UIAlertUnit("TriggerName", lv_player, StringExternal("Param/Alert/..."), null, lv_unit);

// Minimap ping with facing angle (used for objectives)
libNtve_gf_CreatePingFacingAngle(
    PlayerGroupAll(),
    "PingObjective",                   // ping type from data
    lv_point,
    ColorWithAlpha(0, 100, 0, 0),      // RGBA color (0-255 or 0-1 depending on version)
    0.0,                               // duration (0 = permanent until destroyed)
    270.0                              // facing angle
);
ping lv_ping = PingLastCreated();
PingSetScale(lv_ping, 0.75);
PingSetTooltip(lv_ping, StringToText("Objective location"));
```
