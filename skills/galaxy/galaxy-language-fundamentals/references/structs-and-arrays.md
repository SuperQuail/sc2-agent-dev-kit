# 结构体与数组

声明结构体与定长数组，以及按引用传递它们。

## 结构体

结构体在头文件里声明。字段用 `.` 访问。

```galaxy
// Declaration (replace libXXXXXXXX_ with your mod's auto-generated prefix):
struct libXXXXXXXX_gs_Spawner {
    point  lv_point;
    string lv_unitType;
    int    lv_player;
};

struct libXXXXXXXX_gs_Spawn {
    point                          lv_point;
    libXXXXXXXX_gs_UnitsToSpawn[5] lv_unitSpawn;  // fixed-size array field
    int                            lv_respawnTime;
    int                            lv_spawnTime;
    trigger                        lv_deathCallbackTrigger;
};

// Usage:
libXXXXXXXX_gv_spawners[lv_index].lv_unitType = "Marine";
libXXXXXXXX_gv_createdSpawners[lv_index].lv_lastIndexPoint += 1;
```

按引用传递结构体用 `structref<T>`：

```galaxy
void libXXXXXXXX_gf_AddJungleSpawn (structref<libXXXXXXXX_gs_Spawn> lp_toSpawn) {
    libXXXXXXXX_gv_jungleToSpawn[libXXXXXXXX_gv_lastIndexOfSpawn].lv_point = lp_toSpawn.lv_point;
}
```

---

## 数组

定长数组用 `[size]` 声明。通常用 0 基下标。

```galaxy
// Declaration (in GlobalVariables.galaxy):
PlayerStruct[gv_MaxAmountPlayers + 1] gv_PlayerStats;   // indexed 1..gv_MaxAmountPlayers
int[gv_MaxAmountParts] gv_PartWins;                     // sized by const

// Multi-dimensional array (part x difficulty x playerCount):
int[gv_MaxAmountParts][gv_MaxAmountDifficulties][gv_MaxAmountPlayers] speedrunsTime;

// Initialization loop:
int tmpInt = 0;
for (; tmpInt < gv_MaxAmountPlayers; tmpInt += 1) {
    gv_ActivePG = PlayerGroupEmpty();
}
```
