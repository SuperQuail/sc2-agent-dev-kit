# 近战初始化、科技树升级与限制

## 科技树升级

```galaxy
// Add an upgrade level to a player's tech tree
TechTreeUpgradeAddLevel(lv_player, "UpgradeName", 1);   // +1 level
TechTreeUpgradeAddLevel(lv_player, "UpgradeName", -1);  // -1 level (downgrade)

// Set level to a specific value
TechTreeUpgradeSetLevel(lv_player, "UpgradeName", 3);

// Get current level
int lv_lvl = TechTreeUpgradeGetLevel(lv_player, "UpgradeName");

// Query event for upgrade changes
TriggerAddEventUpgradeLevelChanged(myTrigger, c_playerAny);
// Inside handler:
string lv_upgrade = EventUpgradeName();
int    lv_delta   = EventUpgradeLevelDelta();

// Production restrictions
TechTreeRestrictionsEnable(lv_player, "Marine", false);   // allow
TechTreeRestrictionsEnable(lv_player, "Marine", true);    // restrict
```

更深入的科技树内容（单位计数、前置条件检查、生产上限）在 `galaxy-ai-and-techtree` 中。

## 近战初始化（RTS 玩家设置）

为 AI/RTS 玩家初始化默认近战经济与单位摆放：

```galaxy
// Place starting units at startPoint for the given player and race
MeleeInitUnitsForPlayer(lv_player, PlayerRace(lv_player), lv_startPoint);

// Give standard starting resources
MeleeInitResourcesForPlayer(lv_player, PlayerRace(lv_player));

// Set upgrade levels for the RTS player (see Tech Tree Upgrades section)
TechTreeUpgradeAddLevel(lv_player, "UpgradeName", 1);
```
