---
name: galaxy-units-and-groups
description: Unit creation, properties, behaviors, abilities, XP/leveling, unit groups, orders, death/kill events, custom values, and cargo in Galaxy script. Use when creating units, modifying HP/shields/energy, adding behaviors, issuing orders, querying or iterating unit groups, handling level-up events, or listening for unit death. Do not use for the AI that controls units (use galaxy-ai-and-techtree). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 —— 单位与单位组

## 何时使用

- 创建、杀死、移除或移动单位，判断单位是否存活或有效。
- 读写单位属性（生命、护盾、能量、击杀数、速度）或四个自定义值槽位。
- 添加/移除行为（Behavior），重置或启用技能（Ability），下达单位或单位组命令。
- 构建、过滤、查询和遍历单位组，处理升级 / 死亡事件。
- 装载量、缩放与变体、单位状态标志、所有权转移、选中控制。

## 加载边界

- **本子技能负责：** 单位创建、属性、行为、技能、经验/升级、单位组、命令、死亡/击杀事件、自定义值、装载量
- **父根技能（`sc2-galaxy-scripting`）负责：** 项目约定、schema/语法规则、真源、命名前缀
- **兄弟子技能负责：** `galaxy-ai-and-techtree`（控制单位的 AI）、`galaxy-actor-and-visuals`（视觉挂接）、`galaxy-triggers-and-functions`（通过触发器（Trigger）的单位事件）

## 核心规则

1. 先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md)：它负责语法约束（局部变量提升（hoisting）、不支持 `++`/`--`、用 `while` 而非 `for`）、真源规则和 `libMy_`/`libMy_g_` 命名。本技能只是 API 参考。
2. 绝不写凭记忆的函数名：先在 `TriggerLibs/NativeLib.galaxy` 或原生参考中确认——编造的名字会浪费一整轮编译。
3. `UnitCreate(...)` 没有返回值；要在紧接着的下一条语句用 `UnitLastCreated()` / `UnitLastCreatedGroup()` 取回，中间不能插入任何其他创建调用。
4. `UnitCreate` 需要一个显式标志：`c_unitCreateIgnorePlacement`（常用选择）或 `c_unitCreateUsePreferences`——该标志决定是否套用放置规则。
5. `UnitGetPropertyFixed(u, prop, true)` 读当前值，`false` 读最大值；最大值/百分比是各自独立的常量（`c_unitPropLifeMax`、`c_unitPropLifePercent`）。
6. `UnitKill` 不给击杀归属也不给经验，`UnitRemove` 跳过死亡动画和死亡事件——两者不能互换。
7. 单位组下标从 1 开始；只要循环中可能移除单位，就用 `UnitGroupUnitFromEnd` 从末尾遍历。
8. `UnitFilter(requiresMask, excludeMask, requiresType, excludeType)` 有四个位置参数，位掩码写作 `(1 << c_targetFilter...)`——保持顺序。
9. `UnitBehaviorAdd`/`UnitBehaviorRemove`/`UnitBehaviorSetCount` 都带一个来源单位参数；作用于自身的行传单位自己。
10. 自定义值是四个定点数槽位（下标 0–3，默认 `0.0`）；只存在于运行时，不会写入 bank。
11. `UnitSetOwner` 和 `UnitSetState` 改变所有权/可见性但不移动单位，且不可选中的单位仍可被选为目标。
12. 命令由技能-指令字符串构造，例如 `UnitOrder(u, OrderTargetingPoint(AbilityCommand("Move", 0), pt))`；`UnitOrderAdd` 和 `UnitGroupOrder` 末尾带一个布尔标志。

## 参考文档

- [references/unit-creation-and-lifecycle.md](references/unit-creation-and-lifecycle.md) — 创建、检测、杀死和定位单位。
- [references/unit-properties-and-custom-values.md](references/unit-properties-and-custom-values.md) — 属性常量、自定义值槽位、装载量、缩放。
- [references/behaviors-and-abilities.md](references/behaviors-and-abilities.md) — 行为的增删改计数与技能控制。
- [references/experience-and-leveling.md](references/experience-and-leveling.md) — 经验与等级查询，以及升级事件。
- [references/unit-groups.md](references/unit-groups.md) — 单位组的创建、过滤、查询与安全遍历。
- [references/unit-orders.md](references/unit-orders.md) — 移动/攻击/停止/坚守与排队命令。
- [references/unit-state-and-selection.md](references/unit-state-and-selection.md) — 状态标志、所有权、变体、选中控制。
- [references/sources-and-guides.md](references/sources-and-guides.md) — 自己造 API 之前先看的上游代码库与编辑器指南。
