# 菌毯与小地图标记

查询与修改虫族菌毯，以及在某个点创建/销毁小地图标记。

## 菌毯

```galaxy
// Check if Zerg creep is present at a point
bool lv_hasCreep = CreepIsPresent(lv_point);

// Add or remove creep in a radius
CreepModify(lv_point, 5.0, true,  false);  // add creep (spread = false)
CreepModify(lv_point, 5.0, false, false);  // remove creep
```

---

## 小地图标记

```galaxy
PingCreate(PlayerGroupAll(), lv_point, 5.0, ColorWithAlpha(1.0, 0.0, 0.0, 1.0), "");
ping lv_ping = PingLastCreated();
PingSetDuration(lv_ping, 8.0);
PingDestroy(lv_ping);
PingDestroyAll();
```
