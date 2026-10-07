# 击杀资源奖励

监听单位死亡，判断类型，用 `PlayerModifyPropertyInt` 奖励击杀者。

```galaxy
void KillReward_Init() {
    trigger lv_t = TriggerCreate("KillReward_Handler");
    TriggerAddEventUnitDied(lv_t, null);
}

bool KillReward_Handler(bool testConds, bool runActions) {
    unit   lv_dead   = EventUnit();
    int    lv_killer = EventUnitKillingPlayer();
    string lv_type   = UnitGetType(lv_dead);

    if (lv_killer < 1) { return true; }  // killed by environment / trigger

    if (lv_type == "Zergling") {
        PlayerModifyPropertyInt(lv_killer, c_playerPropMinerals, c_playerPropOperAdd, 10);
    } else if (lv_type == "Roach") {
        PlayerModifyPropertyInt(lv_killer, c_playerPropMinerals, c_playerPropOperAdd, 25);
        PlayerModifyPropertyInt(lv_killer, c_playerPropVespene,  c_playerPropOperAdd, 5);
    }
    return true;
}
```

> `EventUnitKillingPlayer()` 返回造成致命一击的玩家。由非玩家来源击杀时返回 -1。

奖励数值可以改成查表，而不是硬编码——见 [references/userdata-reward-tables.md](userdata-reward-tables.md)。
