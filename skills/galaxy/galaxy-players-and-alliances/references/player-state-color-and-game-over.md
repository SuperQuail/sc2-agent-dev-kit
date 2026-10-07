# 玩家状态、颜色与结束游戏

玩家状态标志、leader panel/分数开关、玩家颜色转换，以及结束游戏。

## 玩家状态标志

```galaxy
// Hide this player from the leader panel (used for non-playing computer slots)
PlayerSetState(lv_player, c_playerStateDisplayInLeaderPanel, false);

// Disable score accumulation for this player
PlayerSetState(lv_player, c_playerStateShowScore, false);
PlayerSetState(lv_player, c_playerStateXPGain, false);

// Disable unit fidgeting animations (helps performance / cutscenes)
PlayerSetState(lv_player, c_playerStateFidgetingEnabled, false);

// Query a state
bool lv_inPanel = PlayerGetState(lv_player, c_playerStateDisplayInLeaderPanel);
```

## 结束游戏

```galaxy
// For a specific player
GameOver(lv_player, c_gameOverVictory, true, true);  // (player, result, showScore, restart)
GameOver(lv_player, c_gameOverDefeat,  true, true);

// For a playergroup
libNtve_gf_EndGameForPlayerGroup(lv_group, c_gameOverVictory, true, true);

// Constants
c_gameOverVictory
c_gameOverDefeat
c_gameOverLeave
c_gameOverTie
```

## 玩家颜色

```galaxy
// Get color index
int lv_idx = PlayerGetColorIndex(lv_player, false);

// Convert to color value
color lv_color = libNtve_gf_ConvertPlayerColorToColor(lv_player);

// Use in text
text lv_colored = TextWithColor(StringToText(PlayerName(lv_player)), lv_color);
```
