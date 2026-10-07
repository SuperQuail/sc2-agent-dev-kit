# 命名规范

函数、结构体、全局或局部变量上该挂哪个前缀/后缀 —— 按项目形态而定。

## 命名规范

### ⭐ 处理模块模式（SC2-IngameDevTools —— 第一参考）

这是本项目的**规范命名约定**。每个模块都是一个自包含的 `.galaxy` 文件，有明确的公开入口 `ModuleName_Init()`，所有内部辅助函数标为 `static`。

| 种类 | 约定 | 示例 |
|---|---|---|
| **公开模块函数** | `ModuleName_FunctionName` | `UnitHandler_Init`, `ItemList_FilterListRebuild`, `DevTools_ChatCommandCreate` |
| **触发器（Trigger）处理函数** | `ModuleNameEvent(bool a, bool b)` | `BehaviorContainerSendHandler`, `UnitListBoxFilterQuery` |
| **库函数** | `libPrefix_lowercasename` | `libGalExe_unit`, `libGalExe_strip`, `libGalExe_debug` |
| **结构体类型** | `FeatureContainerStruct`（PascalCase + Struct 后缀） | `ItemListContainerStruct`, `ListBoxFilterStruct`, `EffectContainerStruct` |
| **结构体 typedef（structref）** | `StructNameRef` | `typedef structref<ItemListContainerStruct> ItemListContainerStructRef` |
| **Funcref typedef** | `CallbackNameRef` 或 `CallbackName` | `typedef funcref<ItemListSetActiveCallbackDef> ItemListSetActiveCallback` |
| **全局模块状态** | PascalCase 实例名 | `BehaviorContainer`, `LightingContainer`, `EffectContainer` |
| **文件内私有** | `static` 关键字 | `static string ItemList;`, `static ListBoxFilterStruct ListBoxFilter;` |
| **路径字符串常量** | `UPPER_SNAKE_CASE` + `static const string` | `CONTAINERDLG_PATH`, `ADDBTN_PATH`, `EDITBOX_PATH` |
| **配置/上限常量** | `UPPER_SNAKE_CASE` + `const int` | `ITEM_LIST_MAX`, `DEBUG_TYPE_INFO` |
| **DataTable 键前缀** | `c_FeatureName` 或裸字符串 | `c_ChatCommandkey = "DevTools.ChatCommand."` |
| **局部变量** | 短 camelCase（无前缀） | `player`, `pg`, `val`, `i`, `entry`, `trigRan` |
| **结构体字段** | camelCase（无前缀） | `panel`, `messageBox`, `addButton`, `ownerPlayerPulldown` |

#### SC2-IngameDevTools 的关键结构规则：

- 每个模块文件恰好一个公开的 `void ModuleName_Init()` 函数 —— 唯一入口。
- 所有内部辅助函数、状态变量、过滤函数都是 `static`。
- 触发器函数按字符串注册：`TriggerCreate("BehaviorContainerSendHandler")` —— 字符串**必须与函数名完全一致**。
- 触发器函数签名：**永远**是 `bool FunctionName(bool a, bool b)`。
- 头文件（`_h.galaxy`）只放**前置声明** —— 不放实现。
- 调试/日志辅助函数用超短全局名：`print(string s)`、`console(string s)`、`err(string s)`。
- 模块私有的 console 包装：`static void ModuleName_console(string s)` 包装 `TriggerDebugOutput`。

#### SC2-IngameDevTools 的真实示例：

```galaxy
// BehaviorHandler.galaxy — complete module pattern
include "Script/debug_h"
include "Script/ItemList"

static const string CONTAINERDLG_PATH = "UIContainer/ConsoleUIContainer/CatalogManager/BehaviorManager";

ItemListContainerStruct BehaviorContainer;   // global module state (PascalCase)
static string ItemList;                       // file-private
static ListBoxFilterStruct ListBoxFilter;     // file-private

// Trigger handler — bool(bool,bool) signature, registered by string name
bool BehaviorListBoxFilterQuery(bool a, bool b) {
    int player = EventPlayer();               // locals are plain camelCase
    playergroup pg = PlayerGroupSingle(player);
    string val = DialogControlGetPropertyAsString(
        ListBoxFilter.editbox, c_triggerControlPropertyEditText, player);
    ItemList_FilterListRebuild(ItemList, ListBoxFilter, val, pg);
    return true;
}

bool BehaviorContainerSendHandler(bool a, bool b) {
    bool trigRan = true;
    string val = DialogControlGetPropertyAsString(
        BehaviorContainer.messageBox, c_triggerControlPropertyEditText, EventPlayer());
    // ...
    return trigRan;
}

void BehaviorHandler_Init() {                 // the ONE public entry point
    playergroup pg = PlayerGroupAll();
    ItemList = "BehaviorList";
    ItemListContainer_InitStandard(BehaviorContainer, CONTAINERDLG_PATH,
        "BehaviorContainerSendHandler");
    ItemListInitFromCatalog(ItemList, c_gameCatalogBehavior, ItemListCatalogFilter);
    ItemList_FilterListInitStandard(ListBoxFilter, "BehaviorListBox",
        BehaviorListBoxSetActive, ItemListItemTextValue, CONTAINERDLG_PATH+"/NavList");
    ItemList_FilterListRebuild(ItemList, ListBoxFilter, "", pg);
}
```

```galaxy
// Struct declaration — FeatureNameStruct pattern
struct EffectContainerStruct {
    int panel;
    int messageBox;
    int addButton;
    int removeButton;
    int sourceButton;
    int targetButton;
    int sourceUnitFrame;
    int targetUnitFrame;
    unit source;
    unit target;
};
// Typedef for passing by reference:
typedef structref<EffectContainerStruct> EffectContainerStructRef;
```

```galaxy
// Funcref typedef pattern for callbacks
void ItemListSetActiveCallbackDef(ItemListStructRef itemList, int index, playergroup pg);
typedef funcref<ItemListSetActiveCallbackDef> ItemListSetActiveCallback;

int ItemListForEachCallBack(string element, int currentIndex, ItemListStructRef itemList);
typedef funcref<ItemListForEachCallBack> ItemListForEachCallBackRef;
```

```galaxy
// Library utility functions: libPrefix_lowercaseName
void libGalExe_debug(int player, string msg) { ... }
string libGalExe_strip(string message) { ... }
actor libGalExe_actor(int player, string param) { ... }
```

```galaxy
// debug.galaxy — ultra-short global debug helpers
void print(string s)   { TriggerDebugOutput(1, StringToText(s), true); }
void console(string s) { TriggerDebugOutput(1, StringToText(s), false); }
void err(string s)     { TriggerDebugOutput(1, StringToText(s), true); }

// Module-private console wrapper:
static void CatalogValueHandler_console(string s) {
    TriggerDebugOutput(DEBUG_TYPE_INFO, StringToText(s), false);
}
```

---

### 独立地图 / SC2Map（次要 —— SSF 模式）

| 种类 | 约定 | 示例 |
|---|---|---|
| 全局变量 | `gv_SystemName_Variable` | `gv_PlayerStats`, `gv_ActivePG` |
| 全局常量（全局配置） | `gv_MaxX` | `gv_MaxAmountPlayers`, `gv_GameTimeMax` |
| 具名常量 / 枚举 | `c_Category_Name` | `c_Part_Terran`, `c_BossFightState_Alive` |
| 函数 | `SystemName_Action` | `Player_AddExp`, `MapInit_ActivePlayers` |
| 静态触发器参数 | `SystemName_Param_Name` | `Utility_DelayedTextTagDestroyer_ParamTextTag` |
| 局部变量 | 无前缀（短名） | `tmpInt`, `hero`, `playerID` |
| 结构体字段 | 无前缀 | `activeFlag`, `heroUnit`, `bankfile` |

### 库模组 / SC2Mod（备选 —— 编辑器生成）

| 种类 | 前缀 | 示例 |
|---|---|---|
| 全局变量 | `libHASH_gv_` | `libXXXXXXXX_gv_soldiers` |
| 全局函数 | `libHASH_gf_` | `libXXXXXXXX_gf_IsZerg` |
| 触发器变量 | `libHASH_gt_` | `libXXXXXXXX_gt_SpawnJungle` |
| 结构体类型 | `libHASH_gs_` | `libXXXXXXXX_gs_Spawn` |
| 局部参数 | `lp_` | `lp_startPosition1` |
| 局部变量 | `lv_` | `lv_raceName` |
