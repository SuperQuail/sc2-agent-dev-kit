# 地图初始化模式

从 `main()` 到地图初始化触发器的引导链，以及幂等的库模组初始化模式。

## 地图初始化模式（SSF）

引导链：`MapScript.galaxy` → `main()` → 地图初始化触发器 → 各初始化函数：

```galaxy
// scripts/main.galaxy
void main() {
    TriggerAddEventMapInit(TriggerCreate("MapInit_Main"));
}

// scripts/MapInit.galaxy
bool MapInit_Main(bool testCond, bool runActions) {
    MapInit_ActivePlayers();   // alliances, playergroups
    SSFCustomUI_Init();        // UI
    PartTerran_TriggerCreate(); // register part triggers
    // ... other init calls
    return true;
}
```

### 库模组初始化模式（备选）

库使用幂等的 `InitLib` 函数：
```galaxy
bool libXXXXXXXX_InitLib_completed = false;
void libXXXXXXXX_InitLib() {
    if (libXXXXXXXX_InitLib_completed) { return; }
    libXXXXXXXX_InitLib_completed = true;
    libXXXXXXXX_InitVariables();
    libXXXXXXXX_InitTriggers();
}
```
