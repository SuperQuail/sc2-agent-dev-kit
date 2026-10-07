# 常见事件

按所触发事件的种类分组的注册调用。

## 时间事件

```galaxy
TriggerAddEventMapInit(myTrigger);                        // map loads
TriggerAddEventTimeElapsed(myTrigger, 5.0, c_timeGame);   // after 5 seconds
TriggerAddEventTimePeriodic(myTrigger, 3.0, c_timeGame);  // every 3 seconds
TriggerAddEventTimer(myTrigger, myTimer);                 // when a timer expires
```

## 单位事件

```galaxy
TriggerAddEventUnitDied(myTrigger, null, false);
TriggerAddEventUnitCreated(myTrigger, null, "", "");
TriggerAddEventUnitGainLevel(myTrigger, null);
TriggerAddEventUnitGainExperience(myTrigger, null);
TriggerAddEventUnitBehaviorChange(myTrigger, null, "", 0);
TriggerAddEventUnitRegion(myTrigger, null, someRegion, true);
TriggerAddEventUnitOrder(myTrigger, null, null);
TriggerAddEventUnitBecomesIdle(myTrigger, null);
TriggerAddEventUnitProperty(myTrigger, null, c_unitPropLife);

// Range-based proximity events
TriggerAddEventUnitRange(myTrigger, lv_unit, lv_nearUnit, 5.0, true);   // unit enters/exits radius of another unit
TriggerAddEventUnitRangePoint(myTrigger, null, lv_point, 8.0, true);    // any unit enters/exits radius of a point

// Unit is issued a specific ability order
TriggerAddEventUnitOrder(myTrigger, null, AbilityCommand("move", 0));

// Unit is attacked (EventUnitTarget() returns the attacker)
TriggerAddEventUnitAttacked(myTrigger, null);
unit lv_attacker = EventUnitTarget();   // inside handler

// Unit loaded/unloaded as cargo
TriggerAddEventUnitCargo(myTrigger, null, false);  // false = load event, true = unload

// Unit takes fatal damage (use with UnitDied for damage-source filtering)
TriggerAddEventUnitDamaged(myTrigger, null, c_unitDamageTypeAny, c_unitDamageFatal, null);

// Event accessors inside the trigger func:
unit lv_unit   = EventUnit();
int  lv_player = EventPlayer();
```

> 单位专属事件的语义（目标、伤害来源、载具）由 `galaxy-units-and-groups` 拥有；本文件只是注册清单。

## 玩家 / UI 事件

```galaxy
TriggerAddEventChatMessage(myTrigger, c_playerAny, "!cmd", false);
TriggerAddEventDialogControl(myTrigger, c_playerAny, myButton, c_triggerControlEventTypeClick);
TriggerAddEventPlayerJoin(myTrigger);
TriggerAddEventPlayerLeft(myTrigger, c_playerAny, c_gameOverLeave);
TriggerAddEventPlayerPropChange(myTrigger, c_playerAny, c_playerPropMinerals);
TriggerAddEventKeyPressed(myTrigger, c_playerAny, 'A', true, 0);
TriggerAddEventUpgradeLevelChanged(myTrigger, c_playerAny);
```

## 通用 / 自定义事件

```galaxy
TriggerSendEvent("MyCustomEvent");
TriggerAddEventGeneric(myTrigger, c_playerAny, "MyCustomEvent");
string lv_name = EventGenericName(); // inside handler
```
