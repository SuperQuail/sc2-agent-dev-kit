# 科技树——单位计数、限制与生产

## 单位计数

```galaxy
// How many of a unit type does player have?
int lv_count = TechTreeUnitCount(lv_player, "Marine", c_techCountBoth);

// Count constants
c_techCountAny   // counting training + alive
c_techCountBoth  // both alive and in training
c_techCountMade  // how many have ever been trained
c_techCountLost  // how many have been killed
```

## 限制

```galaxy
// Stop a player from training/building something
TechTreeRestrictionsEnable(lv_player, "Banshee", true);  // restrict
TechTreeRestrictionsEnable(lv_player, "Banshee", false); // allow

// Check if restricted
bool lv_restr = TechTreeRestrictionsEnabled(lv_player, "Banshee");

// Requirements (prerequisite check)
TechTreeRequirementsEnable(lv_player, false); // disable all requirements checks
```

## 生产

```galaxy
// Set production cap (max simultaneous training of a type)
TechTreeProductionCapSet(lv_player, "Marine", 5);

// Reset
TechTreeProductionCapSet(lv_player, "Marine", c_techTreeProductionCapUnlimited);
```

> `TechTreeRestrictionsEnable(player, entry, true)` 是**限制**（禁止），`false` 是**允许**。这个布尔值的读法与查询函数 `TechTreeRestrictionsEnabled` 相反。
>
> `TechTreeRequirementsEnable(player, false)` 会关掉该玩家**全部**前置条件检查——记得调回来。
