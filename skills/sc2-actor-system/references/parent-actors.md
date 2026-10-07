# 常用父 Actor

| Actor 类型 | 父级名 | 何时使用 |
|---|---|---|
| Unit | `GenericUnitStandard` | 所有单位 Actor 的默认值。处理出生、死亡、选中、攻击动画。 |
| Action | `GenericAttack` | 攻击/技能动作 Actor 的默认值。提供 `Attack`、`Launch`、`Impact` 效果 token。光束/瞬发攻击设 `Attack`；导弹攻击设 `Launch` + `Impact`。**不要**三个都设。 |
| Model | `ModelAddition` | 把模型挂到另一个 Actor 上（例如单位身上的增益视觉）。 |
| Model | `ModelAnimationStyleContinuous` | 常驻的非挂载模型，销毁由你控制（例如灵能风暴范围）。 |
| Model | `ModelAnimationStyleOneShot` | 动画播完自动清理（例如爆炸）。一次性 VFX 优先用它，而不是手工清理。 |
| ModelMaterial | `BehaviorGlaze` | 行为激活期间给单位上一层釉色/着色。设置 `Buff` token 与 `Model`。 |
| Beam | `Beam Simple Animation Style Continuous` | 一直存在直到被销毁的光束。 |
| Beam | `Beam Simple Animation Style One Shot` | 播一次就自毁的光束。 |
| Range | `Range Abil` | 目标选取期间显示技能射程环。设置 `Ability` token——自动读取施放距离。 |
| Range | `Range Behavior` | 行为在单位身上期间显示射程环。射程需手工设置。 |
| Range | `Range Weapon` | 显示武器射程环（与 Range Abil 同样的自动读取模式）。 |
| Splat | `Cursor Splat` | 用于 AOE 预览的光标位置溅射（例如灵能风暴）。设置 `Abil` token 与 `Model`。 |
| Sound | `SoundOneShot` | 播一次音效后自毁。 |
| Sound | `SoundContinuous` | 一直播到 Actor 被销毁。 |

## Range/Splat 父级的已知编辑器 bug

在数据模块里设置 token 之后，生成的 `<On>` 事件可能是畸形的。修法：右键 Events 字段 → `Reset To Parent Value` → 选择父级名。或者打开 XML 视图，删掉新 Actor 上生成的 `<On>` 行。

## 继承注意事项

- 从具名单位 Actor 继承时，只改 `unitName` 不会重定向所有父级事件。要按它们在父级链上的实际索引覆盖与单位相关的事件；不要盲目追加一个新的出生监听器。
- 不要把旧的 Ravager 索引表套到所有 Actor 上——建造 Start/Finish 索引在不同来源页里说法不一致。一律对照当前的 `GenericUnitMinimal` 与那个具名父级。
- 项目专属弹体可以引用 `GenericAttackMissile`；攻击动作可以引用 `GenericAttack`。复制所需的挂载点、模型、命中图、物理、标志位与动画。
- 自定义 Beam 必须真的被某个 Action 引用——没被绑定的 Beam 什么也没改。

