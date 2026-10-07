# 战役空投舱与剧情状态

## 空投舱（战役）

`libCamp_gf_CreateDropPod` 负责整个空投动作：生成舱体、弹出单位、播放效果（Effect）。

```galaxy
// Build the group of units that will drop
unitgroup lv_dropGroup = UnitGroupEmpty();
libNtve_gf_CreateUnitsAtPoint2(4, "Zergling", lv_enemyPlayer, lv_dropPoint);
UnitGroupAdd(lv_dropGroup, UnitLastCreated());

// Zerg drop pod
libCamp_gf_CreateDropPod(libCamp_ge_DropPodRace_Zerg, lv_dropPoint, lv_dropGroup, false);

// Terran drop pod
libCamp_gf_CreateDropPod(libCamp_ge_DropPodRace_Terran, lv_dropPoint, lv_dropGroup, true);
```

## 战役科技与剧情状态

```galaxy
// Enable or disable a campaign tech unit for a player
libCamp_gf_EnableCampaignTechUnit(true, libCamp_ge_StoryTechGroup_Marine, lv_player);

// Read or write a story state variable (mission-to-mission carry-over)
// Story state values are integers stored in the campaign save
int lv_val = libCamp_gf_StoryState(libCamp_ge_StoryStateID_SomeFlag);
libCamp_gf_SetStoryState(libCamp_ge_StoryStateID_SomeFlag, 1);

// Common merc purchase state check
bool lv_hired = (libCamp_gf_StoryState(libCamp_ge_StoryStateID_SomeMercGroup) == libCamp_ge_StoryMercStatus_Purchased);
```
