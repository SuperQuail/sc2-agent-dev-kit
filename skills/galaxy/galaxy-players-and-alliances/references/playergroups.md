# 玩家组

`playergroup` 句柄的创建、填充、查询与遍历。

## 玩家组

### 创建 / 填充

```galaxy
playergroup lv_pg = PlayerGroupEmpty();     // empty group
PlayerGroupAdd(lv_pg, 3);                   // add player 3
PlayerGroupAdd(lv_pg, 4);

playergroup lv_all    = PlayerGroupAll();    // all players (observers too)
playergroup lv_active = PlayerGroupActive();// only active (connected, playing) players
playergroup lv_single = PlayerGroupSingle(lv_player);// single-player group

// From lobby — all players assigned to a team
playergroup lv_team1 = GameAttributePlayersForTeam(1);
```

### 查询

```galaxy
bool lv_has   = PlayerGroupHasPlayer(lv_pg, 2);
int  lv_count = PlayerGroupCount(lv_pg);
```

### 遍历活动玩家（SSF 范式）

```galaxy
int tmpInt = -1;
while (true) {
    tmpInt = PlayerGroupNextPlayer(gv_ActivePG, tmpInt);
    if (tmpInt < 0) { break; }
    // act on tmpInt
}
```
