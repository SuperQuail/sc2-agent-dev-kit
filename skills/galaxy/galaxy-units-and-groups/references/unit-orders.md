# 单位命令

单个单位与单位组的移动、攻击移动、停止、坚守和排队命令。

## 单位命令

```galaxy
// Move to point
UnitOrder(lv_unit, OrderTargetingPoint(AbilityCommand("Move", 0), lv_point));

// Attack-move to point
UnitOrder(lv_unit, OrderTargetingPoint(AbilityCommand("Attack", 0), lv_point));

// Stop
UnitOrder(lv_unit, Order(AbilityCommand("Stop", 0)));

// Hold position
UnitOrder(lv_unit, Order(AbilityCommand("HoldPosition", 0)));

// Queue orders
UnitOrderAdd(lv_unit, OrderTargetingPoint(AbilityCommand("Move", 0), lv_pt2), false);

// Queued group orders
UnitGroupOrder(lv_group, OrderTargetingPoint(AbilityCommand("Move", 0), lv_point), false);
```
