# 玩家信息与种族

名字、种族、颜色、槽位状态/类型、种族判定工具函数，以及设置玩家种族。

## 玩家信息

```galaxy
string lv_name   = PlayerName(lv_player);
string lv_race   = PlayerRace(lv_player);            // "Terr", "Zerg", "Prot", "random"
int    lv_color  = PlayerGetColorIndex(lv_player, false);
color  lv_c      = libNtve_gf_ConvertPlayerColorToColor(lv_player);

// Slot state
int lv_status = PlayerStatus(lv_player);  // c_playerStatusActive, c_playerStatusLeft, etc.
bool lv_active = (lv_status == c_playerStatusActive);

// Slot type
PlayerType(lv_player);  // c_playerTypeUser, c_playerTypeComputer, c_playerTypeHuman
```

## 种族工具函数

```galaxy
bool IsTerran(int player) {
    return StringContains(PlayerRace(player), "Terr", c_stringAnywhere, c_stringNoCase);
}
bool IsZerg(int player) {
    return StringContains(PlayerRace(player), "Zerg", c_stringAnywhere, c_stringNoCase);
}
bool IsProtoss(int player) {
    return StringContains(PlayerRace(player), "Prot", c_stringAnywhere, c_stringNoCase);
}
```

## 设置种族

```galaxy
PlayerSetRace(lv_player, "Terr");   // "Terr", "Zerg", "Prot"
```
