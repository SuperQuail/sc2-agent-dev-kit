# 技能花费与目标过滤

技能使用的花费/冷却/充能字段，以及 TargetFilters 标志语法。

## 花费字段

| 字段 | 用途 |
|---|---|
| `Vital index="Energy"` | 能量花费 |
| `Vital index="Life"` | 生命值花费 |
| `Cooldown TimeUse` | 使用后冷却（秒） |
| `Cooldown TimeStart` | 游戏开始时的初始冷却 |
| `Charge TimeUse` | 基于充能的冷却 |

## TargetFilters

多个过滤组之间用 `;` 分隔。组内各个值是标志位——前缀 `!` 表示取反：

```xml
<!-- Visible, non-self, non-ally, non-dead targets -->
<TargetFilters index="0" value="Visible;Self,Ally,Dead"/>
```

常用过滤词：`Visible`、`Invulnerable`、`Self`、`Ally`、`Enemy`、`Neutral`、`Dead`、`Ground`、`Air`、`Structure`、`Hero`、`Biological`、`Mechanical`。
