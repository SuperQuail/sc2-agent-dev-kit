# 死亡与复活系统

## 英雄单位复活（`UnitRevive`）

`UnitRevive` 只对 `CUnitHero` 类型单位有效。它保留经验与等级，并让单位原地复活。

```galaxy
void HeroRevive_Init() {
    trigger lv_t = TriggerCreate("HeroRevive_Handler");
    TriggerAddEventUnitDied(lv_t, null);
}

bool HeroRevive_Handler(bool testConds, bool runActions) {
    unit lv_dead = EventUnit();  // capture BEFORE any Wait

    if (!UnitIsHero(lv_dead)) { return true; }

    Wait(10.0, c_timeGame);

    UnitRevive(lv_dead);  // revives in place; XP and level preserved
    return true;
}
```

## 普通单位重生（`UnitCreate`）

非英雄单位要在 `Wait` 之前捕获全部所需状态，然后重建：

```galaxy
bool UnitRespawn_Handler(bool testConds, bool runActions) {
    unit   lv_dead   = EventUnit();            // capture BEFORE Wait
    string lv_type   = UnitGetType(lv_dead);
    int    lv_player = UnitGetOwner(lv_dead);
    point  lv_pos    = UnitGetPosition(lv_dead);
    fixed  lv_face   = UnitGetFacing(lv_dead);

    Wait(10.0, c_timeGame);

    UnitCreate(1, lv_type, c_unitCreateIgnorePlacement,
        lv_player, lv_pos, lv_face);
    return true;
}
```

> **始终把 `EventUnit()` 及所有派生状态作为最先执行的动作捕获，早于任何 `Wait`。** `Wait` 之后，触发器事件上下文可能已被后来的死亡事件复用。
