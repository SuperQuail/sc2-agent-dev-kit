# 战术 AI 辅助函数与难度取值（NativeLib）

## 战术 AI 辅助函数

以下 `libNtve_gf_*` 函数已在 NativeLib.galaxy 中确认存在：

```galaxy
// Set tactical range for a unit type for a player
libNtve_gf_SetTacticalAIRange(lv_player, "Marine", 8);
// (player, unitType, distance)

// Set tactical think target for a unit type
libNtve_gf_SetTacticalAIThink(lv_player, "Marine", "Zerg_Zergling", false);
// (player, unitType, targetUnitType, isNative)

// Force an AI unit to cast an ability (via scripted order)
libNtve_gf_AICast(lv_unit, OrderTargetingPoint(AbilityCommand("PsiStorm", 0), lv_point));
// (unit, order)

// Declare the next town center location for AI expansion
libNtve_gf_DeclareNextTown(lv_player, lv_expansionPoint);
// (player, centerPoint)
```

## 基于难度的取值辅助函数

```galaxy
// Returns a value based on current game difficulty
int   lv_hp   = libNtve_gf_DifficultyValueInt  (100, 150, 200, 250);  // easy/normal/advanced/expert
fixed lv_spd  = libNtve_gf_DifficultyValueFixed(1.0, 1.25, 1.5, 2.0);
string lv_type = libNtve_gf_DifficultyValueUnitType("Zergling", "Hydralisk", "Ultralisk", "Ultralisk");
```
