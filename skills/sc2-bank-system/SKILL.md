---
name: sc2-bank-system
description: StarCraft II bank (.SC2Bank) campaign persistence for the AeonOfIhanrii campaign. Use when designing or editing campaign progress saves, player choices, unlocks, difficulty tracking, or mission completion state. Covers schema versioning, section design, BankLoad/Save workflow, and native Galaxy bank function signatures.
---
# SC2 Bank 系统与战役持久化

## 何时使用

任务涉及战役持久化时加载本技能：编写或修改 Bank 初始化、任务胜利存档、解锁状态、难度追踪，或任何调用 `BankLoad` / `BankSave` / `BankValueGet*` / `BankValueSet*` 的代码。为新战役设计 section/key schema，或把既有 Bank 跨版本迁移时也加载。

## 加载边界

- **本技能拥有：** Bank schema 与 section 设计、schema 版本与迁移、带类型的原生 Bank API 及其命名注意事项，以及持久化安全规则。
- **父技能拥有：** `sc2-project-entry` 拥有项目身份与 Bank 名/前缀证据；`sc2-galaxy-scripting` 拥有通用 Galaxy 语法与脚本真源规则；`sc2-map-triggers` 拥有预载与胜利钩子在地图中的接线位置。
- **兄弟技能拥有：** `galaxy-game-systems` 拥有更深的存/读档 API 参考；`sc2-tools-validation` 拥有校验工具；`sc2-editor-handoff` 拥有 playtest 与接受闸门；Bank 相关 playtest 设置见 [references/simulated-bank-playtests.md](references/simulated-bank-playtests.md)。

## 核心规则

1. **`BankSave` 是必须的**——Bank 写入从不自动落盘；漏掉保存会在卸载地图时被静默丢弃。
2. **缺失的键读出来是 `0` 或 `""`，与存了默认值无法区分**——解读取值前一律先用 `BankKeyExists` / `BankSectionExists` 守卫。
3. **每套 schema 都要版本化**：用 `Meta.SchemaVersion` 整数加逐级迁移链，迁移时回填安全默认值，而不是重置进度。
4. **把数据分进确定的 section**（`Meta`、`Missions`、`Choices`、`Upgrades`），不要把键撒得到处都是。
5. **使用原生名** `BankValueGetAsInt` / `BankValueSetFromInt` 和双参数的 `BankPreload(name, player)`——样例架构用的是旧名字，不要照抄。
6. **在地图初始化早期预载，在里程碑/胜利触发器里保存**，并通过模组的 GUI action 而不是地图里的裸 Custom Script。
7. **绝不把 Bank 状态当成共享的**——Bank 是每个玩家本地的；跨玩家数据必须走触发器同步或游戏状态。
8. **Bank 文件就是磁盘上的普通 XML**（`Documents\StarCraft II\Banks\<BankName>.SC2Bank`）；实际玩家、Bank 名与路径来自项目与运行时，绝不硬编码。
9. **新工作优先用 GUI Bank 触发器；** 内嵌 Custom Script 只用于 GUI 无法合理表达的部分。

## 参考文件

- [references/persistence-design.md](references/persistence-design.md) — section 布局、schema 版本化、键存在性安全与预载/保存时机。
- [references/bank-natives-and-caveats.md](references/bank-natives-and-caveats.md) — 带类型的原生函数表，以及 `GetAsInt`/`GetInt` 命名冲突；写任何 Bank 调用前读它。
- [references/bank-architecture-example.md](references/bank-architecture-example.md) — 完整的初始化/迁移/保存 Galaxy 样例及其迁移链。
- [references/hard-rules-and-safety.md](references/hard-rules-and-safety.md) — 必须保存、缺失键与跨客户端规则及其理由。
- [references/bank-system.md](references/bank-system.md) — 原始持久化架构页；[references/galaxy-bank.md](references/galaxy-bank.md) — 原生函数签名。
- [references/simulated-bank-playtests.md](references/simulated-bank-playtests.md) — 没有打包运行时的情况下如何模拟 Bank playtest。
- `skills/sc2-galaxy-scripting/references/galaxy-gotchas.md` — 同样适用于 Bank 代码的 Galaxy 语法规则。
