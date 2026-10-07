# 地图初始化与触发器注册模式

## 地图初始化 / 触发器注册模式（SSF）

SSF 风格的地图里，`main()` 只注册一个地图初始化触发器。每个模块在各自专属的 `_TriggerCreate()` 函数里注册自己的触发器：

```galaxy
// scripts/main.galaxy
void main() {
    TriggerAddEventMapInit(TriggerCreate("MapInit_Main"));
}

// scripts/MapInit.galaxy
bool MapInit_Main(bool testCond, bool runActions) {
    MapInit_ActivePlayers();
    SSFCustomUI_Init();
    PartTerran_TriggerCreate();   // each module registers its own triggers
    PartProtoss_TriggerCreate();
    PartZerg_TriggerCreate();
    return true;
}

// scripts/PartTerran.galaxy
void PartTerran_TriggerCreate() {
    TriggerAddEventUnitDied(TriggerCreate("PartTerran_SomeHandler"), null, false);
}
```

## 库模组模式（备用）

```galaxy
// Replace libXXXXXXXX_ with your mod's auto-generated library prefix.
void libXXXXXXXX_InitTriggers() {
    libXXXXXXXX_gt_SpawnUnits_Init();
    libXXXXXXXX_gt_WinTeam1_Init();
    // ... all _Init calls ...
}
```
