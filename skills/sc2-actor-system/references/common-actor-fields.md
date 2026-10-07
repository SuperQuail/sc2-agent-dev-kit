# 常用 Actor 字段

| 字段 | 说明 |
|---|---|
| Aliases | 备用引用名。`_Unit` 是标准单位 Actor 别名，其他 Actor 用它寻址该单位而不必知道其确切 ID。 |
| Copy Source | 把另一个 Actor 设为代理父级。子 Actor 会从 Copy Source 继承，前提是相关属性列在 `Accepted Property Transfers`（源）与 `Inherited Properties`（子）中。 |
| Filter | 把 Actor 可见性限制到 Ally / Enemy / Neutral / Self 组。 |
| Filter Player | 按玩家覆盖可见性。 |
| Flags — Suppress Creation Errors | 压掉资源缺失错误。用于有意设计成在特定条件下创建失败的 Actor。 |
| Fog Visibility | Dimmed / Hidden / Snapshot / Visible——控制战争迷雾下 Actor 的表现。 |
| Events | Actor 事件列表（事件 + term + 消息 三元组）。 |
| Macros | 可复用的宏事件集合，可挂到多个 Actor 上。 |
| Remove | 从父级链上显式移除不想要的继承事件。 |
| Terms | 短路创建条件——与该创建事件的 term 等价；两者都设置时会覆盖后者。 |
| Host Supporter | 一个支援用 Actor；它死亡时会向宿主 Actor 发送 `SupporterDestruction`。 |
| Accepted Property Transfers | 本 Actor 向子 Actor 传递哪些属性（模型缩放、不透明度、队伍颜色、可见性、贴花、贴图、迷雾颜色、位置、朝向）。 |
| Inherit Type | `Once`（创建时继承一次）、`Continuous`（动态更新）、`None`。 |
| Inherited Properties | 本 Actor 创建时从宿主继承哪些属性。必须与宿主的 `Accepted Property Transfers` 对应。 |
