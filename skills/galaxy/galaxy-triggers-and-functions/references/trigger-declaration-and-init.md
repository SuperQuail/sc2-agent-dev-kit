# 触发器声明与初始化

覆盖父技能的 *Triggers* 一节：触发器如何声明、如何按字符串名注册、如何初始化。

触发器就是一个具名回调函数，注册后在特定游戏事件上触发。

## ⭐ SC2-IngameDevTools 触发器注册模式（首选——照这个写）

SC2-IngameDevTools 代码库在每个模块的 `_Init()` 函数里按字符串名注册触发器，模式干净统一。照这个写：

```galaxy
// Handler functions always have the bool(bool,bool) signature
bool BehaviorContainerSendHandler(bool a, bool b) {
    // ... handler logic
    return true;
}

bool BehaviorListBoxFilterQuery(bool a, bool b) {
    int player = EventPlayer();
    playergroup pg = PlayerGroupSingle(player);
    string val = DialogControlGetPropertyAsString(
        ListBoxFilter.editbox, c_triggerControlPropertyEditText, player);
    ItemList_FilterListRebuild(ItemList, ListBoxFilter, val, pg);
    return true;
}

void BehaviorHandler_Init() {
    trigger t;
    // Register trigger by STRING name — the string must exactly match the function name
    t = TriggerCreate("BehaviorContainerSendHandler");
    TriggerAddEventDialogControl(t, c_playerAny, BehaviorContainer.addButton,
        c_triggerControlEventTypeClick);
    TriggerAddEventDialogControl(t, c_playerAny, BehaviorContainer.removeButton,
        c_triggerControlEventTypeClick);

    // Filter query trigger
    t = TriggerCreate("BehaviorListBoxFilterQuery");
    TriggerAddEventDialogControl(t, c_playerAny, ListBoxFilter.editbox,
        c_triggerControlEventTypeTextChanged);
}
```

这个模式的关键规则：

- 触发器函数的签名固定为 `bool FunctionName(bool a, bool b)`。
- `TriggerCreate("FunctionName")`——字符串就是代码里**准确的函数名**。
- 每个模块的 `_Init()` 创建自己的全部触发器并挂接自己的事件。
- `trigger t;` 声明为局部变量，在同一个 `_Init()` 里复用于多次注册。
- 聊天消息触发器用 `TriggerAddEventChatMessage(t, c_playerAny, "commandString", false)`。
- 通用事件触发器用 `TriggerAddEventGeneric(handler, "EventName")` + `TriggerSendEvent("EventName")`。

```galaxy
// Chat command registration (from DevTools/ChatCommand.galaxy)
trigger DevTools_ChatCommand;

void DevTools_ChatCommand_Init() {
    DevTools_ChatCommand = TriggerCreate("DevTools_ChatCommand_Func");
    // Commands registered later via DevTools_ChatCommandCreate():
    //   TriggerAddEventChatMessage(DevTools_ChatCommand, c_playerAny, cmd, false);
    //   TriggerAddEventGeneric(handler, "DevTools_ChatCommand.Exec."+cmd);
}

void DevTools_ChatCommandCreate(string cmd, trigger handler, text description) {
    ItemListAdd(ItemList, cmd);
    TriggerAddEventChatMessage(DevTools_ChatCommand, c_playerAny, cmd, false);
    TriggerAddEventGeneric(handler, "DevTools_ChatCommand.Exec."+cmd);
    DevTools_ChatCommandSetDescription(cmd, description);
}

void DevTools_ChatCommandSend(string cmd) {
    TriggerSendEvent("DevTools_ChatCommand.Exec."+cmd);
}
```

## 声明触发器全局变量

```galaxy
// In the header (_h.galaxy) — replace libXXXXXXXX_ with your mod's auto-generated library prefix:
trigger libXXXXXXXX_gt_SpawnUnits;
trigger libXXXXXXXX_gt_WinTeam1;
```

## 完整触发器模式（编辑器生成的样子）

```galaxy
// 1. The logic function — bool (bool testConds, bool runActions)
bool gt_SpawnUnits_Func(bool testConds, bool runActions) {
    // Conditions block
    if (testConds) {
        if (!(someCondition)) { return false; }
    }
    // Actions block
    if (!runActions) { return true; }

    // ... do work ...

    return true;
}

// 2. The init function — registers event(s) onto the trigger
void gt_SpawnUnits_Init() {
    gt_SpawnUnits = TriggerCreate("gt_SpawnUnits_Func");
    TriggerEnable(gt_SpawnUnits, true);
    // attach one or more events:
    TriggerAddEventTimePeriodic(gt_SpawnUnits, 10.0, c_timeGame);
}
```

## 动态创建触发器（异步 / 独立线程）

```galaxy
trigger MyGlobalTrigger;

bool MyTrigger_Func(bool testConds, bool runActions) {
    UIDisplayMessage(PlayerGroupSingle(EventPlayer()), c_messageAreaChat,
        StringToText(EventChatMessage()));
    return true;
}

void MyTrigger_Init() {
    MyGlobalTrigger = TriggerCreate("MyTrigger_Func");
    TriggerAddEventChatMessage(MyGlobalTrigger, c_playerAny, "echo", false);
}
```
