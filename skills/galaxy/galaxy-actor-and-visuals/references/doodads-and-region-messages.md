# Doodad 与整区域 Actor 消息

显示/隐藏/移除 doodad 与死亡模型，以及向整个区域广播 actor 消息。

## Doodad

```galaxy
// Show/hide doodads by region — (bool showHide, region, string doodadType)
// doodadType is the doodad type string; empty string = all doodads
libNtve_gf_ShowHideDoodadsInRegion(true,  lv_region, "");  // show all doodads
libNtve_gf_ShowHideDoodadsInRegion(false, lv_region, "SomeDoodadType");  // hide specific type
// NOTE: NO playergroup parameter — this affects doodads globally on the map

// Remove doodads by type in region
libNtve_gf_RemoveDoodadsinRegion(lv_region, "");  // (region, doodadType)

// Remove death models from a region (cleanup)
libNtve_gf_RemoveDeathModelsinRegion(lv_region);
libNtve_gf_RemoveDeathModelsinRegionImmediately(lv_region);

// Send actor message to all actors in a game region
libNtve_gf_SendActorMessageToGameRegion(lv_region, "Destroy");
// With class/term filters:
libNtve_gf_SendActorMessageToGameRegionWithFilters(
    lv_region,
    c_actorIntersectAgainstCenter,  // intersect type
    "Destroy",                      // message
    "",                             // class filters
    ""                              // terms
);
```
