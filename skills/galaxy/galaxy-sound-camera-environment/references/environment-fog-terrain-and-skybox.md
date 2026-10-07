# 环境 —— 雾、地形与天空盒

雾设置、地形高度、环境可见性、禁飞区、天空盒/背景与昼夜时间。

## 环境 —— 雾

```galaxy
// Enable / disable fog
FogSetEnabled(true);

// Fog settings
FogSetDensity(0.3);
FogSetColor(Color(0.1, 0.1, 0.3));
FogSetFallOff(2.0);
FogSetStartHeight(0.0);
```

## 环境 —— 地形 / 水面

```galaxy
// Terrain height at a point
fixed lv_h = WorldHeight(lv_p);

// Show/hide environment (terrain, water, sky)
EnvironmentShow(lv_player, true);

// Disable/enable no-fly zones dynamically
PathAddNoFlyZone(lv_region);
PathRemoveNoFlyZonesInRegion(lv_region);
```

## 环境 —— 天空盒 / 背景

```galaxy
// Set the skybox (background) for all players
GameSetBackground(c_backgroundFixed, "ShakurasSkyBox", 100.0);
// (mode, name, weight)

// Clear the skybox (restore default)
GameSetBackground(c_backgroundFixed, null, 100.0);

// Time of day (affects lighting/shadows)
GameTimeOfDaySet("08:00:00");
GameTimeOfDayPause(true);   // freeze time of day
GameTimeOfDayPause(false);  // unfreeze
```
