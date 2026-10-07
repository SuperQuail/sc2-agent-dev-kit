# 近战 AI 初始化

给使用标准 SC2 近战 AI 的 RTS 玩家：

```galaxy
// Place starting units (workers + command center/hatchery/nexus)
MeleeInitUnitsForPlayer(lv_player, "Terr", lv_startPoint);
MeleeInitUnitsForPlayer(lv_player, "Zerg", lv_startPoint);
MeleeInitUnitsForPlayer(lv_player, "Prot", lv_startPoint);

// Give starting resources (minerals/gas)
MeleeInitResourcesForPlayer(lv_player, PlayerRace(lv_player));

// Start the AI
AIStart(lv_player, "AI\Terran.SC2AIData", false, false, false, false);

// Variant that applies to all computer players
MeleeInitAI();
```
