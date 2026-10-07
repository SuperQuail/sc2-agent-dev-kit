# 玩家资源

晶体矿、瓦斯、人口与 AI handicap 的读取和修改。

## 玩家资源

```galaxy
// Read
int lv_minerals = PlayerGetPropertyInt(lv_player, c_playerPropMinerals);
int lv_gas      = PlayerGetPropertyInt(lv_player, c_playerPropVespene);
int lv_supply   = PlayerGetPropertyInt(lv_player, c_playerPropSuppliesUsed);

// Write (absolute set)
PlayerModifyPropertyInt(lv_player, c_playerPropMinerals, c_playerPropOperSetTo, 500);

// Write (add/subtract)
PlayerModifyPropertyInt(lv_player, c_playerPropMinerals, c_playerPropOperAdd, 100);
PlayerModifyPropertyInt(lv_player, c_playerPropVespene,  c_playerPropOperSubtract, 25);

// Set handicap (AI difficulty scalar — reduces AI unit HP/damage)
PlayerModifyPropertyInt(lv_player, c_playerPropHandicap, c_playerPropOperSetTo, 50);

// Common property constants
c_playerPropMinerals
c_playerPropVespene
c_playerPropSuppliesUsed
c_playerPropSuppliesMade
c_playerPropSuppliesLimit
c_playerPropKills
c_playerPropDeaths
c_playerPropHandicap     // AI handicap level (0-100)
```
