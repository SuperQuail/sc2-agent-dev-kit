# Catalog 规则与继承 XML 模式

## 改源之前先取有效值证据

改数值前，先去证明有效值，而不是从叶子 XML 行推断。记录本地单位或 catalog 对象、它的 `parent`、精确字段/索引处的继承值，以及打算做的本地覆盖。当本地字段缺失或在保存时被移除，要查活动依赖链与编辑器规范化。复制进子行的取值可能是冗余的；只要父级提供了预期取值，子级缺字段可能仍然是对的。

对 UI 文本，先把观察到的界面归类为 **世界悬停**、**选择面板**、**命令卡按钮**、**提示框** 或 **编辑器文本**。然后把该界面追到它所属的 catalog 对象与生效的本地化锚点。在 UI 面与消费它的 XML 字段被识别出来之前，不要改 `GameStrings.txt` 键；同理，不要为了修由按钮或提示框拥有的文本，去改单位名或 Actor 高亮字段。

这份证据是 [问题生命周期](../../sc2-editor-handoff/references/testing-feedback-workflow.md#6-stage-issue-lifecycle) 中 `root cause confirmed` 状态的前置条件。做源修复之前先把它写进问题记录。

## 任务局部覆盖与变体隔离

只有当某个场景、目标、Boss 或地形交互是该任务独有时，才用地图局部的 catalog 覆盖。保持它很小，并在实现备注里写明所属地图/机制。可复用的阵营机制应当放进项目模组，带配置的项目前缀。

不要仅为造一个阵营变体就去覆盖原版 ID：那会改到所有解析该 ID 的依赖消费者，可能破坏项目的替换隔离。可选阵营内容要用带父级、带项目前缀的变体。同 ID 覆盖只留给明确有意、战役范围的改动，且仍需要有效值证据与受影响地图的编辑器测试。

## 足迹与菌毯放置

改 `CUnit.Footprint` 或 `CUnit.PlacementFootprint` 之前，先看 `DataEditorXML/* Footprints.txt`。足迹不只是一个 ID：暴雪的足迹可以带 `Check`、`Place`、`Pathing` 层以及 `Shape`。做菌毯友好建筑的足迹时，保留原足迹的尺寸/形状/各层，只清掉菌毯阻挡。

已确认有用的参考：

- `Liberty Mod Footprints.txt`：`Footprint2x2IgnoreCreepContour` 与 `Footprint3x3IgnoreCreepContour` 是原生的编辑器示例。它们用 `parent="Footprint2x2"` / `parent="Footprint3x3"`，清掉 `Negative[Creep]`，保留 `Negative[Fogged]`，并加上轮廓/寻路形状数据。
- `Footprint5x5DropOff` 与 `Footprint5x5Contour` 保留了城镇中心资源/卸货行为；克隆它们时保留 `NearResources` / `DropOff` 集合。
- 气泉建筑同时用 `FootprintGeyserRoundedBuilt` 与 `Footprint3x3CappedGeyser`；做菌毯友好变体时保留它们的资源邻接/封顶气泉布局。

---

## RequirementNodeData 模式

```xml
<!-- CRequirementNot child field: -->
<OperandArray index="0" value="NodeID"/>   <!-- NOT NodeArray or Value -->

<!-- CRequirement: -->
<NodeArray index="Show" Link="NodeID"/>
```

**新增 `CRequirement` + `CRequirementCountUpgrade` 时**，一律往 `ObjectStrings.txt` 加 4 行：
```
Requirement/EditorPrefix/HaveMyUpgrade=My
Requirement/Name/HaveMyUpgrade=Have MyUpgrade
RequirementNode/EditorPrefix/CountMyUpgrade=My
RequirementNode/Name/CountMyUpgrade=Count My Upgrade
```

---

## 已存在的验证器（不要重新定义——按名字从 Void.SC2Mod 引用）

- `SourceIsNotStationary` — 单位移动速度 > 0

---

## 组合验证器

`CValidatorCombine` 通过 catalog 默认记录取 `Type="Or"`。简单的 OR 验证器就省略显式的 `<Type value="Or"/>`；SC2 编辑器保存时可能把它规范化掉。只有当所有子验证器都必须通过时才显式写 `<Type value="And"/>`，取反组合结果用 `<Negate value="1"/>` 而不是 `Type="Not"`。

---

## CBehaviorBuff 的 WeaponRange

```xml
<CBehaviorBuff.Modification WeaponRange="1"/>  <!-- adds +1 to ALL weapon ranges -->
```

---

## 跨模组数组覆盖

当本地 catalog 行与另一个已加载模组里的行使用同一个 ID 时，数组覆盖要显式给出索引。后面某个 `CEffectSet` 里不带索引的 `EffectArray` 可能追加到前一个模组的数组上，而不是替换它的元素。尤其是 `VortexEffectGA` 同时出现在 `SCORE-Golden.SC2Mod` 与 `Heros.SC2Mod` 里；它在 Heros 里的五个效果与两个验证器必须从索引 `0` 开始编号，这样合并加载后不会超出该效果数组支持的位置。

## 自动施法配置

要让技能自动施法（AI 控制施放），在数据编辑器 → Abilities 标签页配置：

1. **启用自动施法：** 在 `Stats - Flags` 里勾 `Auto Cast`。如果希望默认开启，勾 `Auto Cast (On)`。
2. **目标过滤器：** 设 `Auto Cast Filters` 限制技能对谁施放（例如 `Enemy`、`Visible`、`Biological`）。过滤器只检查通用属性（同盟关系、是否建筑）——不检查具体单位类型。
3. **条件分组（验证器）：** 过滤器表达不了的规则用 `Auto Cast Validators`——例如「只有射程内有 3 个以上敌人时才施放」。建一个 `UnitCount` 或 `UnitFilter` 验证器并挂到这里。

对范围效果，在技能的搜索区域效果里设置半径，以定义什么算一个「组」。

近期 bug 模式：对自定义战役技能来说，光靠过滤器通常太宽。如果自动施法可能对错误的一类单位抽蓝/治疗/加增益，就为确切的目标集合加验证器；技能有脚本化副作用时，保留一个 Galaxy 处理器守卫。在 Galaxy 里处理施法时，优先用技能事件里的 `EventUnit()` 与 `EventUnitTargetUnit()`，而不是「最近的单位」推断。

---

## XML 注释卫生

SC2 XML 是严格 XML：注释正文里不要出现 `--`。像 `<!-- foo -- bar -->` 这样的注释会让编辑器拒绝该文件，即使数据本身没问题。优先用简短的 ASCII 注释；XML 条目本身够明显时就别写注释。

---

## EditorCategories 必须匹配活动依赖的标签页

不要盲目从非活动导出里复制 `EditorCategories`。那些导出可能含活动依赖栈不认识的编辑器分类桶。

设置或批量规范化分类之前，先在当前 `DataEditorXML/` 转储里 grep 同一个 catalog/标签页，只用在那里被证明过的取值。常见的活动分类包括种族桶如 `Race:Terran`、`Race:Protoss`、`Race:Zerg`、`Race:Neutral`；技能/行为分类如 `Race:Zerg,AbilityorEffectType:Units`；升级分类如 `Race:Zerg,UpgradeType:Talents`；以及基于 `ObjectType` / `ObjectFamily` 的单位分类如 `ObjectType:Unit,ObjectFamily:Campaign`。

单位数据不使用活动转储里那些导入的 StarCoop 阵营族。对导入的战役/阵营单位，把不受支持的族如 `FactionMecha`、`FactionPrimal`、`FactionInfested`、`FactionCovertOps`、`FactionRaider`、`FactionPurifier`、`FactionTaldarim` 收拢到一个合法的活动族如 `ObjectFamily:Campaign`，同时保留 `ObjectType`（`Unit`、`Structure`、`Projectile`、`Hero` 或 `Other`）。

音效分类在本仓库是个例外，因为当前 `DataEditorXML/` 参考集不含音效转储。把用户/编辑器保存的 `SoundData.xml` 分类当作当前规范信号，除非以后加入了匹配的活动音效转储。

---

## 编辑器保存导致的拆分 catalog 重复

当 SC2 编辑器把某条标准 catalog 条目保存进 `EffectData.xml` 这类专用文件时，要把 `GameData.xml` 或旁路 catalog 文件里同一 `(catalog type, id)` 的旧定义删掉。加载器把 `Base.SC2Data/GameData/` 下的所有文件当成同一个 catalog 命名空间，所以 `GameData.xml` 与 `EffectData.xml` 里都写 `CEffectSet id="Example"` 会产生 `Unable to create duplicate entry` 告警，即使两行完全相同。

除非已经过时，保留引用、预载行与本地化锚点。每个 `(catalog type, id)` 只应留下一个 XML 定义。

自写的阵营数据优先放进带类型的 catalog 文件。混合旁路 catalog 便于手改，但每行都住在自己的标准 catalog 文件里时，编辑器/运行时更可靠：`CUnit` 在 `UnitData.xml`，`CButton` 在 `ButtonData.xml`，`CAbil*` 在 `AbilData.xml`，`CBehavior*` 在 `BehaviorData.xml`，`CEffect*` 在 `EffectData.xml`，`CRequirement` 在 `RequirementData.xml`，需求节点在 `RequirementNodeData.xml`，验证器在 `ValidatorData.xml`，模型在 `ModelData.xml`，mover 在 `MoverData.xml`，升级在 `UpgradeData.xml`。没有编辑器往返方面的理由，不要引入新的自定义 catalog 文件名。

拆完旁路 catalog 之后，在 SC2 编辑器里把迁移过的条目打开再关闭一次。编辑器可能剥掉本地化已提供的冗余 `CButton` 名字/提示框字段，往 `PreloadAssetDB.txt` 加单位/效果行，并暴露出本地 XML 解析查不到的坏图标路径。只要模组重新打开时没有 XML 告警，就接受这些规范化。

---

## 从具名单位 Actor 派生 Actor 变体（parent="SomeUnitActor"）

创建从某个具体单位 Actor 继承视觉/音效的单位 Actor 时（例如 `parent="Ravager"`），`##unitName##` 宏 **不会** 解析到叶子 Actor 自己的 `unitName`。它解析到 **父级链上最近的、定义了 `unitName` 的祖先**。对 `parent="Ravager"`（它的 `unitName="Ravager"`），所有继承事件仍然写着 `UnitBirth.Ravager`、`UnitRevive.Ravager` 等——即使子级定义了 `unitName="MyMyVariant"`。

**修法：** 在子 Actor 里按索引覆盖每一个与单位名相关的事件。

告警示例：`Supplicant` 曾用 `parent="Zealot"` 并追加 `UnitBirth.Supplicant` / `UnitRevive.Supplicant` / `UnitConstruction.Supplicant.Start` 事件。结果标准原版狂热者会在同一个单位作用域里同时创建 `CActorUnit[Zealot]` 与 `CActorUnit[Supplicant]`，报出 `More than one CActorUnit persisting in the same unit scope`。修法是在 `Supplicant` Actor 上覆盖索引 0、1、2、3、5，而不是追加新的创建事件。

### `parent="Ravager"` 必需的带索引覆盖

| 索引 | 来源 | 要覆盖的事件 |
|-------|--------|-------------------|
| 0, 1 | GenericUnitMinimal | `UnitBirth.##unitName##`（两份） |
| 2, 3 | GenericUnitMinimal | `UnitRevive.##unitName##`（两份） |
| 5 | GenericUnitMinimal | `UnitConstruction.##unitName##` |
| 69 | GenericBurrowerStandard | `UnitBirth.##unitName##Burrowed` → `Send="Create"` |
| 70 | GenericBurrowerStandard | `UnitBirth.##unitName##Burrowed` → `Send="AnimBracketStart Burrow..."` |
| 74 | Ravager（硬编码） | `AbilMorph.*.Cancel; MorphFrom Ravager; MorphTo RavagerBurrowed` |
| 75 | Ravager（硬编码） | `AbilMorph.*.Finish; MorphTo Ravager; MorphFrom RavagerBurrowed` |
| 76 | Ravager（硬编码） | `AbilMorph.*.Finish; MorphTo Ravager; MorphFrom RavagerCocoon` → `Send="Create"` |
| 77 | Ravager（硬编码） | `AbilMorph.*.Finish; MorphTo Ravager; MorphFrom RavagerCocoon` → `Send="$Birth 0 0.000000"` |

注意：索引 4（UnitConstruction 的第二份）与单位名无关——保持继承即可。

### 完整示例

```xml
<CActorUnit id="MyRavagerVariant" parent="Ravager" unitName="MyRavagerVariant">
    <On index="0" Terms="UnitBirth.MyRavagerVariant"/>
    <On index="1" Terms="UnitBirth.MyRavagerVariant"/>
    <On index="2" Terms="UnitRevive.MyRavagerVariant"/>
    <On index="3" Terms="UnitRevive.MyRavagerVariant"/>
    <On index="5" Terms="UnitConstruction.MyRavagerVariant"/>
    <On index="69" Terms="UnitBirth.MyRavagerVariantBurrowed" Send="Create"/>
    <On index="70" Terms="UnitBirth.MyRavagerVariantBurrowed" Send="AnimBracketStart Burrow Burrow IGNORE Unburrow ClosingFull,OpeningPlayForever,Instant,DontResetOnUnhide"/>
    <On index="74" Terms="AbilMorph.*.Cancel; MorphFrom MyRavagerVariant; MorphTo MyRavagerVariantBurrowed" Send="AnimClear Burrow"/>
    <On index="75" Terms="AbilMorph.*.Finish; MorphTo MyRavagerVariant; MorphFrom MyRavagerVariantBurrowed" Send="AnimBracketStop Burrow"/>
    <On index="76" Terms="AbilMorph.*.Finish; MorphTo MyRavagerVariant; MorphFrom RavagerCocoon" Send="Create"/>
    <On index="77" Terms="AbilMorph.*.Finish; MorphTo MyRavagerVariant; MorphFrom RavagerCocoon" Send="$Birth 0 0.000000"/>
</CActorUnit>

<!-- Burrowed splat: unitName = the UNBURROWED unit's ID -->
<CActorSplat id="MyRavagerVariantBurrowedSplat" parent="BurrowedSplat" unitName="MyRavagerVariant">
    <Scale value="1.100000"/>
</CActorSplat>

<!-- Attack action: only effectLaunch needs overriding; effectImpact inherits from parent -->
<CActorAction id="MyRavagerVariantAttack" parent="RavagerAttack" effectLaunch="MyRavagerVariantWeaponLM"/>
```

### SC2 编辑器里的红色条目

SC2 编辑器把 **所有带索引的覆盖标成红色**——这只是视觉/信息提示，不是错误。它的意思是「该条目覆盖了一个继承的父级事件」。用索引覆盖模式时无法避免。

替代方案是用 `parent="GenericUnitBase"` 并从头重定义所有事件（基础游戏里的 `RoachCorpser` 就是这么做的）。那样没有红色条目，但需要把父级单位 Actor 的每个视觉/音效事件都复制一遍。

### 具名导弹与动作父级会保留原生监听器

同样的继承风险也适用于 `CActorMissile` 与 `CActorAction`，但在叶子上改 `unitName`、`effectAttack`、`effectLaunch` 或 `effectImpact` 并不能可靠地给继承的事件数组换 token。运行时症状包括：本来合法的自定义弹药单位出现回退球体、`Can only create one CActorAction per effect`，或 `ActionImpactPhysics arrived before action commenced`。

项目自有的导弹，优先 `parent="GenericAttackMissile"`，设置自定义 `unitName`，并显式绑定活动依赖的 `CModel`。把属于预期表现的原生缩放或存活动画字段复制过来。项目自有的动作，优先 `parent="GenericAttack"`（或已验证的通用动作基类），只设本地效果，并复制源动作的挂载查询、资源、命中图、物理、索敌弧线与标志位。清理事件住在自定义光束 Actor 上时，要显式绑定自定义 `Beam`；只改那个光束而不改动作继承的光束字段，运行时没有任何效果。

### 具名单位的变形变体

自定义单位变体用从某个具名单位复制来的 `CAbilMorph` 时，用 `InfoArray index="0" Unit="CustomTarget"` 覆盖继承的变形目标。裸写 `<InfoArray Unit="CustomTarget"/>` 很容易被误读成替换，但对继承的变形数组来说，编辑器往返之后它可能仍让源单位的变形目标与源过渡 Actor 生效。在单位上，要在同一个 `AbilArray` 槽位里替换继承的变形技能，而不是追加新技能。例如父级单位的解除架设变形是第三个技能，就用 `<AbilArray index="2" Link="MyUnsiege"/>`；裸写 `<AbilArray Link="MyUnsiege"/>` 会让源变形与自定义变形命令同时存在。

对 Actor，为每个自定义变形端点提供本地 `CActorUnit`，并按索引覆盖它与单位名相关的出生/复活/建造事件。如果源变形用了 `*SiegeModeMorphModel` 这类过渡 Actor，也要建本地过渡 Actor，把 `MorphTo`/`MorphFrom` term 改写成自定义单位 ID。这样就能避免在两种修法之间反复：一种修完出现没有单位 Actor 的回退球，另一种修完源单位或源过渡 Actor 仍会进入自定义单位作用域。

---

## 不要对 Core 动画风格的基础设施槽位做索引覆盖

`<On index="N">` 是 **替换** 该槽位里的父级事件，不是追加。Core 动画风格的父级把基础设施放在那里：`ModelAnimationStyleContinuous` 用槽位 0 放 `ActorCreation -> AnimBracketStart BSD Birth Stand Death`，槽位 1 放 `ActorOrphan -> Destroy`；`ModelAnimationStyleOneShot` 的槽位 0 相同（带 `ContentPlayOnce`），槽位 1 放 `AnimBracketState.*.AfterClosing; AnimName BSD -> Destroy`。

所以子级若写 `<On index="0" Terms="Effect.MyPersistent.Start" Send="Create"/>` 加 `<On index="1" Terms="Effect.MyPersistent.Stop" Send="AnimBracketStop BSD"/>`，就删掉了括号开始与孤儿兜底：BSD 永不打开，`AnimBracketStop BSD` 变成空操作，`AfterClosing -> Destroy` 永不触发，模型永远留在地图上（ISSUE-016，`ArchonShakurasVoidRiftModel`）。

写这些 term 时 **不要** 带索引，让它们追加在继承槽位之后。这个形态的每一个已发行参考都是这么做的：灵能风暴 `HighTemplarVoidStormModelD`，SCORE-Ihanrii `BlackHoleBombI`，SCORE-Taldarim `HighTemplarVoidStormImpactD`/`SoundD`，SCORE-Golden `ArchonShieldModelG`。

加索引之前值得先确认两个相关事实：

- 具名父级自己的事件是追加在 Core 基础设施槽位 **之后** 的，所以子级的 `index="0"` / `index="1"` 打到的是基础设施，不是父级的技能监听器。去读父级 Actor 并数槽位，不要假设父级的第一个事件就是索引 0。
- 索引覆盖只移除它点名的那一个槽位。如果变体不应该响应父级的监听器（`Behavior.SomeParentBuff.On`、`Effect.SomeParentPersistent.Start`），那些父级槽位需要用显式的 `removed="1"` 行；覆盖槽位 0/1 并不能让它们静音。

核验疑似残留的 VFX Actor 时，检查它的 `AnimBracketStop BSD` 是否还有活的括号可停，并确认父级提供了孤儿路径或 `AfterClosing -> Destroy` 路径——已发行的 `Effect.<persistent>.Stop` 在自然到期时的投递是可靠的，所以缺少销毁几乎总是监听器缺失或被覆盖，而不是事件缺失。

---

## 伤害响应效果必须自己锚定位置

`CBehaviorBuff.DamageResponse Fatal="1"` 的效果树起始于伤害上下文，其中 `Target` 是伤害来源（攻击者），不是行为的持有者。`CEffectCreatePersistent` 默认用 `<WhichLocation Value="TargetPoint"/>`（Core Effects.txt L34-35），所以一个没声明位置的死亡触发持续效果会锚在击杀者身上而不是死掉的单位上——裂隙/场/爆炸开在错误的位置，它的 `RevealRadius` 也在那里揭示。

用 `<WhichLocation Value="CasterUnit"/>` 显式锚定它，也就是已发行的 `SS_ScourgeDeathPersistent` 形态（Liberty Campaign Effects.txt L852-861，由 Liberty Campaign Behaviors.txt L268-271 的 `SS_ScourgeDeath` 启动）。该持续效果的子级继承它的位置，所以子级自己的 `ImpactLocation`/`WhichLocation` 保持 `TargetPoint`——这就是为什么 `SS_ScourgeDeathLaunchMissile`（Liberty Campaign Effects.txt L845-851）在 `CasterUnit` 持续效果里保留 `TargetPoint`。

同一个 `CEffectSet` 里的兄弟条目 **不是** 子级：它不继承位置，必须自己锚定（ISSUE-018 正是为此把 `ArchonShakurasVoidRiftBurstSearch` 的 `ImpactLocation` 改成 `CasterUnit`）。`CasterUnit` 对 `WhichLocation` 与 `ImpactLocation` 都合法。

---

## ZergGroundArmors / ZergMissileWeapons —— 每一级都要有

`ZergGroundArmorsLevel1`、`Level2`、`Level3` 是独立的升级，研究到哪一级就各自触发一次。只有当 **三级** 升级上都有 EffectArray 条目时，单位在 Level3 才拿到完整的 +3 护甲。只往 Level2 与 Level3 加条目，单位最多只到 +2。

把新的异虫单位加进地面护甲升级时，三级都要加 `AffectedUnitArray` + `EffectArray` 条目（LifeArmor 与 LifeArmorLevel，各 +1）：

```xml
<!-- In ZergGroundArmorsLevel1, Level2, AND Level3 — same entries in all three -->
<AffectedUnitArray value="MyRavagerVariant"/>
<AffectedUnitArray value="MyRavagerVariantBurrowed"/>
<EffectArray Reference="Unit,MyRavagerVariant,LifeArmor" Value="1"/>
<EffectArray Reference="Unit,MyRavagerVariant,LifeArmorLevel" Value="1"/>
<EffectArray Reference="Unit,MyRavagerVariantBurrowed,LifeArmor" Value="1"/>
<EffectArray Reference="Unit,MyRavagerVariantBurrowed,LifeArmorLevel" Value="1"/>
```

对 `ZergMissileWeapons`，如果变体的武器命中集引用与基础单位相同的伤害效果（例如 `RavagerWeaponDamage`），就只需要 `AffectedUnitArray` 条目——不需要单独的 EffectArray——因为基础游戏的伤害缩放已经通过共享效果生效了。

人族与星灵也有类似的升级。

---

## 原版 `parent=` 的继承风险

**绝不要在我的 `CUpgrade` 上用 `parent="HotS*"`（或任何原版升级 ID）。**

SC2 数据编辑器的 `parent=` **只是数据字段继承**——它 **不** 表示「本升级依赖父级」。当你通过 `TechTreeUpgradeAddLevel` 应用 `HydraliskAncillaryCarapace` 时，游戏 **不会** 把 `HotSHydraliskHealth` 标记为已完成。它们是彼此独立的升级 ID。

风险在于：原版虫群之心战役地图里有 Galaxy 触发器，会在地图加载时根据 `ZCampaign` Bank 直接调用 `TechTreeUpgradeAddLevel(player, "HotSHydraliskHealth")`。这些触发器绕过了 `ArmyUpgradeData.xml` 里的 `CArmyUpgrade` 覆盖。如果玩家的 `ZCampaign` Bank 里标了那项进化（例如来自之前一次原版虫群之心流程），`HotSHydraliskHealth` 就会独立于你的模组触发，导致：

- **数值加成重复生效**（一次来自原版触发器经 `HotSHydraliskHealth`，一次来自继承同一 EffectArray 的自定义升级）
- **命令卡上多出一个原版按钮**（Row=2 Column=1 处由 `HaveHotSHydraliskHealth` 门控的槽位变可见，与你的自定义按钮冲突）

修法始终是 **写显式的 EffectArray 条目** 并去掉 `parent=`。多几行 XML，但彻底消除耦合。

```xml
<!-- WRONG — inherits effects AND risks double-apply from vanilla map triggers -->
<CUpgrade id="HydraliskAncillaryCarapace" parent="HotSHydraliskHealth"/>

<!-- CORRECT — fully self-contained, no coupling to vanilla upgrade system -->
<CUpgrade id="HydraliskAncillaryCarapace">
    <Icon value="Assets\Textures\BTN-Upgrade-Zerg-AncillaryArmor.dds"/>
    <Race value="Zerg"/>
    <EditorCategories value="Race:Zerg,UpgradeType:Talents"/>
    <EffectArray Reference="Unit,Hydralisk,LifeMax" Value="20"/>
    <EffectArray Reference="Unit,Hydralisk,LifeStart" Value="20"/>
    <!-- ... all variants ... -->
    <AffectedUnitArray value="Hydralisk"/>
    <!-- ... -->
</CUpgrade>
```

**重要：`ArmyUpgradeData.xml` 覆盖挡不住直接的 `TechTreeUpgradeAddLevel` 调用。** 把 `CArmyUpgrade` 重定向到哑升级只能拦截部队面板的升级路径。直接调用 `TechTreeUpgradeAddLevel` 的原版地图 Galaxy 触发器不受影响。

---

## CEffectDamage 字段

`CEffectDamage` 只接受 `Amount`（以及伤害类型字段）。它上面 **不存在** `ImpactLocation`、`WhichUnit`、`KillType`：

```xml
<CEffectDamage id="MyMyDoomDamage">
    <Amount value="5000"/>
</CEffectDamage>
```

不要拿 `CEffectDamage` 当自杀用的 `PeriodicEffect`——见 [galaxy-gotchas.md](../../sc2-galaxy-scripting/references/galaxy-gotchas.md)。

---

## DamageDealtFraction 是累加的

`CBehaviorBuff.Modification.DamageDealtFraction` 是施加到单位输出伤害倍率上的累加分数。正值用于加伤，负值用于减伤：

```xml
<!-- 40% less damage dealt; final multiplier is 1 + (-0.4) = 0.6 -->
<DamageDealtFraction index="Melee" value="-0.4"/>
<DamageDealtFraction index="Ranged" value="-0.4"/>
```

暴雪的示例用同样的模式：`DamageDealtMinimal` 把 Melee/Ranged/Spell/Splash 设为 `-0.9`，战役减益用 `-0.25`、`-0.35` 或 `-0.5` 做减伤。像 `0.6` 这样的值是 +60% 加伤，不是最终伤害的 60%。

---

## XML 节点结构与常见 schema 陷阱

星际争霸 II 的 GameData 修改依赖带父级继承（`parent="..."`）的级联 XML catalog。为保证结构合法、避免被引擎拒绝 schema：

### 1. 子取值元素 vs 裸属性
多数 catalog 字段要求用带 `value="..."` 属性的子 XML 元素，而不是把属性直接放在 catalog 根元素上。
- **错：** `<CUnit id="Marine" LifeMax="100" Speed="3.15"/>`
- **对：**
  ```xml
  <CUnit id="Marine" parent="MarineBase">
      <LifeMax value="100"/>
      <Speed value="3.15"/>
      <CostResource index="Minerals" value="75"/>
  </CUnit>
  ```

### 2. Card Layout 37 格式
命令卡按钮槽位用 `CardLayout37` 配 `LayoutButtons` 索引数组：
```xml
<CardLayout37>
    <LayoutButtons index="0" Ability="Stimpack" AbilityCmd="Execute" Row="2" Column="0"/>
</CardLayout37>
```

### 3. 技能与行为数组索引
字段数组（如 `CmdButtonArray`、`InfoArray`、`CostResource`）在赋值时要求显式的索引键。

### 4. 行为修改的嵌套
`CBehaviorBuff` 下的行为属性修改必须嵌在 `<Modification>` 里：
```xml
<CBehaviorBuff id="MyMyBuff">
    <Modification WeaponRange="1">
        <DamageDealtFraction index="Melee" value="0.2"/>
    </Modification>
</CBehaviorBuff>
```
不要把修改子元素直接放在根 `<CBehaviorBuff>` 标签下。

### 5. Actor 事件列表
在 `<CActorUnit>`、`<CActorAction>` 等里面，Actor 事件触发必须使用合法的 `<On Terms="..." Send="..."/>` 语法。写新的 term 表达式之前，先看当前依赖里已有的 Actor。

### 6. UI 布局（`.SC2Layout`）
`Base.SC2Data/UI/Layout/*.SC2Layout` 里的界面框体使用 `<Frame type="..." name="...">`、`<Anchor>`、`<StateGroup>` 与 `<Animation>` 标签，模板命名空间必须合法。

### schema 校验参考
GameData 与 Layout 文件的正式 XSD schema 打包在 `tools/schemas/sc2-xsd/`（源自 `sc2-arcade-watcher/sc2-xsd` 与 `SC2Mapster/sc2layout-schema`）。
- 预检校验（`tools/test-suite.py` 调用的 `tools/validate-mod.py`）会用 `lxml` 自动把全部 GameData XML 对照 `Catalog.xsd` 校验。
- VS Code 工作区设置（`.vscode/settings.json`）把 `Base.SC2Data/GameData/*.xml` 绑到 `Catalog.xsd` 以获得实时编辑器诊断。
- 外部仓库链接见 [external-sc2-resources.md](../../sc2-project-entry/references/external-sc2-resources.md)。
