# 从非活动 XML 提取 Actor

从非活动的 `XMLFromDependenciesWeDontUse/` 重建单位时，把主 `CActorUnit` 当作表现根。它通常串联了实时模型、建造/放置模型、头像模型、线框、编组图标、单位图标、死亡模型、语音音效、足迹 Actor、攻击动画事件、变形/模型切换事件，以及挂载的次级 Actor。

动作 Actor（`CActorAction`）通常是游戏性通往视觉/音频的桥梁。按 `effectAttack`、`effectLaunch` 或 `effectImpact` 匹配它们，然后只复制它们真正引用的发射/命中模型与音效。

对合作任务专属的 Actor term 要严格。使用 `ValidatePlayer Is...CoopCommander`、`CommanderPrestige*`、突变因子 ID、顶栏信号，或指挥官专属变形/复活钩子的事件，通常应当移除或为项目重写。Actor 事件 term 是精确字符串引用，所以改名过的单位、武器、技能、效果、行为、验证器、模型与音效必须一致地改名。

## 单位提取检查单

1. 区分完整单位定义与既有单位上的补丁叠加。优先复用当前依赖的基础数据；只复制设计所需的最小完整链。
2. 从单位出发追技能、武器、效果（追到终止符）、行为、验证器、需求、生产与命令卡、Mover/Turret/Footprint。
3. 然后追单位 Actor、攻击/技能动作 Actor、Missile/Beam/Model/Sound Actor，一路追到实际的 Model、Sound、图标、线框与本地化。
4. 列出源 ID、目标项目 ID、复用的活动定义、需要复制的定义、排除的系统。合作指挥官等级、威望、精通、顶栏与语音系统，只在设计需要时才保留。
5. 改名之后，审计字符串事件 `UnitBirth.X`、`Effect.X.Start`、`Abil.X.Start`、`MorphTo X`、`MorphFrom X`、`ValidateUnit X`——单纯的 XML 链接替换覆盖不了所有事件文本。
6. 非活动导出只是参考资料；出现在索引里并不证明该依赖已启用。确认 `XMLFromDependenciesWeDontUse/` 路径确实存在。

完整的精简提取流程在 `skills/sc2data/sc2data-units-reference/references/unit-extraction-from-inactive-xml.md`。

## 资源追踪注意事项

一个只含 `scale`/`radius` 的模型导出，可能依赖某个未导出的父级模型资源；Sound 及其模板/父级同理。要追到真正的资源；不要停在「ID 存在」这一步。

