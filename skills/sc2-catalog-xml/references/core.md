# 已验证 XML 模式

经过验证、可用的数据编辑器 XML 模式。写新 XML 前一律先 grep `DataEditorXML/` 文件——字段名区分大小写。

> **注意：** 下面的代码样例用通用 `My*` ID（例如 `MyUpgrade`、`MyBehavior`、`MyAbility`）。请替换成 `AGENTS.md` 里你那个模组的前缀。

DataEditorXML 文件索引见 [skills/sc2-catalog-xml/references/data-editor-xml.md](data-editor-xml.md)。

**本地化规则：** 新增或覆盖 XML catalog 条目时，在同一次改动里加上匹配的 `ObjectStrings.txt` 编辑器 `Name` 与 `EditorPrefix` 键。所需的键格式与校验预期见 [localization.md](../../sc2-localization/references/localization.md)。

---

## 编辑器往返审计

即使主单位、技能、效果、行为与 Actor 行都能解析，单位或阵营的提取在机械层面仍可能不完整。见 [编辑器往返检查单](../../sc2-actor-system/references/editor-roundtrip-and-actors.md#editor-round-trip-checklist)。

对任何提取或复制来的单位集，把这些 catalog 都当成同一次必需实现面的一部分：

- 游戏性：`UnitData.xml`、`AbilData.xml`、`WeaponData.xml`、`EffectData.xml`、`BehaviorData.xml`、`RequirementData.xml`、`RequirementNodeData.xml`、`ValidatorData.xml`、`TurretData.xml`，以及任何被引用的 mover/足迹/升级。
- 表现：`ActorData.xml`、`ModelData.xml`、`SoundData.xml`、`ButtonData.xml`、图标/线框，以及本地化的名字/提示框。
- 编辑器可读性：每个新 catalog 对象的 `ObjectStrings.txt` 名字/前缀/后缀，包括 Actor、模型、音效、炮塔、验证器与辅助武器。

交接前做一次 ID 级审计：每个被引用的自定义 ID 或非活动源 ID，要么存在于活动依赖链，要么已被复制进项目。不要因为 XML 解析器接受该文件，就以为 SC2 编辑器会容忍缺失的模型、音效、炮塔、验证器、武器或按钮图标记录。

编辑器在改动 `EditorCategories` 后保存对象时，可能迁移或重建该对象类型在标准 catalog 文件里的行，例如 `AbilData.xml` 里的技能、`UpgradeData.xml` 里的升级。把这当成规范化信号：保留标准 catalog 行，保留旧旁路行里任何有用的 `Name`/`Tooltip` 锚点，并从宽泛的 `GameData.xml` 里删掉那条重复的旧行。不要保留两行相同的 `(catalog type, id)`。

编辑器还可能把 `BuildModel` 这类继承字段的精确子级副本规范化掉，或者删掉重复了继承 token 派生值的显式 `CActorAction.Beam`。只要父级有效值是对的，就保留规范化后的继承状态。当运行时表现需要明确无歧义的叶子绑定时，提供一个以规范光束 Actor 为父级、token 匹配的 Actor（`<custom action id>Beam`）。每次保存后都审计本地化：编辑器把它搬进 `ObjectStrings.txt`，并不能替代仍被运行时 `Name` 字段引用的 `GameStrings.txt` 键。

编辑器重写之后，保留被提升的带类型 catalog 行，删掉任何同 ID 的 `GameData.xml` 行，保住 `CAbilWarpTrain` 的 `IgnoreRampTest=1` 这类非默认字段，并重跑重复提供者与命令卡 `AbilCmd` 审计。

---

## 自定义单位创建检查单

新增或审计独立自定义单位时用它：

- **单位：** 新建模组自有的 `CUnit`，而不是改原版单位。为了组织方便设置种族/对象类型/族，但游戏性字段要单独处理：技能、命令卡、行为、武器、mover/平面、属性、标志位、生命/能量/护盾、载员、视野、得分与别名。
- **生产费用：** 训练/建造 UI 不要依赖单位编辑器里通用的 `Cost` 字段。原生的生产费用与时间由产出方 `CAbilTrain`/`CAbilBuild` 的 InfoArray 条目加上被产出单位的 `CostResource`/`Food` 决定；确保被产出单位允许显示资源，且按钮没有隐藏资源、人口或时间。见下面的「Train/Build 按钮的命令卡费用显示」。
- **技能/命令卡：** 先把技能加到产出方或单位上，再加指向确切技能命令的命令卡按钮（`Type="AbilCmd"` + 匹配的 `AbilCmd`）。光有可见的名字/提示框只证明 `Face` 有效，不证明费用/时间命令链接有效。
- **武器/效果：** 克隆或自建武器的效果链，让自定义单位独立。确认 `CWeaponLegacy.Effect`、`DisplayEffect`、目标过滤器、射程、周期与图标。对导弹/光束攻击，核验发射效果、命中效果、弹药/导弹单位、mover 与动作 Actor。
- **Actor：** 每个自定义单位都需要一个链接到它的单位 Actor。带父级的 Actor 优先用显式事件覆盖；Actor token 与单位名宏可能意外继承到父级单位。武器视觉还需要链接到自定义发射/命中效果的动作/导弹/光束 Actor。
- **模型/音效/炮塔/验证器：** Actor 与武器链经常依赖显而易见的单位/效果文件之外的 `CModel`、`CSound`、`CTurret`、辅助 `CWeapon` 与 `CValidator` 行。只追踪并复制最终 Actor/效果/武器集实际引用到的记录。
- **升级：** 把自定义单位加进相关的 `AffectedUnitArray` 条目，并为它的护甲/伤害图标、等级与数值变化加显式 `EffectArray` 条目。所有武器/护甲升级层级都要重复这些条目。
- **多形态/钻地单位：** 对复制来的多形态单位格外小心。为每种形态建独立的单位/Actor，并显式接好变形、钻地、溅射、别名与选中事件，而不是假设父级链会干净地重定向。
- **本地化/编辑器名：** 每个新 catalog ID 都要在 `GameStrings.txt` 加面向玩家的单位/按钮/提示框，在 `ObjectStrings.txt` 加面向编辑器的名字/前缀。需要时加上真正的 XML `Name`、`Tooltip` 或 `Description` 锚点，免得编辑器保存时剪掉重要文本。
- **测试：** 在游戏里放置或训练该单位，然后测生产 UI、生产完成、移动/寻路、命令卡技能、对每一类预期目标的武器、Actor/VFX、每一级升级，以及战役科技树解锁。

已知编辑器注意事项：重复/复制的数据可能让父级链接、token 与 Actor 事件仍指向原对象。调试「数据生效但 UI/VFX 不生效」时，要看整条链，而不是只看单位行。

---

## 升级数值修改（简单数值——生命、速度、伤害、护甲、费用）

```xml
<CUpgrade id="MyUpgrade">
    <Level index="0">
        <EffectArray index="0">
            <Effect value="MyEffect"/>
        </EffectArray>
    </Level>
</CUpgrade>

<CEffectModifyUnit id="MyEffect">
    <Modification MaxVitalArray="Life" Value="30"/>  <!-- +30 HP -->
</CEffectModifyUnit>
```

---

## 通过 BehaviorArray 挂行为（被动、常在单位身上——由 Requirement 门控）

```xml
<!-- BehaviorData.xml -->
<CBehaviorBuff id="MyBehavior">
    <InfoFlags index="Hidden" value="1"/>
    <Requirements value="HaveMyUpgrade"/>  <!-- inert until upgrade applied -->
    <Period value="5"/>
    <PeriodicEffect value="MyPeriodicEffect"/>
</CBehaviorBuff>

<!-- UnitData.xml — use BehaviorArray, NOT DefaultBehaviorArray (that field does not exist) -->
<CUnit id="Mutalisk">
    <BehaviorArray Link="MyBehavior"/>
</CUnit>
```

---

## 通过 XML 授予技能

```xml
<!-- AbilData.xml: channeled targeted ability -->
<CAbilEffectTarget id="MyAbility">
    <PrepEffect value="MyAbilityChannelAB"/>
    <Effect index="0" value="MyAbilityEffect"/>
    <Range value="500"/>
    <Cost><Cooldown TimeUse="30"/></Cost>
    <CastOutroTime value="2"/>
    <UninterruptibleArray index="Channel" value="1"/>
    <CmdButtonArray index="Execute" DefaultButtonFace="Hyperjump" Requirements="HaveMyAbility"/>
    <CmdButtonArray index="Cancel" DefaultButtonFace="Cancel"/>
</CAbilEffectTarget>

<!-- UnitData.xml -->
<CUnit id="Mutalisk">
    <AbilArray Link="MyAbility"/>
</CUnit>
```

---

## 智能命令路由

对支持它的技能类别，`<Flags index="Smart" value="1"/>` 让该技能参与引擎的通用智能命令选择（通常由右键触发）。`SmartPriority` 给该技能相对移动与其他可智能命令的排序，而 `SmartValidatorArray` 限制它有资格生效的目标或情形。

因此，一个复制来或继承来的指向型技能可能在本意是普通攻击或移动时截走右键。如果这不是设计的一部分，就在数据编辑器里清掉 **Stats: Flags -> Smart Command**，并核验继承后的有效值；命令卡可见性不控制智能命令路由。

来源与范围：本地 SC2 catalog schema 把 `SmartPriority` 记为智能命令的选择依据，导出的 Liberty/NovaStory 技能行把 `Smart` 标志与 `SmartValidatorArray` 成对使用。这是一条通用技能规则；社区报告的 Nova Break Neck 案例未在本地复现。

---

## 修改字段的写法

```xml
<Modification UnifiedMoveSpeedFactor="0.3"/>  <!-- +30% movement speed additive -->
<Modification AttackSpeedMultiplier="1.5"/>   <!-- 1.5× = +50% attack speed -->
<!-- Both are TOP-LEVEL attributes on <Modification>, not child elements -->

<!-- Evasion (% chance to take zero damage) -->
<DamageResponse ModifyFraction="-1">
    <Chance value="0.3"/>  <!-- 30% evasion -->
</DamageResponse>
```

---

## 传送效果

```xml
<CEffectTeleport id="MyTeleport">
    <WhichUnit Value="Caster"/>
    <TargetLocation Value="TargetPoint"/>
    <TeleportFlags index="TestFog" value="0"/>
    <PlacementRange value="15"/>
</CEffectTeleport>
```

---

## 通过升级改武器 TargetFilters

`CWeaponLegacy.TargetFilters` 是 **单个字段**，不是数组——不要用 `index` 属性。直接用 `Operation="Set"`：

```xml
<EffectArray Operation="Set" Reference="Weapon,ViperAir,TargetFilters" Value="Visible;Missile,Stasis,Dead,Hidden,Invulnerable"/>
```

---
