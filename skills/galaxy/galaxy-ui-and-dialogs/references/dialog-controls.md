# 对话框控件

挂接函数表、SC2-IngameDevTools 控件范式，以及按钮、文本标签、图片与头像的创建/更新。

## 对话框控件

### ⭐ SC2-IngameDevTools `DialogControlHookup` 范式（首选 —— 用于已有的 XML 框架）

SC2-IngameDevTools 代码库挂接的是**已有的 XML UI 框架**，而不是用代码新建 dialog control。当 UI 框架定义在 `Base.SC2Data/UI/Layout/` 下时，这是首选范式：

```galaxy
// Hookup an existing XML panel by its frame path (no creation needed)
// The path is relative to the UI layout root
static const string CONTAINERDLG_PATH =
    "UIContainer/ConsoleUIContainer/CatalogManager/BehaviorManager";
static const string EDITBOX_PATH    = "Item";
static const string ADDBTN_PATH     = "AddButton";
static const string REMOVEBTN_PATH  = "RemoveButton";

struct BehaviorContainerStruct {
    int panel;       // the hooked panel (int = dialog control handle)
    int messageBox;
    int addButton;
    int removeButton;
};
BehaviorContainerStruct BehaviorContainer;

void BehaviorHandler_Init() {
    trigger t;
    // Hook the root panel by absolute path
    BehaviorContainer.panel =
        DialogControlHookupStandard(c_triggerControlTypePanel, CONTAINERDLG_PATH);

    // Hook child controls relative to the panel
    BehaviorContainer.messageBox =
        DialogControlHookup(BehaviorContainer.panel, c_triggerControlTypeEditBox, EDITBOX_PATH);
    BehaviorContainer.addButton =
        DialogControlHookup(BehaviorContainer.panel, c_triggerControlTypeButton, ADDBTN_PATH);
    BehaviorContainer.removeButton =
        DialogControlHookup(BehaviorContainer.panel, c_triggerControlTypeButton, REMOVEBTN_PATH);

    // Register click handler — trigger registered by STRING function name
    t = TriggerCreate("BehaviorContainerSendHandler");
    TriggerAddEventDialogControl(t, c_playerAny, BehaviorContainer.addButton,
        c_triggerControlEventTypeClick);
    TriggerAddEventDialogControl(t, c_playerAny, BehaviorContainer.removeButton,
        c_triggerControlEventTypeClick);
}
```

用到的关键函数：
| 函数 | 用途 |
|---|---|
| `DialogControlHookupStandard(type, path)` | 按绝对 XML 路径挂接根级框架 |
| `DialogControlHookup(parent, type, childPath)` | 相对父面板挂接子控件 |
| `DialogControlGetPropertyAsString(ctrl, prop, player)` | 读文本/字符串属性 |
| `DialogControlSetPropertyAsString(ctrl, prop, pg, val)` | 写文本/字符串属性 |
| `DialogControlGetPropertyAsInt(ctrl, prop, player)` | 读整数属性（例如选中下标） |
| `DialogControlGetSelectedItem(list, player)` | 取列表框选中项下标 |
| `DialogControlAddItem(list, pg, text)` | 向列表框/下拉框添加条目 |
| `DialogControlRemoveAllItems(list, pg)` | 清空列表框全部条目 |
| `DialogControlGetItemCount(list, player)` | 统计列表框条目数 |

### 按钮

```galaxy
libNtve_gf_CreateDialogItemButton(
    lv_dlg,          // parent dialog
    200, 50,         // width, height
    c_anchorTopLeft, // anchor within dialog
    10, 10,          // x, y offset from anchor
    StringToText(""),         // tooltip
    StringToText("Click Me"), // label
    ""               // style (empty = default)
);
dialogcontrol lv_btn = DialogControlLastCreated();
```

### 文本标签

```galaxy
libNtve_gf_CreateDialogItemLabel(
    lv_dlg,
    300, 40,
    c_anchorTopLeft,
    10, 60,
    StringToText("Score: 0"),  // initial text
    ColorWithAlpha(255, 255, 255, 255), // white
    false,           // word wrap
    0                // wrap width (0 = no limit)
);
dialogcontrol lv_label = DialogControlLastCreated();
```

### 图片

```galaxy
libNtve_gf_CreateDialogItemImage(
    lv_dlg,
    100, 100,
    c_anchorTopLeft,
    10, 110,
    StringToText(""),   // tooltip
    "Assets\\Textures\\UI_Protoss_Something.dds",
    c_triggeredImageTypeNormal,
    false,   // tiled
    ColorWithAlpha(255, 255, 255, 255)
);
dialogcontrol lv_img = DialogControlLastCreated();
```

### 头像 / 单位图像

```galaxy
libNtve_gf_CreateDialogItemPortrait(
    lv_dlg, 150, 150, c_anchorTopLeft, 10, 10,
    StringToText(""), -1,
    null, // unit (null for no unit)
    true  // use unit model
);
```

## 更新控件

```galaxy
// Change text
libNtve_gf_SetDialogItemText(lv_label, StringToText("Score: 42"), PlayerGroupAll());

// Change image
libNtve_gf_SetDialogItemImage(lv_img, "Assets\\Textures\\NewImage.dds", PlayerGroupAll());

// Enable / disable
DialogControlSetEnabled(lv_btn, PlayerGroupAll(), true);
DialogControlSetEnabled(lv_btn, PlayerGroupSingle(lv_player), false);

// Guard against an uninitialized control (c_invalidDialogControlId == 0)
if (lv_label != c_invalidDialogControlId) {
    libNtve_gf_SetDialogItemText(lv_label, StringToText("..."), PlayerGroupAll());
}
```
