# 科技树——升级

```galaxy
// Add a level to an upgrade for a player
TechTreeUpgradeAddLevel(lv_player, "MarineRange", 1);
TechTreeUpgradeAddLevel(lv_player, "MarineRange", -1);  // remove a level

// Set to specific level
TechTreeUpgradeSetLevel(lv_player, "MarineRange", 2);

// Read current level
int lv_lvl = TechTreeUpgradeGetLevel(lv_player, "MarineRange");

// Event when an upgrade changes
TriggerAddEventUpgradeLevelChanged(myTrigger, c_playerAny);
// Inside handler:
string lv_upgrade = EventUpgradeName();
int    lv_delta   = EventUpgradeLevelDelta();  // +1 or -1
int    lv_player  = EventPlayer();
```
