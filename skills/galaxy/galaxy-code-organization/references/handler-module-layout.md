# SC2-IngameDevTools 处理模块布局

本项目的主要文件结构模式：每个功能一个自包含的处理文件、一个协调器，以及 `_h.galaxy` 头文件。

## SC2-IngameDevTools 处理模块布局（首选模式）

SC2-IngameDevTools 项目把每个功能组织成 `Script/` 里一个自包含的**处理文件**。有一个主协调器、一个共享工具文件夹，以及一个放前置声明的 `_h.galaxy` 头文件。

### 文件夹与文件结构
```
Script/
├── DevToolsMain.galaxy          ← coordinator: includes all handlers, calls all _Init()
├── debug.galaxy                 ← global debug helpers: print(), console(), err()
├── debug_h.galaxy               ← forward declarations only (header pattern)
├── split_string.galaxy          ← utility (string split)
├── ItemList.galaxy              ← shared data structure (aggregator)
├── ItemListListBoxFormat.galaxy ← formatting helpers for ItemList
├── AbilityHandler.galaxy        ← feature handler (one per module)
├── AbilityOrderHandler.galaxy
├── ActorMessageHandler.galaxy
├── BehaviorHandler.galaxy
├── CameraHandler.galaxy
├── CameraShakeHandler.galaxy
├── CatalogLinkHandler.galaxy
├── CatalogValueHandler.galaxy
├── CheatHandler.galaxy
├── DataEditorHandler.galaxy
├── DataTableHandler.galaxy
├── DoodadHandler.galaxy
├── EffectHandler.galaxy
├── FogHandler.galaxy
├── LightingHandler.galaxy
├── PlayerHandler.galaxy
├── PortraitHandler.galaxy
├── RaceHandler.galaxy
├── SkinHandler.galaxy
├── SoundtrackHandler.galaxy
├── UnitHandler.galaxy
├── UpgradeHandler.galaxy
├── UserDataHandler.galaxy
├── WeaponHandler.galaxy
├── FreeCamHandler.galaxy
├── ItemList/
│   ├── index.galaxy             ← the actual ItemList implementation
│   └── Listbox.galaxy           ← listbox sub-feature
└── DevTools/
    ├── helpers.galaxy           ← shared helper functions (spawn point, movement tracker)
    ├── helpers_h.galaxy         ← forward declarations for helpers
    ├── ChatCommand.galaxy       ← chat command subsystem
    └── ChatCommand/
        └── Commands.galaxy        ← registered chat commands
```

### 主协调器（`DevToolsMain.galaxy`）
```galaxy
include "Script/debug"
include "Script/DevTools/helpers"
include "Script/ActorMessageHandler"
include "Script/BehaviorHandler"
include "Script/EffectHandler"
include "Script/UnitHandler"
include "Script/UpgradeHandler"
include "Script/WeaponHandler"
include "Script/AbilityHandler"
include "Script/CatalogValueHandler"
include "Script/CatalogLinkHandler"
include "Script/LightingHandler"
include "Script/DataTableHandler"
include "Script/DataEditorHandler"
include "Script/CameraHandler"
include "Script/FogHandler"
include "Script/DevTools/ChatCommand"
include "Script/DevTools/ChatCommand/Commands"

void DevToolsMain_Init() {
    DevTools_ChatCommand_Init();
    helpersInit();
    ActorMessageHandler_Init();
    CatalogValueHandler_Init();
    UnitHandler_Init();
    BehaviorHandler_Init();
    EffectHandler_Init();
    LightingHandler_Init();
    // ... every handler's _Init() called here
    DevTools_ChatCommand_Commands_Init();
}
```

### 入口接线（`Lib7C0075CB.galaxy` —— 编辑器生成的库文件）
```galaxy
include "TriggerLibs/natives"
include "Lib7C0075CB_h"
include "TriggerLibs/NativeLib"
include "Script/DevToolsMain"

void TestMap_main() {
    DevToolsMain_Init();
}
void lib7C0075CB_InitCustomScript() {
    TestMap_main();
}
bool lib7C0075CB_InitLib_completed = false;
void lib7C0075CB_InitLib() {
    if (lib7C0075CB_InitLib_completed) { return; }
    lib7C0075CB_InitLib_completed = true;
    lib7C0075CB_InitCustomScript();
}
```

### 头文件模式（`debug_h.galaxy`）
```galaxy
// ONLY forward declarations — no implementations
void print(string s);
void printT(text t);
void console(string s);
void err(string s);
```

### 单个处理模块的解剖（`BehaviorHandler.galaxy`）
```galaxy
// 1. Include shared utilities
include "Script/debug_h"
include "Script/ItemList"

// 2. Path constants with UPPER_SNAKE_CASE
static const string CONTAINERDLG_PATH =
    "UIContainer/ConsoleUIContainer/CatalogManager/BehaviorManager";

// 3. Module struct + global instance
ItemListContainerStruct BehaviorContainer;

// 4. File-private state
static string ItemList;
static ListBoxFilterStruct ListBoxFilter;

// 5. Trigger event functions (bool a, bool b signature)
bool BehaviorListBoxFilterQuery(bool a, bool b) { ... }
bool BehaviorListBoxSelectionChanged(bool a, bool b) { ... }
bool BehaviorContainerSendHandler(bool a, bool b) { ... }

// 6. One public Init function
void BehaviorHandler_Init() {
    playergroup pg = PlayerGroupAll();
    ItemList = "BehaviorList";
    ItemListContainer_InitStandard(BehaviorContainer, CONTAINERDLG_PATH,
        "BehaviorContainerSendHandler");         // trigger registered by STRING name
    ItemListInitFromCatalog(ItemList, c_gameCatalogBehavior, ItemListCatalogFilter);
    ItemList_FilterListInitStandard(ListBoxFilter, "BehaviorListBox",
        BehaviorListBoxSetActive, ItemListItemTextValue,
        CONTAINERDLG_PATH+"/NavList");
    ItemList_FilterListRebuild(ItemList, ListBoxFilter, "", pg);
}
```
