# 持久化设计准则

1. **schema 版本化：** `Meta` section 里始终保留一个 `SchemaVersion` 整数。若 schema 在版本之间演进，迁移触发器就能安全地翻译旧存档，而不破坏玩家进度。
2. **确定的 section：** 把数据按逻辑分组进专用 section：
   - `<Section name="Meta">`：`SchemaVersion`、`CampaignCompleted`、`Difficulty`、`LastPlayedMission`。
   - `<Section name="Missions">`：逐任务完成状态、奖励目标、最高分。
   - `<Section name="Choices">`：阵营、指挥官、科技或部队配装选择。
   - `<Section name="Upgrades">`：全局货币、科技解锁、已购研究层级。
3. **安全与校验：** 读取前先确认键存在。键不存在时 SC2 Bank 读取函数返回默认空值（`""` 或 `0`）——一律先用 `BankKeyExists` 守卫，再把取值当有意义的数据解读。
4. **预载与保存时机：** 在地图初始化时预载 Bank（`BankPreLoad` / `BankLoad`）。在里程碑触发器或胜利过场时保存（`BankSave`）。

## Bank 文件位置

- **Windows：** `Documents\StarCraft II\Banks\<BankName>.SC2Bank`（配置了 OneDrive 重定向的机器上是 `OneDrive\Documents\StarCraft II\Banks\`）。
- **Mac：** `~/Library/Application Support/Blizzard/StarCraft II/Banks/<BankName>.SC2Bank`。

Bank 是按玩家本地存放的 XML 文件——不跨网络同步。

## 模组身份（AeonOfIhanrii）

- Bank 名常量：`"AeonOfIhanriiBank"`
- Galaxy 函数前缀：`libEpi_`（例如 `libEpi_Bank_Init`、`libEpi_Bank_SaveMissionVictory`）
- 一律在地图初始化早期通过模组的 GUI action 预载 Bank，不要用地图里的裸 Custom Script。
