# SC2 触发器 XML 速查表

手工验证过的 native ID、参数 ID 与常用 preset 取值。完整的 native 函数查询见 [triggers-native-functions.md](triggers-native-functions.md)。

## 已验证速查表（小集合，带完整参数 ID）

这是人工核对过的子集。下面那份庞大的自动生成表涵盖所有 native，但只给函数 ID——不在本表里的条目，其参数 ID 要直接去 `nativelib.triggerlib` 里查。

### 事件

- `TriggerAddEventMapInit` = Ntve `00000120`（无参数）
- `TriggerAddEventTimePeriodic` = Ntve `6D565EB4`
  - 参数 `4AB413B8` dur（fixed）
  - 参数 `3E8E573F` timeType（preset `00000006`；取值 `00000013`=游戏默认、`00000012`=真实、`EC544EA4`=ai）
- `TriggerAddEventUnitRegion` = Ntve `00000041`（做「任意单位出生」时优先于
  `TriggerAddEventUnitCreated`；传 Entire Map 区域 + 默认 Enter 状态；
  记住规则 #11：所有参数都要显式写出，文档里的默认值 **不会** 自动生效）

### 控制流

- `IfThenElse` = Ntve `00000137`（SubFuncType：`00000003`=if、
  `00000004`=then、`00000005`=else）
- `Return` = Ntve `00000097`（参数 `00000488` val，返回类型）
- `PickEachUnitInGroup` = Ntve `C4DC760C`（参数 `F96B466D` group、
  SubFuncType `9441B8B5` body）
- `Comparison` = Ntve `C439C375`（参数 `ABB380C4` val1 anycompare、
  `51567265` op preset、`4A15EC5F` val2 sameas val1）
  - Op preset `4FAE2F8A`；取值 `1E7A4625`=eq、`500677B2`=ne、
    `684CA9CE`=lt
- `Or` = Ntve `00000133`；条件子类型 `00000001`
- `And` = Ntve `00000132`；条件子类型 `00000002`
- `customscriptaction` = Ntve `00000123`（FlagCustomScript；主体是
  FunctionCall 内的 `<ScriptCode>`——逃生口）

### 变量 / 算术

- `SetVariable` = Ntve `00000136`（参数 `00000219` var anyvariable、
  `00000220` val sameas var）
- `ArithmeticInt` = Ntve `00000128`（参数 `00000205` val1、`00000206`
  op preset `00000027`、`00000207` val2）
- `ArithmeticReal` = Ntve `00000129`（参数 `00000208` val1、`00000209`
  op、`00000210` val2）
  - 共享的 op 取值：`00000085`=+、`00000086`=-、`00000087`=*、`00000088`=/

### 单位

- `UnitGroup`（构造器）= Ntve `00000359`（参数 `00000691` type、
  `00000692` player、`00000693` region、`00000695` unitfilter、
  `00000694` count）
  - 默认值：type `00000231` Any Unit，player `2999701E` Any Player，
    region = `RegionEntireMap` 调用，count `1468BD55`
- `UnitGroupLoopCurrent` = Ntve `19CE733E`（返回 unit——被选中的迭代项）
- `UnitGetType` = Ntve `00000079`（参数 `00000138` u unit；返回
  gamelink Unit）
- `UnitTypeTestAttribute` = Ntve `7A0CB31F`（测的是单位 **类型** 上的属性，
  不是单位实例；这样调用：
  `UnitTypeTestAttribute(UnitGetType(u), attribute)`）

### 玩家

- `PlayerIsEnemy` = Ntve `CA3AB9A9` — 签名 `(source, target,
  relation)`，其中 `relation` 来自 preset `PlayerRelation`
  （`2D9EC843`），取值 `Enemy`/`Ally`/`Neutral`。

### 区域

- `RegionEntireMap` = Ntve `00000356`（无参数，返回 region）
- `RegionPlayableMap` = Ntve `46F2D5B8`

### UI / 文本

- `UIDisplayMessage` = Ntve `0366EE04`（参数 `0D120C33` players
  playergroup、`A4C19F41` messageArea preset、`78A54B52` message text）
  - MessageArea preset `08050A84`；取值 `89CC0A21`=Chat（调试用！）、
    `875889C8`=Subtitle（战役隐藏）、`818777CD`=Objective、
    `A1F3F135`=Directive、`DDCB86EF`=Error、`F282A60A`=Cinematic、
    `18995805`=Debug
- `PlayerGroupAll` = Ntve `00000192`（无参数，返回 playergroup）
- `StringToText` = Ntve `55C79F96`（参数 `30693AA2` val string）
- `IntToText` = Ntve `EAC465A1`（参数 `D125FE97` val int）
- `CombineText` = Ntve `86E40471`（参数 `174E8B8C` text1、`BD48D330`
  text2）

### 对话框（高层）

- `CreateDialogItemImage` = Ntve `CFF28424` — 一次调用即创建
  Image 控件 **并且** 同时设置尺寸/位置/贴图/着色/混合模式。
  12 个参数：dialog、w、h、anchor preset、offX、offY、tooltip、
  filepath、imageType preset、tiled bool、tintColor、blendMode preset。
  强烈优先于裸的 `DialogControlCreate(Image) +
  SetPropertyAsImage` 链。
- Image filepath 参数的 XML：`<Value>Assets\Textures\btn-unit-terran-marine.dds</Value><ValueType Type="filepath"/><ValueTypeInfo Value="5"/>` —— filepath 参数必须带 `ValueTypeInfo Value="5"`。
- 自带单位头像贴图：`Assets\Textures\btn-{unit|building|ability|upgrade}-{terran|zerg|protoss}-{name}.dds`（例如 `btn-unit-terran-marine.dds`）。所有 SC2 地图都可用。

### 逐卡对话框设计

卡片式对话框：**每张卡** 建一个 Dialog（不要用一个大 Dialog 套子控件）。
典型尺寸约 220×340，锚在屏幕 Bottom，Y 偏移取正使其位于命令卡区域上方。
跳过 `DialogSetImageVisible`——SC2 对话框默认框体 **就是** 卡片边框。头像用
`CreateDialogItemImage` 放在 Top-center；Bottom-center 放一个带富文本（彩色
名字 + 描述）的 Button 作点击处理器。常见错误是用单个 Dialog 加 N 个按钮加
N 个背景 Image 控件当「卡框」——没有贴图的 Image 控件会渲染成惨白色，很难看。
逐卡 Dialog 免费给你正确的框体。


---

## 常用 preset（枚举式参数取值）

nativelib 里有 447 个 preset，合计 2,994 个具名取值。完整列表太长，无法内联，
但每个 preset 都在 `nativelib.triggerlib` 的 `<Element Type="Preset" Id="…">` 下
——它的 `<Item Type="PresetValue" Id="…"/>` 子元素列出各取值。Id 相同的
`PresetValue` 元素带 `<Identifier>` 名字与 `<Value>` 字面量。

最常伸手去取的 preset：

| Preset | ID | 用在哪 | 常用取值（名字 = Id） |
|---|---|---|---|
| MessageArea | `08050A84` | `UIDisplayMessage` | Chat=`89CC0A21`、Subtitle=`875889C8`、Objective=`818777CD`、Directive=`A1F3F135`、Error=`DDCB86EF`、Cinematic=`F282A60A`、Debug=`18995805` |
| TimeType | `00000006` | `TriggerAddEventTimePeriodic`、计时器 | Game=`00000013`（默认）、Real=`00000012`、AI=`EC544EA4` |
| ComparisonOp | `4FAE2F8A` | `Comparison` | Eq=`1E7A4625`、Ne=`500677B2`、Lt=`684CA9CE`、Gt、Le、Ge（按名字查） |
| ArithmeticOp | `00000027` | `ArithmeticInt`、`ArithmeticReal` | +=`00000085`、-=`00000086`、*=`00000087`、/=`00000088` |
| PlayerRelation | `2D9EC843` | `PlayerIsEnemy` | Enemy、Ally、Neutral（在 nativelib 里按名字查） |

要找某个 preset 的完整取值列表，在 nativelib 里 grep 该 preset 的 Id，
顺着 `<Item Type="PresetValue" Id="…"/>` 引用走。


---
