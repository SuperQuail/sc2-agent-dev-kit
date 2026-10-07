# 类型系统 —— 基元、句柄、引用类型与常量

Galaxy 基元类型、引擎句柄类型、`funcref`/`structref`/`arrayref` 引用类型，以及内置 `c_` 常量的完整清单。

## 基元类型

```galaxy
int     lv_count = 0;          // integer
bool    lv_flag = true;        // boolean
string  lv_name = "";          // text string (ASCII)
text    lv_display;            // localized text (UI)
fixed   lv_amount = 3.5;       // fixed-point number (like float)
byte    lv_b;                  // 8-bit integer
char    lv_c;                  // single character
```

### 所有引擎句柄类型（Galaxy 语言规范的完整清单）

```galaxy
unit             lv_hero;       // reference to a unit on the map
unitgroup        lv_soldiers;   // a collection of units
unitfilter       lv_filter;     // unit filter bitfield
unitref          lv_ref;        // unit reference (for data)
playergroup      lv_team;       // a collection of player slots
trigger          lv_spawnTrig;  // a trigger object
bank             lv_bank;       // a saved bank file
point            lv_pos;        // a 2D map coordinate
region           lv_region;     // a map region
color            lv_col;        // RGBA color
actor            lv_actor;      // model/effect visual actor
actorscope       lv_scope;      // actor scope (groups related actors)
abilcmd          lv_cmd;        // ability command (ability + target)
order            lv_order;      // unit order
sound            lv_snd;        // a playing sound instance
soundlink        lv_link;       // link to a sound asset
timer            lv_timer;      // a countdown or repeating timer
camerainfo       lv_cam;        // camera info / snapshot
revealer         lv_rev;        // vision revealer
marker           lv_marker;     // path marker
doodad           lv_doodad;     // doodad reference
bitmask          lv_bits;       // bitmask
generichandle    lv_handle;     // generic engine handle
effecthistory    lv_hist;       // effect history handle
aifilter         lv_aiFilter;   // AI filter
transmissionsource lv_trans;    // transmission source
wave             lv_wave;       // AI wave
waveinfo         lv_waveInfo;   // AI wave info
wavetarget       lv_waveTarget; // AI wave target
datetime         lv_dt;         // timestamp
```

### 引用类型（funcref / structref / arrayref）

它们让函数与结构体可以按引用传递 —— SSF 用它做 boss 技能回调与通用系统：

```galaxy
// funcref: store a pointer to a function matching a blueprint signature
bool blueprint_BossAbility(unitgroup ug, unit boss);
typedef funcref<blueprint_BossAbility> Blueprint_BossAbility;

Blueprint_BossAbility lv_fn = someConcreteFunction;
lv_fn(lv_group, lv_boss);   // call via funcref

// structref: pass a struct by reference (avoids copying)
void BossFight_Init(structref<BossFightData> data) {
    data.amountBosses = 3;
}

// arrayref: pass an array by reference
void FillArray(arrayref<int> arr, int size) {
    int i = 0;
    for (; i < size; i += 1) { arr[i] = 0; }
}
```

---

## 常量

```galaxy
// In a header file (replace libXXXXXXXX_ with your mod's auto-generated prefix):
const int libXXXXXXXX_gv_heroDialogWidth = 1200;
const int libXXXXXXXX_gv_buttonSizePickY = 50;

// Built-in engine constants use the c_ prefix:
c_timeGame          // time constant for Wait()
c_playerAny         // wildcard player slot
c_invalidDialogControlId
c_invalidDialogId
c_anchorTopLeft
c_stringAnywhere
c_stringNoCase
c_unitCountAll
c_gameOverVictory
c_gameOverDefeat
c_playerPropMinerals
c_playerPropVespene
c_playerPropOperSetTo
c_unitPropEnergy
c_unitPropCurrent
c_allianceIdSeekHelp
c_allianceIdChat
c_targetFilterMissile
c_targetFilterDead
c_targetFilterHidden
```
