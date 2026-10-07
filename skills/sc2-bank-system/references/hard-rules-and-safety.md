# Bank 硬规则

- **`BankSave` 是必须的**——写入不会自动持久化。`Set*` 调用之后忘记 `BankSave`，改动会在卸载地图时被静默丢弃。
- **缺失的键读出来是零或空**——没有先查 `BankKeyExists`，就绝不要把 `BankValueGetAsInt(...) == 0` 解读成「玩家什么都没完成」。读取时，缺失的键和存进去的 `0` 无法区分。
- **新工作优先用 GUI Bank 触发器。** 直接用原生函数在读遗留地图、理解暴雪示例时仍有价值。只有当 GUI 无法合理表达该操作时，才内嵌一个小的 Custom Script 动作。明确要求的 Galaxy 工作或既有脚本逻辑维护，用 `Epi_Main.galaxy` 或 `Base.SC2Data/Scripts/`；只在 `workspace_copy` 模式下部署，然后在编辑器里保存以重新生成编译库。
- **绝不要跨客户端信任 Bank 状态**——Bank 是每个玩家本地的。跨玩家比较必须通过触发器同步或游戏状态交换数据，而不是读 Bank。

## 来源注意事项

**Bank API 冲突：** `references/bank-system.md` 用的是 `GetInt`/`SetInt`/单参数 `BankPreLoad`；[references/galaxy-bank.md](galaxy-bank.md) 与 `skills/sc2-map-triggers/references/triggers-native/b.md` 用 `GetAsInt`/`SetFromInt` 和 `BankPreload(name, player)`。一律以原生定义和实际生成的脚本为准。保留架构思路；不要照抄示例。
