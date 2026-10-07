# SC2 Actor 系统参考

Actor 控制游戏里一切可见或可听的东西——单位模型、命中音效、技能 VFX、光束、溅射等等。它们在玩家本地机器上异步计算，**不跨网络同步**，所以跨客户端比较 Actor 状态不可靠。低端机器上，某些画质设置会令 Actor 根本不存在。

---

## Actor 如何工作

Actor 逻辑是 **事件驱动** 的：Actor 自己声明监听哪些事件、事件触发时做什么。这与游戏性数据（技能、效果、武器）恰好相反——那些是前向声明的：技能说该跑哪个效果；Actor 说的是它何时创建自己。

对战役人类空投舱，`BarracksDropPod` 是一个 `CActorModel`，在 `Effect.DropTrain.Start` 时创建，使用 `BarracksDropPod` 模型（`DropPodFalling.m3`）。游戏性一侧是 `DropTrainSet` → `MakePrecursor` + `DropTrain`；项目可以复用这条链来实现人类现场折跃生产。见 [production.md](../../sc2-catalog-xml/references/production.md#terran-drop-pod-visuals-for-field-warp-production)。

每个 Actor 事件由三个元素组成：

| 元素 | 触发器中的对应物 | 说明 |
|---------|---------------------|-------------|
| Event | 触发器事件 | 发生了什么（例如 `UnitBirth.Marine`） |
| Terms | 条件 | 必须为真的额外条件 |
| Message/Send | 动作 | 要做什么（例如 `Create`、`AnimPlay`、`SetTintColor`） |

没有目标的 Message 由 Actor 发给自己。Actor 之间可以按 **名字**、按 **别名**（例如 `_Unit` 是单位 Actor 的标准别名），或按 **系统引用**（例如 `::Creator`）寻址。

Actor 事件也可以通过触发器编辑器里的 `Send Actor Message` 动作触发，让触发器直接驱动 Actor 行为。

---

## 常用 Actor 字段

| 字段 | 说明 |
|-------|-------------|
| Aliases | 备用引用名。`_Unit` 是标准单位 Actor 别名，其他 Actor 用它寻址该单位而不必知道其确切 ID。 |
| Copy Source | 把另一个 Actor 设为代理父级。子 Actor 会从 Copy Source 继承，前提是相关属性列在 `Accepted Property Transfers`（源）与 `Inherited Properties`（子）中。 |
| Filter | 把 Actor 可见性限制到 Ally / Enemy / Neutral / Self 组。 |
| Filter Player | 按玩家覆盖可见性。 |
| Flags — Suppress Creation Errors | 压掉资源缺失错误。用于有意设计成在特定条件下创建失败的 Actor。 |
| Fog Visibility | Dimmed / Hidden / Snapshot / Visible——控制战争迷雾下 Actor 的表现。 |
| Events | Actor 事件列表（事件 + term + 消息 三元组）。 |
| Macros | 可复用的宏事件集合，可挂到多个 Actor 上。 |
| Remove | 从父级链上显式移除不想要的继承事件。 |
| Terms | 短路创建条件——与该创建事件的 term 等价；两者都设置时会覆盖后者。便于组织数据。 |
| Host Supporter | 一个支援用 Actor；它死亡时会向宿主 Actor 发送 `SupporterDestruction`（常用于销毁宿主或播放死亡动画）。 |
| Accepted Property Transfers | 本 Actor 向子 Actor 传递哪些属性（模型缩放、不透明度、队伍颜色、可见性、贴花、贴图、迷雾颜色、位置、朝向等）。 |
| Inherit Type | `Once`（创建时继承一次）、`Continuous`（动态更新）、`None`。 |
| Inherited Properties | 本 Actor 创建时从宿主继承哪些属性。必须与宿主的 `Accepted Property Transfers` 对应。 |

**编辑器 Events 字段的配色：**
- 灰色 = 继承自核心游戏数据
- 蓝色 = 来自某个暴雪依赖模组
- 绿色 = 来自当前项目
- 红色 = 对被继承父级事件做了带索引的覆盖（只是视觉提示——不是错误）

---

## 常用父 Actor

| Actor 类型 | 父级名 | 何时使用 |
|-----------|-------------|-------------|
| Unit | `GenericUnitStandard` | 所有单位 Actor 的默认值。处理出生、死亡、选中、攻击动画等。 |
| Action | `GenericAttack` | 攻击/技能动作 Actor 的默认值。提供 `Attack`、`Launch`、`Impact` 效果 token。光束/瞬发攻击设 `Attack`；导弹攻击设 `Launch` + `Impact`。**不要**三个都设。 |
| Model | `ModelAddition` | 把模型挂到另一个 Actor 上（例如单位身上的增益视觉）。 |
| Model | `ModelAnimationStyleContinuous` | 常驻的非挂载模型，销毁由你控制（例如灵能风暴范围）。 |
| Model | `ModelAnimationStyleOneShot` | 动画播完自动清理（例如爆炸）。一次性 VFX 优先用它，而不是手工清理；攻击则用 GenericAttack 内置的发射/命中模型。 |
| ModelMaterial | `BehaviorGlaze` | 行为激活期间给单位上一层釉色/着色。设置 `Buff` token 与 `Model`。 |
| Beam | `Beam Simple Animation Style Continuous` | 一直存在直到被销毁的光束。 |
| Beam | `Beam Simple Animation Style One Shot` | 播一次就自毁的光束。 |
| Range | `Range Abil` | 目标选取期间显示技能射程环。设置 `Ability` token——自动读取施放距离。 |
| Range | `Range Behavior` | 行为在单位身上期间显示射程环。射程需手工设置。 |
| Range | `Range Weapon` | 显示武器射程环（与 Range Abil 同样的自动读取模式）。 |
| Splat | `Cursor Splat` | 用于 AOE 预览的光标位置溅射（例如灵能风暴）。设置 `Abil` token 与 `Model`。 |
| Sound | `SoundOneShot` | 播一次音效后自毁。 |
| Sound | `SoundContinuous` | 一直播到 Actor 被销毁。 |

**Range/Splat 父级的已知编辑器 bug：** 在数据模块里设置 token 之后，生成的 `<On>` 事件可能是畸形的。修法：右键 Events 字段 → `Reset To Parent Value` → 选择父级名。或者打开 XML 视图，删掉新 Actor 上生成的 `<On>` 行。

### 动作 Actor 的导弹默认值

对基于 `GenericAttack` 的 `CActorAction`，名字恰好是 `<ActionActorId>Missile` 的导弹 Actor 就是默认导弹绑定。在 XML 里它可能表现为显式的 `<Missile value="..."/>`，但 SC2 编辑器保存时可能删掉该字段，因为解析出的预载 token 已经是 `##id##Missile`。

以下全部为真时，把这次删除当正常现象：

- 动作 Actor 用 `parent="GenericAttack"`，或另一个导弹 token 已解析为 `##id##Missile` 的父级。
- 预期导弹 Actor ID 恰好是动作 Actor ID 加上 `Missile`。
- `PreloadAssetDB.txt` 仍显示该动作 Actor 通过 `##id##Missile` 解析导弹槽位。

不要把它推广到那些从父级继承了一个具名导弹的动作 Actor。例如基于 `MarauderAttackBase` 的动作 Actor 可能继承 `MarauderAttackMissile`；如果自定义弹体应该是 `<CustomActionId>Missile`，就保留显式的 `<Missile value="..."/>` 覆盖。

### 贴图声明与继承的贴图切换

模型上的 `TextureDeclares` 把贴图前缀和文件名适配映射到贴图槽位；它们本身不会发起切换。`TextureSelectById`、`TextureSelectByMatch`、`TextureSelectBySlot` 这类 Actor 消息才执行选择。因此，当一个复制来的模型或皮肤保留了这些声明，而继承的 Actor 事件又通过 `DarkProtoss` 或 `PurifierProtoss` 这类战役升级去命中它们时，模型看起来就会坏掉。

删掉声明可以让那个不想要的选中不再解析，但同时也会禁掉任何需要该槽位的正常动态贴图切换。变体仍然需要动态贴图选择时，优先删除或覆盖继承的 Actor 事件，或者阻止其启用升级。只有模型确实自成一体、且没有任何保留的 Actor 事件会切换它的贴图时，才省略这些声明。

来源与范围：本地 catalog schema 把声明前缀定义为模型贴图所用的引用，并把 `TextureSelect*` Actor 消息单独定义。虚空之遗战役数据同时提供模型声明和升级驱动的 `TextureSelectById` 事件；本规则讲的是动态贴图选择，不是嵌在 `.m3` 模型里的普通贴图。

---

## 从非活动 XML 提取 Actor

从 `XMLFromDependenciesWeDontUse/` 重建单位时，把主 `CActorUnit` 当作表现根。它通常串联了实时模型、建造/放置模型、头像模型、线框、编组图标、单位图标、死亡模型、语音音效、足迹 Actor、攻击动画事件、变形/模型切换事件，以及挂载的次级 Actor。

动作 Actor（`CActorAction`）通常是游戏性通往视觉/音频的桥梁。按 `effectAttack`、`effectLaunch` 或 `effectImpact` 匹配它们，然后只复制它们真正引用的发射/命中模型与音效。

对合作任务专属的 Actor term 要严格。使用 `ValidatePlayer Is...CoopCommander`、`CommanderPrestige*`、突变因子 ID、顶栏信号，或指挥官专属变形/复活钩子的事件，通常应当移除或为项目重写。Actor 事件 term 是精确字符串引用，所以改名过的单位、武器、技能、效果、行为、验证器、模型与音效必须一致地改名。

完整的精简提取流程见 [`../../sc2data/sc2data-units-reference/references/unit-extraction-from-inactive-xml.md`](../../sc2data/sc2data-units-reference/references/unit-extraction-from-inactive-xml.md)。

要写示例，用当前依赖的转储和项目自己的模组 catalog；不要依赖另一个本地参考模组。

---

## 自定义父级

你可以自建父 Actor 来在多个 Actor 之间共享事件逻辑。在对事件系统足够熟悉之前不建议这么做——父级写错会静默继承坏行为。

---

## Actor 与触发器

两套系统能做相似的事，但正确选择取决于作用域：

- **Actor 事件**——描述某一 *类* 单位/效果的通用表现（一个陆战队员永远长什么样、听起来怎样、如何播放动画）。
- **触发器**——在游戏过程中调整单位或对象的 *具体实例*（奖励目标完成时，把这个特定的陆战队员染成蓝色）。

当触发器需要驱动 Actor 变化时，用 `Send Actor Message` 从触发器代码把消息推进 Actor 事件系统。
