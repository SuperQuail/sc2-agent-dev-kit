# XML schema 基础

## 用子元素承载取值，不要用裸属性

多数 catalog 字段要求用带 `value="..."` 的子元素：

```xml
<!-- WRONG -->
<CUnit id="Marine" LifeMax="100" Speed="3.15"/>

<!-- CORRECT -->
<CUnit id="Marine" parent="MarineBase">
    <LifeMax value="100"/>
    <Speed value="3.15"/>
    <CostResource index="Minerals" value="75"/>
</CUnit>
```

## 行为修改的嵌套

```xml
<CBehaviorBuff id="MyBuff">
    <Modification WeaponRange="1">
        <DamageDealtFraction index="Melee" value="0.2"/>
    </Modification>
</CBehaviorBuff>
```

## 命令卡布局

```xml
<CardLayout37>
    <LayoutButtons index="0" Ability="Stimpack" AbilityCmd="Execute" Row="2" Column="0"/>
</CardLayout37>
```

## 数组索引

`CmdButtonArray`、`InfoArray`、`CostResource`、`AffectedUnitArray` 与 `EffectArray` 都要求显式的 `index=` 或 `value=` 键。省掉索引是静默错位，不是取默认值。

## 硬语法规则

- **XML 只用 ASCII。** 元素文本与注释里不许出现非 ASCII 字符。
- **不许 `AbilAutoCmd`。** 本项目的 schema 里没有这个字段。
- **不许 `CBehaviorBuff.InitEffect`。**（`InitialEffect` 是另一个字段——不要混淆。）
- **不许纯注释的 catalog。** 内容只有注释的 catalog 文件会让 `validate-mod.py` 失败。
- 权威清单在 `sc2-project-entry` 的硬规则第 5–6 条。
