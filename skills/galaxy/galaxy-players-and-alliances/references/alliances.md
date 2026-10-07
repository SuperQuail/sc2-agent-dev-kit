# 同盟

同盟等级预设、底层同盟通道、关系查询，以及开图时的同盟初始化范式。

## 同盟设置

### 使用库辅助函数（推荐）

```galaxy
// Ally with full shared vision
libNtve_gf_SetAlliance(1, 2, libNtve_ge_AllianceSetting_AllyWithSharedVision);

// Ally + shared vision + can push allies (used for same-team players)
libNtve_gf_SetAlliance(1, 2, libNtve_ge_AllianceSetting_AllyWithSharedVisionAndPushable);

// Ally + shared vision + full control over allies' units
libNtve_gf_SetAlliance(1, 2, libNtve_ge_AllianceSetting_AllyWithSharedVisionAndControl);

// Enemy
libNtve_gf_SetAlliance(1, 3, libNtve_ge_AllianceSetting_Enemy);

// Neutral with shared vision
libNtve_gf_SetAlliance(1, 2, libNtve_ge_AllianceSetting_NeutralWithSharedVision);

// Neutral (no vision)
libNtve_gf_SetAlliance(1, 2, libNtve_ge_AllianceSetting_Neutral);

// Precise alliance level presets (all confirmed in NativeLib_h.galaxy)
// libNtve_ge_AllianceSetting_Ally                                  = 0
// libNtve_ge_AllianceSetting_AllyWithSharedVision                  = 1
// libNtve_ge_AllianceSetting_AllyWithSharedVisionAndPushable        = 2
// libNtve_ge_AllianceSetting_AllyWithSharedVisionAndControl         = 3
// libNtve_ge_AllianceSetting_AllyWithSharedVisionControlAndSpending = 4
// libNtve_ge_AllianceSetting_Enemy                                  = 5
// libNtve_ge_AllianceSetting_EnemyWithSharedVision                  = 6
// libNtve_ge_AllianceSetting_Neutral                                = 7
// libNtve_ge_AllianceSetting_NeutralWithSharedVision                = 8
// libNtve_ge_AllianceSetting_NeutralWithSharedVisionAndPushable     = 9

// Player relation constants (for checking relationships, not setting them)
// libNtve_ge_PlayerRelation_Ally         = 0
// libNtve_ge_PlayerRelation_AllyMutual   = 1
// libNtve_ge_PlayerRelation_Neutral      = 2
// libNtve_ge_PlayerRelation_NeutralMutual = 3
// libNtve_ge_PlayerRelation_Enemy        = 4
// libNtve_ge_PlayerRelation_EnemyMutual  = 5
```

### 底层同盟控制

```galaxy
// Set a specific alliance channel
PlayerSetAlliance(lv_player, c_allianceIdTrade,        lv_other, true);
PlayerSetAlliance(lv_player, c_allianceIdControl,       lv_other, true);
PlayerSetAlliance(lv_player, c_allianceIdSeekHelp,      lv_other, false);
PlayerSetAlliance(lv_player, c_allianceIdPassive,       lv_other, false);
PlayerSetAlliance(lv_player, c_allianceIdSharedVision,  lv_other, true);
PlayerSetAlliance(lv_player, c_allianceIdPushable,      lv_other, true);  // units can physically push each other

// Query an alliance channel
bool lv_allied = PlayerGetAlliance(lv_player, c_allianceIdChat, lv_other);

// Common alliance ID constants
c_allianceIdTrade
c_allianceIdControl
c_allianceIdSeekHelp
c_allianceIdPassive
c_allianceIdPushToTalk
c_allianceIdSharedVision
c_allianceIdChat
c_allianceIdPushable
```

### 敌对/同盟关系查询

```galaxy
// libNtve helper
bool lv_isEnemy = libNtve_gf_PlayerIsEnemy(
    lv_me, lv_target,
    libNtve_ge_PlayerRelation_Enemy  // or _Ally or _Neutral
);
```

## 同盟初始化范式（SSF）

SSF 在开图时从零初始化全部同盟：先全部中立，再设具体关系：

```galaxy
void MapInit_ActivePlayers() {
    int tmpInt;

    gv_ActivePG  = PlayerGroupEmpty();
    gv_StartingPG = PlayerGroupEmpty();

    // Start everyone neutral
    libNtve_gf_SetPlayerGroupAlliance(PlayerGroupAll(), libNtve_ge_AllianceSetting_Neutral);

    // Specific overrides
    libNtve_gf_SetAlliance(gv_BasePlayer, gv_EnemyPlayer, libNtve_ge_AllianceSetting_Enemy);
    libNtve_gf_SetAlliance(gv_EnemyPlayer, gv_CollectiblePlayerEnemyAllied, libNtve_ge_AllianceSetting_AllyWithSharedVision);

    // Add active players to groups and set their alliances
    for (tmpInt = 1; tmpInt <= gv_MaxAmountPlayers; tmpInt += 1) {
        if (PlayerStatus(tmpInt) == c_playerStatusActive) {
            gv_PlayerStats[tmpInt].activeFlag = true;
            PlayerGroupAdd(gv_ActivePG, tmpInt);
            PlayerGroupAdd(gv_StartingPG, tmpInt);
            libNtve_gf_SetAlliance(tmpInt, gv_EnemyPlayer, libNtve_ge_AllianceSetting_Enemy);
            libNtve_gf_SetAlliance(tmpInt, gv_BasePlayer, libNtve_ge_AllianceSetting_AllyWithSharedVisionAndControl);
            PlayerOptionOverride(tmpInt, "simplecommandcard", "0");
        }
    }
    gv_PlayerAmount = PlayerGroupCount(gv_ActivePG);
    gv_PlayerAmountStart = gv_PlayerAmount;

    // All players allied with each other
    libNtve_gf_SetPlayerGroupAlliance(gv_StartingPG, libNtve_ge_AllianceSetting_AllyWithSharedVisionAndPushable);

    // Set enemy player color
    PlayerSetColorIndex(gv_EnemyPlayer, 13, true);
}
```
