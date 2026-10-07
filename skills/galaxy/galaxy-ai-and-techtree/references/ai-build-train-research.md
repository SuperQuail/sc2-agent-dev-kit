# AI 建造 / 训练 / 研究（脚本级）

```galaxy
// Queue a building
AIBuild(lv_player, "CommandCenter", lv_town, 1, true);
AITrain(lv_player, "Marine", lv_town, 1, true);
AIResearch(lv_player, "MarineRange", lv_town);

// Stock control (maintain a count of a unit type)
AISetStock(lv_player, "Marine", 8);
AISetStockEx(lv_player, "Tank", 4, c_townAny, true, true);

// Clear queues
AIClearBuildQueue(lv_player);
AIClearTrainQueue(lv_player);
```
