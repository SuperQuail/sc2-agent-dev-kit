# 继承、父级与变体

## `parent=` 只是数据字段继承

`parent="..."` **不** 建立依赖关系。对升级，绝不把自定义 `CUpgrade` 写成 `parent="HotS*"`——原版地图触发器会直接调用 `TechTreeUpgradeAddLevel`，导致重复生效。改为写显式的 `EffectArray` 条目。

## 从具名单位 Actor 派生变体

写 `parent="SomeUnitActor"` 时，`##unitName##` 宏解析到最近的、定义了 `unitName` 的祖先，**不是** 叶子。要在子 Actor 里按索引覆盖每一个与单位名相关的事件。编辑器里的红色条目只是视觉提示（表示这是覆盖）。

## 变形变体

用复制来的 `CAbilMorph` 时，用 `<InfoArray index="0" Unit="CustomTarget"/>` 覆盖继承的变形目标。要在同一个 `AbilArray` 槽位替换继承的技能（用 `index=`）；不要往后追加。

## 验证器

- `CValidatorCombine` 默认是 `Type="Or"`。省略显式的 `<Type value="Or"/>`（编辑器会把它规范化掉）。AND 要显式写 `<Type value="And"/>`，取反用 `<Negate value="1"/>`——绝不用 `Type="Not"`。
- `Void.SC2Mod` 里已有验证器：`SourceIsNotStationary`（单位移动速度 > 0）——不要重新定义它。
- `CRequirement` 用 `NodeArray index="Show" Link="..."` 表示 Show/Use；Not 节点用 `OperandArray`。复用依赖验证器前先确认当前源。

## 效果与伤害

- `CEffectDamage` 只接受 `Amount`（以及伤害类型字段）。`ImpactLocation`、`WhichUnit`、`KillType` **不存在**。
- `CBehaviorBuff.Modification.DamageDealtFraction` 是 **累加** 的：正值 = 加伤，负值 = 减伤。`0.6` 表示 +60%（最终倍率 1.6），不是最终 60%。

## 异虫升级

`ZergGroundArmorsLevel1/2/3` 是三个独立升级，各触发一次。只有当 **三级全都** 有 `EffectArray` 条目时，单位才拿到完整的 +3。三级都要加 `AffectedUnitArray` + `EffectArray`（LifeArmor 与 LifeArmorLevel，各 +1）。
