# Data Table 基础（全局键值存储）

Data Table 用 `string` 名存放任意值，任何触发器都能访问。它的生命周期超出单个函数的作用域。

## 全局（整张地图）data table

```galaxy
// Save values — always pass true for the global (map-wide) scope
DataTableSetInt(true, "myKey", 42);
DataTableSetFixed(true, "position_x", 16.5);
DataTableSetBool(true, "isPhase2", true);
DataTableSetUnit(true, "heroUnit", lv_hero);
DataTableSetString(true, "lastEvent", "SpawnWave");
DataTableSetPoint(true, "spawnPt", lv_point);

// Load values
int    lv_n   = DataTableGetInt(true, "myKey");
fixed  lv_x   = DataTableGetFixed(true, "position_x");
bool   lv_b   = DataTableGetBool(true, "isPhase2");
unit   lv_u   = DataTableGetUnit(true, "heroUnit");
string lv_s   = DataTableGetString(true, "lastEvent");
point  lv_p   = DataTableGetPoint(true, "spawnPt");

// Check existence
bool lv_has = DataTableValueExists(c_dataTableScopeGlobal, "myKey");

// Remove
DataTableValueRemove(c_dataTableScopeGlobal, "myKey");

// Scope constants
c_dataTableScopeGlobal   // map-wide
c_dataTableScopeLocal    // trigger-local
```

## 实例 Data Table

实例表让你创建多个互相独立的表（相当于每个单位一本字典）：

```galaxy
DataTableInstanceCreate();
int lv_dt = DataTableInstanceLastCreated();

DataTableInstanceSetInt(lv_dt, "kills", 0);
DataTableInstanceSetUnit(lv_dt, "owner", lv_unit);

int  lv_k = DataTableInstanceGetInt(lv_dt, "kills");
unit lv_o = DataTableInstanceGetUnit(lv_dt, "owner");

DataTableInstanceClear(lv_dt);
```
