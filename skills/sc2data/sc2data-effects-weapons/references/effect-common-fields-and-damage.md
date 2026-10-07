# 效果通用字段、伤害与校验器

所有效果共享的字段、标记（marker）、响应/AI 标志、伤害类型与属性加成，以及 ValidatorArray 门控。

## 效果通用字段

以下字段出现在大多数或全部 `CEffect*` 类型上：

| 字段 | 用途 |
|---|---|
| `ValidatorArray` | AND 门控——任一校验器返回假，效果被跳过，关联的 Actor 事件也被抑制 |
| `Chance` | 效果执行的概率，0.0–1.0（默认 1.0 = 必定执行） |
| `DebugTrace` | 设为 `1` 时在测试模式下于发射/命中位置显示可视化调试标记 |
| `CasterHistory` | 把该效果存入施法者的 Effect History Limit（部分校验器会用到） |
| `CanBeBlocked` | 为 `1` 时查询目标单位的 Block Chance——被格挡则发送 Actor 子事件 `Blocked` |

### 标记 — 防止 AoE 重复命中

标记给单位打上「已被本效果实例命中」的记号，避免范围效果重复命中：

```xml
<CEffectSearch id="MySplash">
    <MarkerArray index="0" MatchFlags="Link" MismatchFlags=""/>
    <!-- Units already tagged with "Link" are skipped; new hits get tagged -->
</CEffectSearch>
```

用 `CValidatorUnitCompareMarkerCount` 依据标记状态来门控效果。

### 响应与 AI 通知标志

| 字段 | 取值 | 含义 |
|---|---|---|
| `ResponseFlags` | `Acquire` | 目标开始攻击施法者 |
| `ResponseFlags` | `Flee` | 目标逃离施法者 |
| `AINotifyFlags` | `HelpFriend`、`HurtEnemy`、`HurtFriend`、`MajorDanger`、`MinorDanger` | 触发 AI 的威胁/求援响应 |

### 首效果决定目标选取

**技能效果链中的第一个效果**决定该技能的目标选取类型。要得到可指定目标的技能（点单位或点地面），根效果必须是接受并使用目标的类型。把 `CEffectIssueOrder` 放在**第一位**会让技能无法指定目标——请把 `CEffectSet` 放在最前，或确保排序已考虑这一点。

## 伤害类型与属性加成

| 类型 | 说明 |
|---|---|
| `Melee` | 受护甲与近战升级影响 |
| `Ranged` | 受护甲与远程升级影响 |
| `Spell` | 默认无视护甲 |
| `Splash` | 可命中多个单位 |

属性加成对具备对应属性的单位追加伤害：

```xml
<AttributeBonus index="Light" value="10"/>
<AttributeBonus index="Armored" value="5"/>
<AttributeBonus index="Biological" value="15"/>
<AttributeBonus index="Massive" value="20"/>
```

## 效果中的校验器

把校验器挂到任意效果的 `ValidatorArray` 上。只要**任一校验器返回假**，该效果就**不施加**：

```xml
<CEffectDamage id="ConditionalDamage">
    <Amount value="50"/>
    <ValidatorArray index="0" value="TargetIsExposed"/>
    <ValidatorArray index="1" value="CasterNotBurrowed"/>
</CEffectDamage>
```

校验器的定义模式见 `sc2data-behaviors-validators`。
