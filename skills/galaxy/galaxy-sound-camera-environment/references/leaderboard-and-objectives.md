# 排行榜与目标面板

创建、更新、排序和销毁计分板与目标面板。

排行榜就是默认的计分板面板。

## 排行榜

```galaxy
// Create a leaderboard
LeaderboardCreate(PlayerGroupAll(), StringToText("Kills"), c_leaderboardSortNone,
    c_leaderboardValueUInt, "", Color(1.0,1.0,1.0));
int lv_board = LeaderboardLastCreated();

// Show / hide
libNtve_gf_ShowHideLeaderboard(lv_board, PlayerGroupAll(), true);

// Add a row item for each player
LeaderboardAddItem(lv_board, PlayerGroupAll(), lv_player, IntToText(lv_kills),
    Color(1.0,1.0,1.0), "", Color(1.0,1.0,1.0));

// Set value of a player's row
LeaderboardSetItemValue(lv_board, PlayerGroupAll(), lv_player, IntToText(42),
    Color(1.0,0.8,0.0));

// Sort
LeaderboardSortByValue(lv_board, c_leaderboardSortDescending, false);

// Destroy
BoardDestroy(lv_board);
```

---

## 目标面板

```galaxy
// Create objective
ObjectiveCreate(
    PlayerGroupAll(),
    StringToText("Destroy Enemy Base"),
    StringToText("Eliminate all enemy structures."),
    c_objectivePrimary
);
int lv_obj = ObjectiveLastCreated();

// Show/update state
ObjectiveShow(lv_obj, true, false);
ObjectiveSetState(lv_obj, c_objectiveStateCompleted);

// State constants
c_objectiveStateActive
c_objectiveStateCompleted
c_objectiveStateFailed

// Destroy
ObjectiveDestroy(lv_obj);
```
