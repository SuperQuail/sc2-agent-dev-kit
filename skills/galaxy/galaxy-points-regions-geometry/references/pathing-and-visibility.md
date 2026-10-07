# 寻路与可见性

跨悬崖判定、寻路类型常量，以及战争迷雾 / 揭示者 API。

## 寻路

```galaxy
// Does a line cross a cliff edge?
bool lv_cross = CrossCliff(lv_from, lv_to);

// Pathing type constants (returned by PathingType)
c_pathingTypeAny
c_pathingTypeGround
c_pathingTypeAir
c_pathingTypeWalkable
```

---

## 可见性

```galaxy
// Reveal area for a player (removes FoW)
VisRevealArea(lv_player, lv_region, 0.0, false);

// Explore (permanent reveal without sight)
VisExploreArea(lv_player, lv_region, true, false);

// Check if a point is currently visible
bool lv_vis = VisIsVisibleForPlayer(lv_player, lv_point);

// Enable/disable FoW
VisEnable(lv_player, false);  // disable fog of war for player

// FoW alpha (0 = fully transparent)
VisSetFoWAlpha(0.5);
VisResetFoWAlpha();

// Revealer (persistent vision source at a point)
VisRevealerCreate(lv_player, lv_point, 12.0);
revealer lv_rev = VisRevealerLastCreated();
VisRevealerEnable(lv_rev, true);   // enable/disable without destroying
VisRevealerDestroy(lv_rev);
```
