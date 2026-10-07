# 英雄、升级与计分板面板

SSF 的挂接框架面板范式：英雄选择、升级面板、玩家板，以及 UI 模式/框架可见性。

## 英雄选择对话框范式（SSF）

SSF 用挂接的 XML 框架做英雄选择。每个面板是 `scripts/UI/` 下的独立文件。`HeroSelection.galaxy` 驱动逻辑，`UI-HeroPanel.galaxy` 拥有 dialog control：

```galaxy
// UI-HeroPanel.galaxy
static dialogcontrol HeroPanel_MainFrame;
static dialogcontrol[gv_MaxAmountHeroes + 1] HeroPanel_HeroButtons;

void HeroPanel_Init() {
    HeroPanel_MainFrame = DialogControlHookup(gv_UI_MasterFrame, c_triggerControlTypePanel, "HeroPanel");
    int i = 1;
    for (; i <= gv_MaxAmountHeroes; i += 1) {
        HeroPanel_HeroButtons[i] = DialogControlHookup(HeroPanel_MainFrame, c_triggerControlTypeButton, "Hero" + IntToString(i));
    }
    TriggerAddEventDialogControl(TriggerCreate("HeroPanel_Click"), c_playerAny, c_invalidDialogControlId, c_triggerControlEventTypeClick);
}

void HeroPanel_UpdatePlayer(int playerID) {
    // Show/hide based on unlock state
    int i = 1;
    for (; i <= gv_MaxAmountHeroes; i += 1) {
        bool unlocked = ((gv_PlayerStats[playerID].heroUnlocked & (1 << i)) != 0);
        DialogControlSetEnabled(HeroPanel_HeroButtons[i], PlayerGroupSingle(playerID), unlocked);
    }
}
```

### 升级面板

```galaxy
bool HeroLevelUp_Handler(bool testCond, bool runActions) {
    unit hero = EventUnit();
    int level = UnitXPGetCurrentLevel(hero);
    int player = UnitGetOwner(hero);
    // Show appropriate upgrade panel for this level
    if (level == 2) {
        DialogControlSetVisible(gv_UpgradeFrame_Level2, PlayerGroupSingle(player), true);
    }
    return true;
}
```

## 计分板 / 统计面板（SSF 范式）

SSF 用挂接的 XML 框架做玩家板，通过调用 `PlayerBoard_UpdatePlayer(playerID)` 更新：

```galaxy
// UI-PlayerBoard.galaxy
static dialogcontrol PlayerBoard_MainFrame;
static dialogcontrol[gv_MaxAmountPlayers + 1] PlayerBoard_KillsLabel;
static dialogcontrol[gv_MaxAmountPlayers + 1] PlayerBoard_ScoreLabel;

void PlayerBoard_Init() {
    PlayerBoard_MainFrame = DialogControlHookup(gv_UI_MasterFrame, c_triggerControlTypePanel, "PlayerBoard");
    int i = 1;
    for (; i <= gv_MaxAmountPlayers; i += 1) {
        PlayerBoard_KillsLabel[i] = DialogControlHookup(PlayerBoard_MainFrame, c_triggerControlTypeLabel, "Player" + IntToString(i) + "/Kills");
    }
}

void PlayerBoard_UpdatePlayer(int playerID) {
    libNtve_gf_SetDialogItemText(
        PlayerBoard_KillsLabel[playerID],
        IntToText(gv_PlayerStats[playerID].kills),
        PlayerGroupAll()
    );
}
```

### UI 模式控制

```galaxy
// Switch player(s) to fullscreen UI (hides default game HUD)
UISetMode(PlayerGroupAll(), c_uiModeFullscreen, c_transitionDurationImmediate);

// Hide specific HUD frames
UISetFrameVisible(PlayerGroupAll(), c_syncFrameTypeSupply,        false);  // hide supply display
UISetFrameVisible(PlayerGroupAll(), c_syncFrameTypeResourcePanel, false);  // hide minerals/gas panel

// Hide alert types
UISetAlertTypeVisible(PlayerGroupAll(), "AlertWorkerAttacked", false);
```
