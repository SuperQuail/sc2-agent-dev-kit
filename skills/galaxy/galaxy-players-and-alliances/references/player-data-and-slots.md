# 玩家数据结构与槽位 ID

按玩家存储的状态结构体，以及单槽位与玩家组的关键类型区分。

## 玩家数据范式（SSF）

玩家状态存在以下标为玩家槽位（从 1 开始）的结构体数组里：

```galaxy
// In GlobalVariables.galaxy
const int gv_MaxAmountPlayers = 6;

struct PlayerStruct {
    bool   activeFlag;
    bool   spectatingFlag;
    bank   bankfile;
    unit   heroUnit;
    int    points;
    int[gv_MaxAmountParts] wins;
    int    heroUnlocked;    // bitflag
};
PlayerStruct[gv_MaxAmountPlayers + 1] gv_PlayerStats;

// CRITICAL TYPE DISTINCTION — single-slot player IDs vs player groups:
//   gv_rTSPlayer1, gv_rTSPlayer2  →  int   (single player SLOT ID, e.g. 7 or 14)
//   gv_soldierPlayers1/2          →  playergroup  (group of multiple players on a team)
//
// Single-slot IDs are assigned via PlayerGroupPlayer() which returns int:
//   gv_rTSPlayer1 = PlayerGroupPlayer(GameAttributePlayersForTeam(1), 1);
// These are used everywhere as int: IsZerg(gv_rTSPlayer1), PlayerStartLocation(gv_rTSPlayer2), etc.
// NEVER declare a single-slot id as playergroup — that causes 100+ "Parameter type mismatch" errors.

int gv_rTSPlayer1;        // int — single player slot for team 1
int gv_rTSPlayer2;        // int — single player slot for team 2
playergroup gv_soldierPlayers1;   // playergroup — all players on team 1
playergroup gv_soldierPlayers2;   // playergroup — all players on team 2

// Common shared playergroups
playergroup gv_ActivePG;      // currently active players
playergroup gv_StartingPG;    // players at game start
playergroup gv_SpectatingPG;  // spectators
int gv_PlayerAmount;
int gv_PlayerAmountStart;

// Special player slots
const int gv_BasePlayer  = 7;    // allied non-hero structures
const int gv_EnemyPlayer = 14;   // enemy units
```

访问按玩家的数据：
```galaxy
gv_PlayerStats[playerID].activeFlag = true;
unit hero = gv_PlayerStats[playerID].heroUnit;
PlayerGroupAdd(gv_ActivePG, playerID);
```
