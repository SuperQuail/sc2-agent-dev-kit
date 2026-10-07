# 地图接线检查单与战役地图位置

把原版暴雪战役地图接进自定义战役时：

1. **注册依赖：** SC2 编辑器 → Modules → Dependencies → 添加 `MyCampaignMod.SC2Mod`。
2. **初始化触发器：** 找到地图的初始化触发器（通常是 `Map Initialization` 或 `gt_MapInitialization`）。
3. **调用 GUI action：** 调用模组的自定义初始化 GUI action（例如 `libMy_InitMission("Map01")`）。
4. **Bank 预载：** 在地图初始化早期加上 `BankPreLoad("MyCampaignBank")`。
5. **胜利/失败钩子：** 确保任务胜利触发器在胜利过场之前调用战役进度保存动作。

## 关键规则：地图调用模组必须走 GUI action

除非地图调用的是模组导出的 **GUI Action Definition**，否则 SC2 链接器会 **静默丢掉模组库**。地图里的裸 Custom Script 块 **不** 算数。在模组库里定义 GUI action，再从地图触发器调用它们。

例：在地图初始化触发器里，把 `libMy_InitMission("Map01")` 当 GUI action 调用，不要走 Custom Script。

地图在活动依赖里引用目标模组，并通过模组导出的 GUI Action Definition 调用初始化函数。纯 Custom Script 引用可能导致链接器丢掉模组库；核验实际编译产物。

## 战役地图位置

地图位于 `agent-config.json` 的 `paths.campaign_maps_dir` 下：

- `voidprologue/` — 序章地图
- `void/` — 战役地图（paiur、pshakuras、ppurifier、ptaldarim、pkorhal、pmoebius、pulnar、pstory、sc2epilogue）
