---
name: galaxy-debug-data-catalog
description: Debug output, Data Table key-value storage, Catalog runtime field access, UserData, and asset preloading in Galaxy script. Use when reading or writing catalog fields at runtime, storing cross-trigger state in the Data Table, printing debug output, or preloading models and sounds. Do not use for bank save/load (use galaxy-game-systems). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 – 调试、Data Table 与 Catalog

运行时状态存储、运行时 catalog 访问、调试输出与运行时分析。

## When to Use（何时使用）

- 打印调试输出，或驱动游戏内调试窗口。
- 在 Data Table 中存放跨触发器状态（全局表、局部表或实例表）。
- 运行时读取或覆盖 catalog 字段，或读取 UserData 条目。
- 在用到之前预加载模型、图片、布局与模型动画。
- 用触发器调试器给实际运行中的地图做性能分析。

先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md)，了解项目级 Galaxy 语法约束（局部变量提升（hoisting）、没有 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、唯一真源规则、模块化脚本边界与命名约定（如 `libMy_` 函数前缀、`libMy_g_` 全局前缀）。本子技能只拥有深层 API 参考；项目约定在路由技能里。

## Loading Boundary（加载边界）

- **本子技能拥有：** 调试输出、Data Table 键值存储、catalog 运行时字段访问、UserData、资源预加载
- **父技能（`sc2-galaxy-scripting`）拥有：** 项目约定、schema/语法规则、唯一真源、命名前缀
- **兄弟子技能拥有：** `galaxy-game-systems`（bank 存读档）、`galaxy-triggers-and-functions`（通过触发器传跨触发器状态）、`sc2data-units-abilities`（XML catalog 编写）

## Core Rules（核心规则）

1. data table 的键按 `Module.Feature.Key` 划分命名空间，并带上玩家编号——两个系统共用裸键会静默互相覆盖。
2. 全局表（`true` / `c_dataTableScopeGlobal`）活满整个会话；用完就删键，否则内存会一直累积到本局结束。
3. 局部表（`false` / `c_dataTableScopeLocal`）在触发器退出时被清空——绝不要用它把状态交给另一个触发器。
4. 表查找是 O(1)，与表的大小无关，所以表变大不增加查找时间，但依然占内存。
5. 键不存在时 `DataTableGet*` 返回默认值；必须区分「未设置」与「零」时，先调用 `DataTableValueExists`。
6. 给 `CatalogFieldValueGet` 传你真正想要的玩家上下文——同一个 catalog 字段在不同玩家那里可以取到不同的值。
7. `CatalogFieldValueSet` 只覆盖某一个玩家的字段；要基于当前值做增减，用 `CatalogFieldValueModify` 加运算符常量。
8. 取数组字段要「先取长度再取下标」（`CatalogFieldValueCount` 然后 `"Weapons[0]"`）；光写字段名不会枚举数组。
9. `libNtve_gf_Catalog*` 辅助函数要求依赖链里有 NativeLib；纯 `Catalog*` native 不需要。
10. `UserDataGet*` 的参数顺序是 `(category, entryName, field, instanceIndex, player)`——把玩家号填到索引的位置不会报错，只会返回错误的值。
11. 在用到之前预加载资源；`ModelAnimationLoad` 是 native，不是 `libNtve_gf_` 辅助函数。
12. 触发器调试器本身很耗性能——关掉调试窗口里除 `Show Errors` 与 `Show User Output` 之外的所有「Show」选项，并让游戏以窗口模式运行。

## References（参考文件）

- [references/debug-output.md](references/debug-output.md) — `TriggerDebugOutput`、调试窗口、便捷 `Debug*` 辅助函数与消息类型常量。
- [references/data-table-basics.md](references/data-table-basics.md) — 全局 set/get/remove/作用域常量与实例 data table。
- [references/datatable-devtools-patterns.md](references/datatable-devtools-patterns.md) — SC2-IngameDevTools 的命名空间、UI 状态缓存、事件参数与 ItemList 后备存储模式。
- [references/data-table-performance-and-scope.md](references/data-table-performance-and-scope.md) — 数组与表的查找成本、全局与局部表的生命周期，以及全局表的内存泄漏风险。
- [references/catalog-runtime-fields.md](references/catalog-runtime-fields.md) — catalog 字段的读、写、改、数组下标与引用读取，以及 catalog 常量。
- [references/user-data-lookup.md](references/user-data-lookup.md) — 带类型的 `UserDataGet*` 取值函数及其参数顺序。
- [references/preloading-assets.md](references/preloading-assets.md) — `PreloadAsset` / `PreloadModel` / `PreloadImage` / `PreloadLayout` 与模型动画的加载/卸载。
- [references/trigger-debugger.md](references/trigger-debugger.md) — 启动调试器、它的标签页、断点与性能分析流程。
- [references/sources-and-codebases.md](references/sources-and-codebases.md) — 前置说明段，以及调试器/优化/Data Tables 指南与 NativeLib 的 catalog 辅助函数。
