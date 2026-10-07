# 生产与命令卡 XML 模式

## 单位/建筑费用修改

`Operation="Multiply"` 对 `CostResource` 字段 **不** 生效。用 `Add`/`Subtract`/`Set`：

```xml
<EffectArray Operation="Subtract" Reference="Unit,Roach,CostResource[Minerals]" Value="38"/>
```

### 批量训练的单位（例如跳虫 ×2）

游戏按 `CostResource × count` 扣费。改单位还是改技能，取决于该技能 InfoArray 是否已有 `Resource` 条目：

- **InfoArray 已有 `Resource` 条目** → 从技能资源里 `Subtract`
- **InfoArray 没有 `Resource` 条目** → 把单位费用清零 **并且** `Set` 技能资源

```xml
<!-- Zergling: zero unit cost first, then set ability batch cost -->
<EffectArray Operation="Subtract" Reference="Unit,Zergling,CostResource[Minerals]" Value="25"/>
<EffectArray Operation="Set" Reference="Abil,LarvaTrain,InfoArray[Train2].Resource[Minerals]" Value="25"/>
```

关键技能→单位映射：
- `LarvaTrain InfoArray[Train2]` → 跳虫 ×2（原本没有 Resource——单位清零 + Set 技能）

---

## Train/Build 按钮的命令卡费用显示

优先用原生费用显示，不要把水晶、瓦斯、建造时间或人口写进按钮提示框文本。它与暴雪的原生战役表现一致，并且在升级改动单位费用后仍然准确。

原生费用条显示（水晶图标 + 费用、瓦斯图标 + 费用、人口图标，以及建造/冷却时间）依赖 XML 里三个同步的层：

1. **技能层（`AbilData.xml`）**：
   - `CAbilTrain` 与 `CAbilWarpTrain` 上每个 `InfoArray` 条目都必须有显式的 `<Resource index="Minerals" value="..."/>` 与 `<Resource index="Vespene" value="..."/>`（> 0 时），并与单位费用一致。
   - 对批量单位（例如跳虫 ×3 或平民 ×2），`InfoArray.Resource` 反映批量总费用。

2. **按钮层（`ButtonData.xml`）**：
   - 自定义单位的生产按钮需要 `<Universal value="1"/>`，以告知 SC2 命令卡引擎为提示框与快捷键绑定通用上下文。
   - 打开提示框标志位：`<TooltipFlags index="ShowResources" value="1"/>`、`<TooltipFlags index="ShowSupply" value="1"/>`、`<TooltipFlags index="ShowTime" value="1"/>` 与 `<TooltipFlags index="ShowCooldown" value="1"/>`。

3. **命令卡与单位层（`UnitData.xml`）**：
   - **带索引的卡边界**：在工人建造子菜单（`PBl1`、`TBl1`、`ZBl1`）与生产者卡上，命令按钮必须接在单位权威的带索引槽位（0..11）内。在原生数组之外追加重复或不带索引的 `<LayoutButtons>` 条目会叠在槽位坐标上，破坏引擎的建造/训练技能费用头解析。
   - **费用定义**：被产出的单位必须有权威的 `<CostResource index="Minerals" value="..."/>`、`<CostResource index="Vespene" value="..."/>`、`<Food value="..."/>` 与 `<CostCategory value="Army"/>`（或 `Technology`）。

```xml
<!-- AbilData.xml -->
<CAbilTrain id="MyGatewayTrain">
    <InfoArray index="Train6" Time="38">
        <Resource index="Minerals" value="100"/>
        <Button DefaultButtonFace="MyCommando" Requirements="MyTrainCommando"/>
        <Unit value="MyCommando"/>
    </InfoArray>
</CAbilTrain>

<!-- ButtonData.xml -->
<CButton id="MyCommando">
    <Icon value="Assets\Textures\btn-unit-terran-marine.dds"/>
    <AlertIcon value="Assets\Textures\btn-unit-terran-marine.dds"/>
    <EditorCategories value="Race:Terran"/>
    <Universal value="1"/>
    <TooltipFlags index="ShowResources" value="1"/>
    <TooltipFlags index="ShowSupply" value="1"/>
    <TooltipFlags index="ShowTime" value="1"/>
    <TooltipFlags index="ShowCooldown" value="1"/>
</CButton>
```

自定义训练/建造条目的检查单：

- 生产者的 `AbilArray` 里有该技能。
- 技能槽位有 `InfoArray[TrainX].Unit` 或 `InfoArray[BuildX].Unit`，并带显式的 `Resource` 费用。
- 生产者的命令卡按钮是 `Type="AbilCmd"`，且 `AbilCmd` 与该 InfoArray 命令完全匹配。
- 对六个星灵生产建筑（`Gateway`、`WarpGate`、`RoboticsFacility`、`RoboticsFacilityWarp`、`Stargate`、`StargateWarp`），每个阵营训练按钮都必须在技能按钮（`InfoArray/Button Requirements`）和命令卡 `LayoutButtons` 行上都有既存的单位/阵营需求。叠放的按钮共享命令卡槽位，所以缺失或无效的需求可能一直保持可用，挡住本应在下方的按钮。
- 恢复到自己覆盖过的生产者命令卡上的工具型变形按钮，例如 Gateway <-> Warp Gate，也需要显式的 `Type="AbilCmd"` 与固定的 `Row`/`Column`；按索引屏蔽或替换卡条目之后，继承/默认值不再可靠。
- 在继承的带索引卡上新增建筑命令卡条目，必须用显式的 `LayoutButtons index="N"` 占一个空槽。夹在带索引的继承槽位之间的无索引 `LayoutButtons` 行会在编辑器保存时被丢掉。
- 工人建造卡（`PBl1`、`TBl1`、`ZBl1`）必须直接改继承的槽位索引（例如探测机上的 Gateway 用 `index="3"`），而不是追加无索引行。
- 命令卡的 `Face` 与技能的 `DefaultButtonFace` 使用同一个按钮 ID，并带 `<Universal value="1"/>`。
- 被产出的单位有 `CostResource[Minerals]` / `CostResource[Vespene]`，且不隐藏资源（`FlagArray[ShowResources]=1`）。
- 战役训练命令在该单位平时被战役科技锁住时，还需要 `TechTreeUnitAllow(player, "UnitID", true)`。

`InfoArray.Resource` **不是** 无害的显示字段。它是在被产出单位费用之上的累加费用调整。确认既有技能数据之后，只在批量单位、退款或变形差值这类特殊情况下使用它。

### 继承的 InfoArray 字段与编辑器规范化

带索引的技能条目会从父级技能的同一索引原样继承未改动的子字段。子级可以替换 `InfoArray[Build2].Unit` 及其按钮，同时保留父级的 `Time`。当本地字段与继承值相同时，SC2 编辑器保存时可能删掉冗余的序列化，而不改变 catalog 的有效值。

例如子技能可以把 `Build2.Unit=Pylon` 换成自定义建筑，而 `ProtossBuild,Build2` 提供 `Time="25"`。子级不需要本地时间字段，编辑器也可能删掉重复的子级 `Time="25"`；这是规范化，不是时间归零的回归。恢复被剪掉的带索引字段之前，先检查确切的父级索引并比对有效值。

### 拆分折跃训练的充能

当生产者因为一个技能无法引用所有被产出单位而需要多个 `CAbilWarpTrain` 时，该生产者上所有拆分出的技能必须共用同一个单位本地充能链接。不要给每个拆分技能各自的自链接，否则同一建筑可以从技能 A 折跃一个单位、立刻又从技能 B 折跃另一个。

当前实际拆分组：

| 生产者 | 拆分的技能 | 共享充能链接 |
|---|---|---|
| `WarpGate` | `MyWarpGateTrainA`、`MyWarpGateTrainB`、`MyWarpGateTrainC` | `MyWarpGateCharge` |
| `RoboticsFacilityWarp` | `MyRoboticsWarpTrainA`、`MyRoboticsWarpTrainB` | `MyRoboticsWarpCharge` |
| `StargateWarp` | `MyStargateWarpTrainA`、`MyStargateWarpTrainB` | `MyStargateWarpCharge` |

在这些充能块上用 `<Location value="Unit"/>`，让冷却共享按建筑实例计算，而不是所有建筑全局共享。

`CAbilWarpTrain` **不** 像 `CAbilTrain` 那样支持批量产出。SC2 编辑器会把折跃训练行规范化回单个 `InfoArray.Unit` 值，并删掉重复的子级 `<Unit value="..."/>` 条目。保持 XML 单单位；需要双/三产出时，在折跃单位到达 `c_unitProgressStageComplete` 时用项目脚本实现。不要从通用的单位创建事件或 `EventUnitCreatedAbil()` 走这条路径；该技能链接对现场折跃建造不可靠。

### 折跃门产出的非星灵单位

当自定义 `CAbilWarpTrain` 通过星灵折跃门/折跃机械台/折跃星门桥接产出人族、异虫或自定义单位时，被产出的 `CUnit` 必须有：

```xml
<AbilArray Link="Warpable"/>
```

普通传送门/机械台/星门训练没有它也能工作，所以务必同时测对应的折跃生产者。缺 `Warpable` 会创建正确的自定义单位 Actor，外加一个多余的 `GenericUnitFallback` 球体 Actor，并在控制台报出「Unable to create unit actor」与「More than one CActorUnit persisting in the same unit scope」。

单位 Actor 还需要折跃/建造视觉。在数据编辑器里它显示在单位 Actor 的视觉/建造属性下；在 XML 里是 `CActorUnit.BuildModel`。对通用的非星灵桥接单位，用：

```xml
<BuildModel value="ProtossGenericWarpInOut"/>
```

打这个字段之前，先把被产出单位映射到它真正的 Actor ID。有些依赖 Actor 与它们的单位 ID 不一致；例如 `Spectre` 用 `SpecterUnit`，`DuskWing` 用 `DuskWingBanshee`。

如果被产出单位用的是从具名单位 Actor 继承的自定义 `CActorUnit`，也要用带索引的出生/复活/建造覆盖，而不是追加新的 `UnitBirth.CustomUnit` 行。见下面的「从具名单位 Actor 派生 Actor 变体」。

创建本地单位 Actor 垫片时，先核验父 Actor 的 catalog 类。有些看起来像单位的暴雪 Actor（包括 `HotSHunter` 与 `Reaper`）导出为 `CActorMissile`；本地 `CActorUnit` 不能从它们继承。用兼容的通用单位父级，如 `GenericUnitBase` 或 `GenericBurrowerStandard`，然后显式复制所需的 `Model`、`PlacementModel`、`PortraitModel`、死亡模型与本地单位事件。

### 星灵折跃训练视觉

当自定义星灵单位通过 `CAbilWarpTrain` 产出时，自定义 `CActorUnit.BuildModel` 优先用父级单位专属的一次性折跃入场模型，而不是通用桥接模型。例如：

| 自定义 Actor | BuildModel |
|---|---|
| `MyZealotVariant` | `ZealotAiurWarpIn` |
| `MySentryVariant` | `SentryAiurWarpIn` |
| `MyImmortalVariant` | `ImmortalAiurWarpIn` |
| `MyDarkTemplarVariant` | `DarkTemplarAiurWarpIn` |

选模型之前以活动依赖数据为真源。有些可训练的自定义单位基于平时不从折跃生产者训练出来的依赖单位，所以它们需要最接近的、活动依赖里的星灵一次性模型。

### 现场折跃生产的人族空投舱视觉

当人族阵营地面单位通过 `CAbilWarpTrain`（折跃门/折跃机械台）产出时，用战役的 **轨道打击 / 兵营空投训练** 表现，而不是只用星灵折跃球。

`DataEditorXML/Liberty Campaign Effects.txt` 与 `Liberty Campaign Actors.txt` 里的参考链：

| 部件 | ID | 作用 |
|---|---|---|
| 效果集 | `DropTrainSet` | `MakePrecursor` 隐藏单位，然后启动 `DropTrain` |
| 持续效果 | `DropTrain` | 在被产出单位上持续 2.3 秒；`FinalEffect` = `RemovePrecursor` |
| Actor | `BarracksDropPod` | 监听 `Effect.DropTrain.Start`；播放下落的空投舱动画 |
| 模型 | `BarracksDropPod` | `DropPodFalling.m3`（与战役 `TerranDropPod` 同一资源族） |
| 音效 | `BarracksDropPodFall`、`BarracksDropPodUnload` | 与 `DropTrain` 起止绑定的下落/卸货音频 |
| 行为 | `Precursor` | 空投舱序列运行期间的 `NoDraw` |

**不要假设 `Effect` 在 `CAbilWarpTrain` 上有效。** 轨道打击升级是给 `CAbilTrain`（`BarracksTrain`）的 `InfoArray` 条目打上 `Effect="DropTrainSet"`。`CAbilWarpTrain` 不接受该字段；编辑器会告警并忽略该属性。

**不要用 `ProtossGenericWarpInOut` 行为来门控空投舱。** 现场折跃走的是单位建造加 `CActorUnit.BuildModel=ProtossGenericWarpInOut`，它在 `UnitConstruction.{unit}.Start` 时创建星灵折跃 Actor。这与 `ProtossGenericWarpInOut` 行为增益是两回事。检查该行为层数的验证器会在折跃训练建造期间失败。

**现场折跃只用 `UnderConstruction` 门控。** 建筑上的传送门 `CAbilTrain` 生产应保留正常的星灵折跃入场，且不得生成空投舱。用 `CValidatorUnitFilters` 配 `Filters value="UnderConstruction;-"`。

项目接线示例：

```xml
<!-- Ground unit: passive trigger behavior -->
<CBehaviorBuff id="MyTerranWarpDropPod">
    <InfoFlags index="Hidden" value="1"/>
    <Requirements value="HaveMyTerranFaction"/>
    <InitialEffect value="MyTerranWarpDropPodSet"/>
</CBehaviorBuff>

<!-- Short delay so construction state exists before the execute step -->
<CEffectSet id="MyTerranWarpDropPodSet">
    <EffectArray value="MyTerranWarpDropPodApplyDelay"/>
</CEffectSet>
<CEffectApplyBehavior id="MyTerranWarpDropPodApplyDelay">
    <Behavior value="MyTerranWarpDropPodDelay"/>
</CEffectApplyBehavior>
<CBehaviorBuff id="MyTerranWarpDropPodDelay">
    <Duration value="0.125"/>
    <ExpireEffect value="MyTerranWarpDropPodExecute"/>
</CBehaviorBuff>
<CEffectSet id="MyTerranWarpDropPodExecute">
    <EffectArray value="DropTrainSet"/>
    <ValidatorArray value="MyTerranWarpFieldWarpIn"/>
</CEffectSet>

<CValidatorUnitFilters id="MyTerranWarpFieldWarpIn">
    <Filters value="UnderConstruction;-"/>
</CValidatorUnitFilters>
```

只通过 `BehaviorArray Link` 把 `MyTerranWarpDropPod` 挂到人族地面单位上，排除空中单位。把 `My` 前缀与需求替换成项目配置的等价物。

**预载整条空投舱链。** 只把 `Effect=DropTrainSet` 写进 `PreloadAssetDB.txt` 是不够的。如果 `MakePrecursor` 跑了但 `BarracksDropPod` 没有预载，单位会一直不可见直到 `RemovePrecursor`，而且空投舱不会出现。还要预载：

- Actor：`BarracksDropPod`、`BarracksDropPodFall`、`BarracksDropPodUnload`
- 模型：`BarracksDropPod`
- 效果：`DropTrain`、`MakePrecursor`、`RemovePrecursor`
- 行为：`Precursor`

**要避免的失败做法：**

- 用 `InitialEffect` 给 `ProtossGenericWarpInOut` 打补丁——引擎施加的折跃建造不会可靠地触发该行为的初始效果。
- 在所有人族单位 Actor 上加 `UnitConstruction.*.Start` 通配 Actor 事件——会在星灵建筑折跃入场时造成人族血条/轮廓重叠，而且雇佣兵折跃入场时并不可靠地显示空投舱。
- `DropTrainSet` 不预载 Actor/模型——通过 `MakePrecursor` 隐藏了单位，但看不到空投舱。

某些战役雇佣单位已经通过 `MercGroundDrop` 与单位专属的 `*DropModel` Actor（`MercDropModelBase` → `DropPodFalling`）有另一套空投表现。项目也可以改用上面的兵营/轨道打击链，给人族地面单位共享的空投舱视觉，而不必逐单位接 Actor。

### 现场折跃生产的异虫破土视觉

当异虫地面单位通过 `CAbilWarpTrain` 产出时，用一个纯表现的破土 Actor，而不是创建临时钻地单位。创建真正的钻地单位需要下破土命令并清理重复的游戏性单位；更安全的做法是让折跃训练出的单位保持权威，只创建一个瞬时的 `CActorModel`。

项目接线示例：

```xml
<CBehaviorBuff id="MyZergWarpUnburrowVisual">
    <InfoFlags index="Hidden" value="1"/>
    <Requirements value="HaveMyZergFaction"/>
    <InitialEffect value="MyZergWarpUnburrowVisualSet"/>
</CBehaviorBuff>

<CEffectApplyBehavior id="MyZergWarpUnburrowVisualExecute">
    <Behavior value="MyZergWarpUnburrowVisualMarker"/>
    <ValidatorArray value="MyTerranWarpFieldWarpIn"/>
</CEffectApplyBehavior>

<CActorModel id="MyZergWarpUnburrowVisualSwarmling" parent="ModelAdditionNoAnims">
    <Model value="HotSSwarmling"/>
    <Inherits index="Opacity" value="0"/>
    <Inherits index="Visibility" value="0"/>
    <On Terms="Behavior.MyZergWarpUnburrowVisualMarker.On; ValidateUnit CasterHotSSwarmling" Send="Create"/>
    <On Terms="ActorCreation" Send="AnimBracketStart Burrow Burrow IGNORE Unburrow ClosingFull,OpeningPlayForever,Instant"/>
    <On Terms="ActorCreation" Send="TimerSet 0.062500 Unburrow"/>
    <On Terms="TimerExpired; TimerName Unburrow" Send="AnimBracketStop Burrow"/>
    <On Terms="TimerExpired; TimerName Unburrow" Send="TimerSet 2.000000 Destroy"/>
    <On Terms="TimerExpired; TimerName Destroy" Send="Destroy"/>
</CActorModel>
```

只把 `MyZergWarpUnburrowVisual` 挂到有已知钻地对应体、且单位模型支持 Burrow/Unburrow 动画括号的异虫地面单位上。逐个确认所选单位的模型支持该动画括号。

在自动标记施加上保留 `UnderConstruction` 验证器，这样传送门/机械台/星门的常规建筑内训练不会显示现场折跃破土 Actor。如果地图通过 Galaxy 创建单位并立刻替换成异虫单位，优先创建真正的钻地形态并从 Galaxy 下它原生的破土命令。这与引擎正常的钻地/破土 Actor 路径一致，也避免依赖一个可能在创建单位的 Actor 作用域就绪之前就触发的纯表现效果。

已确认的脚本化入场示例：

- `RoachCorpser` -> 创建 `RoachCorpserBurrowed`，下 `BurrowHotSCorpserUp`。
- 自定义蟑螂变体 -> 创建它的自定义钻地变体并下它的自定义破土命令。
- 自定义刺蛇变体 -> 创建它的自定义钻地变体并下它的自定义破土命令。

当真正的钻地形态或破土命令不可用时，显式的脚本延时行为仍是兜底。走 Actor 标记路径时，把脚本延时/执行/标记行连同所有 Actor/模型行一起预载，并用 `WhichUnit Value="Caster"` 保持 `CValidatorUnitType` 验证器显式。

### 静态叠放的战役生产按钮

对虚空之遗战役生产建筑，优先用静态 XML 按钮叠放，而不是运行时改命令卡：

- 把每个阵营的训练按钮放在与它原版锚点相同的 Row/Column 上。
- 给每个叠放按钮一个互斥需求。一个叠放里应当恰好有一个按钮对某玩家满足 `Show`/`Use`。
- 把按钮加到该建筑实际打开的那一层卡上。战役建筑常常同时有 `CardLayouts index="0"` 和第二层战役卡；先 grep 原版单位，再打到与相关 `*TrainAI` 按钮相同的层。
- 如果继承的原版按钮与项目按钮冲突，用数据需求隐藏它们，让它们在项目升级被授予后变为假。生产 UI 避免用 `CatalogFieldValueSet` 改写命令卡。
- 把名字/提示框锚定在真正的 `CButton` 与训练技能数据上。只存在于运行时的字符串很容易被 SC2 编辑器剪掉。
- 自定义 `CButton` 行留在 `ButtonData.xml`，不要放进宽泛的 `GameData.xml`。如果编辑器保存为被挪进 `GameData.xml` 的按钮重新生成了带类型的 `ButtonData.xml` 行，把它当规范化信号并删掉重复条目。
- 让生产按钮图标使用编辑器规范化后的纯图标形态（`Icon`、`AlertIcon`、`EditorCategories`，加上冷却/自动施法这类非费用显示标志位）**并且**使用直接指向 `Button/Name/{id}` 与 `Button/Tooltip/{id}` 的 `Name`/`Tooltip` 字段。加完新的训练/建造 `CButton` 行后跑 `python tools/audit-gamestrings-anchors.py`；编辑器保存后用 `--fill` 恢复仍存在于 `ObjectStrings.txt` 里的键。
- 如果 `GameStrings.txt` 丢了一个仍被 `Name`、`Tooltip` 或 `Description` XML 引用的键，就恢复该键或删掉过时的 XML 引用。如果丢的键只是辅助/编辑器元数据，优先放 `ObjectStrings.txt` 或不设面向玩家的锚点。

对工蜂建造子菜单，加按钮时用原版子菜单 ID：

```xml
<CardLayouts index="1" CardId="ZBl1">...</CardLayouts>
<CardLayouts index="2" CardId="ZBl2">...</CardLayouts>
```

继承的子菜单索引与原生 `CardId` 都要用。`CardId` 是子菜单目标（`ZBl1` / `ZBl2`），而 `index` 确保模组补丁覆盖继承的工蜂子菜单，而不是追加一张命令卡可能打不开的重复卡。

---
