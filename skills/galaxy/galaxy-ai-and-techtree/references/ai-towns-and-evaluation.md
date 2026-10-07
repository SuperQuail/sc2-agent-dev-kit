# AI 城镇与评估

## AI 城镇（进阶）

```galaxy
// Get number of mineral/gas spots in a town
int lv_minerals = AIGetMineralNumSpots(lv_player, lv_town);
int lv_gas      = AIGetRawGasNumSpots(lv_player, lv_town);

// Get a town's gathering/defense locations
point lv_gather  = AIGetGatherLocation(lv_player, lv_town);
point lv_defense = AIGetGatherDefLocation(lv_player, lv_town);

// Set default economy behavior
AIDefaultEconomy(lv_player);
AIDefaultExpansion(lv_player);
```

## AI 评估

```galaxy
// Compare relative strengths
int lv_ratio = AIEvalRatio(lv_player1, lv_player2);
int lv_wRatio = AIWaveEvalRatio(lv_wave, lv_targetPlayer);

// Get best attack target
point lv_target = AIGetBestTarget(lv_player, lv_town, c_aiAttackWaveGround);
```
