# Data Table 性能与作用域准则

（扩展自 [Data Tables guide](https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/041_Data_Tables/)）

## 查找时间对比

| 结构 | 查找时间 | 说明 |
|---|---|---|
| 数组 | 线性 O(n) | 放大 10 倍，查找时间也变成 10 倍。 |
| Data Table | 常数 O(1) | 查找时间不随表的大小增长。 |

小规模、类型统一、结构明确的集合用数组。集合大、动态变化或类型混杂时用 Data Table。

## 作用域规则

| 作用域 | 由谁创建 | 生命周期 | 可访问范围 |
|---|---|---|---|
| 全局（`c_dataTableScopeGlobal`） | 始终存在 | 整个游戏会话 | 任何触发器都能访问 |
| 局部（`c_dataTableScopeLocal`） | 触发器内第一次 `DataTableSet*` 调用 | 触发器退出时清空 | 仅当前触发器 |

**内存泄漏风险：** 全局 data table 是永久的。只增不删的条目会不断累积。数据不再需要时，一定要调用 `DataTableValueRemove(c_dataTableScopeGlobal, "key")` 或 `DataTableClear(c_dataTableScopeGlobal)`。局部表在所属触发器结束时自动清理。
