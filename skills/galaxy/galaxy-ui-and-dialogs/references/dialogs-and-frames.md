# 对话框与 XML 框架挂接

挂接已有的 SC2Layout 框架、用代码创建对话框、显示/隐藏它们，以及锚点常量。

## 对话框

对话框是叠加面板，可以用代码创建，也可以挂接到布局 XML 里已定义的 UI 框架上。

### 挂接已有的 XML 框架（SSF 范式 —— 推荐）

SSF 绑定到 SC2 UI XML 布局里已经定义好的框架：

```galaxy
// Hook a panel frame by path from the root UI hierarchy
dialogcontrol gv_UI_MasterFrame = DialogControlHookupStandard(
    c_triggerControlTypePanel,
    "UIContainer/FullscreenUpperContainer/SSF_CustomUI"
);

// Hook a specific child control (button, label, etc.) within a parent frame
dialogcontrol saveBtn = DialogControlHookup(
    gv_UI_MasterFrame,
    c_triggerControlTypeButton,
    "Menu/SaveButton"
);

// Register click event on the hooked control
TriggerAddEventDialogControl(
    TriggerCreate("Bank_ManualSave"),
    c_playerAny,
    saveBtn,
    c_triggerControlEventTypeClick
);
```

`UI-Main.galaxy` 中的初始化范式：
```galaxy
void SSFCustomUI_Init() {
    gv_UI_MasterFrame = DialogControlHookupStandard(c_triggerControlTypePanel, "UIContainer/FullscreenUpperContainer/SSF_CustomUI");
    StatsInterface_Init();   // each UI subsystem hooks its own controls
    HeroPanel_Init();
    PlayerBoard_Init();
    // ...
}
```

### 用代码创建对话框（备选）

```galaxy
dialog lv_dlg = DialogCreate(
    1200,              // width  (pixels)
    600,               // height (pixels)
    c_anchorCenter,    // anchor position on screen
    0,                 // x offset from anchor
    0,                 // y offset from anchor
    false              // modal (blocks input to game underneath)
);
// Or capture:
gv_heroDialog = DialogLastCreated();
```

### 显示 / 隐藏

```galaxy
DialogSetVisible(lv_dlg, PlayerGroupAll(), true);   // show to all
DialogSetVisible(lv_dlg, PlayerGroupSingle(3), false); // hide for player 3
```

### 锚点常量

```galaxy
c_anchorTopLeft
c_anchorTop
c_anchorTopRight
c_anchorLeft
c_anchorCenter
c_anchorRight
c_anchorBottomLeft
c_anchorBottom
c_anchorBottomRight
```
