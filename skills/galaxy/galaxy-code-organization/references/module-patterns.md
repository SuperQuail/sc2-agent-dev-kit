# 模块模式 —— UI、Parts、Lib

三种反复出现的子结构模式：每面板一个 UI 文件外加协调器、每内容分块一个 Part 文件外加协调器，以及第三方 `Lib/` 文件夹。

## UI 子目录模式

每个 UI 面板都有自己的文件。协调器文件（`UI-Main.galaxy`）初始化它们全部：

```galaxy
// scripts/UI/UI-Main.galaxy
void SSFCustomUI_Init() {
    gv_UI_MasterFrame = DialogControlHookupStandard(c_triggerControlTypePanel, "UIContainer/...");
    StatsInterface_Init();
    Votekick_Init();
    Options_Init();
    HeroPanel_Init();
    PlayerBoard_Init();
    // ...
}
```

在 `main.galaxy` 里，各 UI 文件要 include 在 `UI-Main.galaxy` **之前**：

```galaxy
include "scripts/UI/UI-HeroPanel"
include "scripts/UI/UI-PlayerBoard"
include "scripts/UI/UI-Options"
// ... all panels first ...
include "scripts/UI/UI-Main"     // coordinator last, so it can call the others
```

---

## Parts 模式 —— 按内容分块

内容按战役任务/章节/阵营拆分时，用「协调器 + 每分块一个文件」：

```galaxy
// In main.galaxy — include individual parts BEFORE coordinator
include "scripts/PartTerran"
include "scripts/PartProtoss"
include "scripts/PartZerg"
include "scripts/Parts"        // coordinator included last
```

每个 `Part*.galaxy` 拥有自己的内容，并暴露一个 `TriggerCreate()` 函数：

```galaxy
// PartTerran.galaxy
void PartTerran_AreaJunker_Second_Open() { ... }
void PartTerran_TriggerCreate() {
    TriggerAddEventXxx(TriggerCreate("PartTerran_SomeHandler"), ...);
}
```

`Parts.galaxy` 在分块之间协调（处理分块切换、共享状态）：

```galaxy
// Parts.galaxy
static const int observer_AmountWaypoints = 11;
static point[observer_AmountWaypoints] observer_Waypoints;

void Part_InitVariables() { ... }
void Part_PartFinished() {
    PartTerran_TriggerCreate();   // calls into part files
}
```

---

## Lib 子目录 —— 第三方代码

外部/库代码放进 `scripts/Lib/`：

```galaxy
include "scripts/Lib/Starcode"    // third-party utility library
```

这样库代码就和自己的代码清楚分开了。
