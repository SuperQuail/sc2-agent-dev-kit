---
name: galaxy-points-regions-geometry
description: Points, regions, geometry, pathfinding, and map coordinate helpers in Galaxy script. Use when working with point creation, distance calculations, offsets, region checks, PointWithOffset, PointWithPolarProjection, region creation, or testing if a unit or point is inside a region. Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 —— 点、区域与几何

## 何时使用

- 创建、移动或测量 `point`；偏移或投影一个位置。
- 查询某个点处的地形情况（高度、悬崖层级、纹理、可通行性）。
- 用代码构造 `region`，或判断单位/点是否在其中。
- 响应单位进入或离开区域。
- 揭示/探索地图，或在世界坐标处放飘字/小地图标记。

## 加载边界

- **本子技能拥有：** 点、区域、几何、寻路，以及地图坐标辅助函数（PointWithOffset、PointWithPolarProjection、区域判定）
- **父根技能（`sc2-galaxy-scripting`）拥有：** 项目约定、schema/语法规则、真源（source-of-truth）、命名前缀
- **兄弟子技能拥有：** `galaxy-math-strings-conversion`（数值数学原语）、`galaxy-units-and-groups`（单位位置查询）、`galaxy-sound-camera-environment`（相机目标点）

## 核心规则

1. **前置条件：** 先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md) —— 项目级语法约束（局部变量提升（hoisting）、无 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、真源规则、模块脚本边界与命名规范（`libMy_` 函数前缀、`libMy_g_` 全局前缀）都在那里；本子技能只拥有几何 API 参考。
2. `point` 是独立类型，不是一对数字 —— 坐标存在它内部；用 `PointGetX`/`PointGetY` 读取，绝不要假设结构体布局。
3. `PointSet`、`PointSetFacing`、`PointSetHeight`、`RegionAdd`/`RegionRemove` 就地修改。原值要保留就先复制。
4. `PointWithOffset`/`PointWithOffsetPolar` 会**创建**新点；`PointSet` 修改已有点。别把两者搞混。
5. 角度处处都是度（朝向、极坐标投影、`AngleBetweenPoints`）。
6. `DistanceBetweenPoints` 与 `AngleBetweenPoints` 是 2D 的；高度要用点自身的 `WorldHeight`/`PointGetHeight`。
7. 点/区域参数是引擎句柄：循环里创建点、又只反复覆盖那唯一引用的话，该点会一直泄漏到地图关闭 —— 尽量复用一个点。
8. `RegionEntireMap()`、`RegionPlayableMap()`、`RegionEmpty()` 每次调用都返回**新**区域；存起来，别放进循环条件里调用。
9. 区域进入/离开事件是按单位触发的，不是按队伍 —— 做昂贵操作前先用 `EventUnit()` 过滤。
10. `PointPathingPassable`/`PointPathingIsConnected` 只回答地面寻路；它们不保证飞行或潜地单位能到达该点。
11. 每帧或每单位创建的飘字必须显式销毁（`TextTagDestroy`）—— 它们不被垃圾回收。
12. 可见性调用（`VisRevealArea`、`VisExploreArea`、`VisEnable`）会为某个玩家改变游戏状态；在循环里调用前先确认目标玩家槽位。

## 参考文档

- [references/points.md](references/points.md) —— 创建/偏移点、读取坐标或朝向，或查询某点地形（高度、悬崖、可通行性）时
- [references/regions.md](references/regions.md) —— 构造区域、测量区域、判断包含关系，或挂接区域进入/离开事件时
- [references/pathing-and-visibility.md](references/pathing-and-visibility.md) —— 检查跨悬崖、寻路类型、战争迷雾或揭示者时
- [references/creep-and-minimap-pings.md](references/creep-and-minimap-pings.md) —— 添加/移除虫族菌毯，或在小地图上打标记时
- [references/text-tags.md](references/text-tags.md) —— 在点或单位上方显示世界空间飘字时
- [references/external-sources.md](references/external-sources.md) —— 当需要 native 参考或上游指南时
