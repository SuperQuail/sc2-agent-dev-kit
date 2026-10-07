---
name: sc2-map-triggers
description: StarCraft II map trigger XML and wiring for the AeonOfIhanrii campaign. Use when editing .SC2Map Triggers, wiring campaign map initialization, bank preload, victory/defeat hooks, or generating trigger XML. Covers GUI action requirement, trigger element schema, event declarations, and per-map setup.
---
# SC2 地图触发器与接线

## 何时使用

在编辑地图触发器 XML（`.SC2Map/Triggers`）、接线战役地图初始化、Bank 预载、胜利/失败钩子，或在外部生成触发器 XML 时加载本技能。

新触发器优先用编辑器可编辑的事件、条件、动作与 GUI 自定义定义。若有一小部分确实无法用 GUI 合理表达，只把那一部分作为 Custom Script 内嵌在触发器里，周边流程仍留在 GUI。除非用户明确要求 Galaxy，否则不要把整个功能搬去做独立 Galaxy 脚本；维护既有 Galaxy 源是另一回事。

## 加载边界

- **本技能拥有：** `Triggers` XML 元素 schema、事件/条件/动作声明形态、参数与 `SubFunctionType` 编码、逐地图战役接线，以及外部触发器编辑纪律。
- **父技能拥有：** `sc2-project-entry` 拥有项目身份、路由与 GUI 优先规则；`sc2-galaxy-scripting` 拥有任何写在 Galaxy 里而不是触发器树里的东西。
- **兄弟技能拥有：** `sc2-bank-system` 拥有 `BankPreLoad`/`BankSave` 背后的 Bank schema 与持久化设计；`sc2-localization` 拥有 `TriggerStrings.txt` 约定；`sc2-tools-validation` 拥有工具；`sc2-editor-handoff` 拥有编辑器保存与接受闸门。

## 核心规则

1. **地图调用模组必须走模组导出的 GUI Action Definition**——地图里的裸 Custom Script 不算数，链接器会静默丢掉模组库。
2. **事件是 `<Element Type="FunctionCall">`，由 `<Event Type="FunctionCall" Id="X"/>` 引用**——`<Event Type="Event"/>` 引用会被静默剥离。
3. **把事件元素紧跟在它所属 Trigger 的收尾 `</Element>` 之后**——位置一漂，代码生成就会静默丢掉 `TriggerAddEvent*`，触发器注册了却永不触发。
4. **每个事件触发器都要有有效动作和一个非空且带名字的 `<Comment>`**，否则模组编辑器可能保存失败。
5. **每个参数都显式给出**，`ValueType` 要准确（gamelink 还要 `ValueGameType`）；绝不假设原生默认值会补齐。
6. **`And` 与 `Or` 使用不同的条件 `SubFunctionType` ID**——混用会在保存时静默剥离 Comparison。
7. **子动作是调用树的子节点，不是 `<Parameter>` 槽位**；绝不让同一个 `FunctionCall` Id 出现在两个父节点下。
8. **外部编辑前先在编辑器里关闭地图**，绝不手改 `MapScript.galaxy`——它会在保存时重新生成。
9. **不要从头到尾读原版 `Triggers` 文件**——用带 `--max-count` 的有界 `rg` 只搜你要的词。
10. **核验生成结果：** 编辑器保存后，在 `MapScript.galaxy` 中确认 `InitTriggers` 与 `TriggerAddEvent*`；出现在触发器树里不等于注册成功。

## 参考文件

- [references/trigger-xml-structure.md](references/trigger-xml-structure.md) — 元素结构、参数、`SubFunctionType` ID、元素标志位与触发器类型→Galaxy 映射。
- [references/event-declarations.md](references/event-declarations.md) — 事件模板、与兄弟元素相邻的要求，以及正确的 XML 形态。
- [references/map-wiring-and-campaign-maps.md](references/map-wiring-and-campaign-maps.md) — 战役接线检查单、GUI action 规则与地图位置。
- [references/external-editing-and-regeneration.md](references/external-editing-and-regeneration.md) — 有界搜索、ID 纪律、`TriggerStrings` 规则与保存/重生成安全。
- [references/triggers-overview.md](references/triggers-overview.md) — 触发器 XML 完整参考；[references/triggers-cheatsheet.md](references/triggers-cheatsheet.md) — 函数/参数 ID 速查。
- [references/triggers-workflow-gotchas.md](references/triggers-workflow-gotchas.md) — 实践中见过的流程级失败；[references/transmission-action.md](references/transmission-action.md) — 传输动作的形态。
- [references/triggers-native-functions.md](references/triggers-native-functions.md) 与 [references/triggers-native/](references/triggers-native/other.md) — native 签名与按字母查询。
- [references/per-map-setup.md](references/per-map-setup.md) — 逐地图设置登记表；`skills/sc2-bank-system/references/bank-system.md` — 持久化设计。
