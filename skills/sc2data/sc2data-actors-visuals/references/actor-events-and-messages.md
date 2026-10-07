# Actor 事件与消息

`<On Terms=... Send=.../>` 事件系统：term 语法、事件名，以及常用 Actor 消息。

## 事件系统 — `<On Terms="..." Send="..."/>`

Actor 是**事件驱动且反向声明**的：由 Actor 自己声明何时激活，而不是由触发它的东西声明。

```xml
<On Terms="EventName.DataId.SubEvent; AnotherTerm; !NegatedTerm" Send="MessageName [Arg1] [Arg2]"/>
```

### Terms 语法

| 语法 | 含义 |
|---|---|
| `ActorCreation` | Actor 已被创建 |
| `ActorOrphan` | Actor 的宿主单位已死亡 |
| `UnitBirth.UnitId` | 该类型的单位出生 |
| `UnitDeath.UnitId` | 该类型的单位死亡 |
| `UnitDeathCustomize` | 单位死亡被触发（更灵活） |
| `WeaponStart.WeaponId.AttackStart` | 武器攻击开始 |
| `WeaponStop.WeaponId.AttackStop` | 武器攻击结束 |
| `Abil.AbilId.SourcePrepStart` | 技能开始施放 |
| `Abil.AbilId.SourceFinish` | 技能施放完成 |
| `Abil.AbilId.SourceStop` | 技能被打断 |
| `AbilMorph.*.Finish` | 任意变形技能完成 |
| `Upgrade.UpgradeId.Add` | 玩家研究了该升级 |
| `Upgrade.UpgradeId.Remove` | 升级被移除/失去 |
| `ValidateUnit UpgradeId` | 检测某升级是否已研究 |
| `IsStatus DeathSuicide 0` | 取反：单位**不是**自杀死亡 |
| `MorphTo UnitId` | 单位变形为该类型 |
| `MorphFrom UnitId` | 单位从该类型变形离开 |

分号 `;` 是 **AND** —— 所有 term 都必须为真。  
`!` 前缀对 term 取反。  
`*` 是匹配单个片段的通配符。

## 常用 Actor 消息（Send）

| 消息 | 作用 |
|---|---|
| `Create` | 创建本 Actor（或引用的 Actor） |
| `Destroy` | 销毁本 Actor |
| `DestroyFinal` | 立即销毁，不播死亡动画 |
| `AnimPlay Attack` | 播放名为 "Attack" 的动画一次 |
| `AnimBracketStart Attack Attack` | 开始循环动画：Opening/Content/Closing 三段 token |
| `AnimBracketStop Attack` | 停止括号式动画 |
| `AnimClear Attack` | 清除正在播放的动画 |
| `AnimGroupApply Superior` | 应用动画组（变体集合） |
| `SetTintColor {255 128 0 255}` | 染色（R G B A，0–255） |
| `SetOpacity 0.5 0` | 设置不透明度（0.0=不透明，1.0=全透明）；0=不混合 |
| `ModelSwap NewModelId` | 运行时替换模型 |
| `GlowStart` | 开始脉冲发光 |
| `GlowStop` | 停止脉冲发光 |
| `HaloStart` | 开始描边光环 |
| `HaloStop` | 停止描边光环 |
| `HaloSetColor {R G B}` | 设置光环颜色 |
| `StatusSet Status 1` | 设置状态标志（如 Cloak） |
| `SetWalkAnimMoveSpeed 3` | 调整行走动画播放速率 |
| `TerrainSquibActivateGroup MyGroup` | 激活一组地形小特效 |
| `SendToRef ::Target Create` | 让目标 Actor 执行 Create |
