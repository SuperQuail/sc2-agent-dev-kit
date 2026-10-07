# 对话框事件

在整个对话框或单个按钮上注册点击处理函数，并在处理函数内读取该事件。

## 对话框事件

### 为对话框内所有控件注册点击处理函数

```galaxy
TriggerAddEventDialogControl(
    myTrigger,
    c_playerAny,
    c_invalidDialogControlId,        // 0 = any control in dialog
    c_triggerControlEventTypeClick
);
```

### 为特定按钮注册点击处理函数

```galaxy
TriggerAddEventDialogControl(
    myTrigger,
    c_playerAny,
    lv_btn,
    c_triggerControlEventTypeClick
);
```

### 在处理函数内读取事件

```galaxy
bool MyClickHandler(bool testCond, bool runActions) {
    dialogcontrol clicked = EventDialogControl();
    int player = EventPlayer();

    if (clicked == gv_LockInButton) {
        HeroSelection_SelectHero(player, gv_SelectedHero[player]);
    }
    return true;
}
```
