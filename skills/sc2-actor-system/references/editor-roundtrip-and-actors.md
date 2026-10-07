# 编辑器往返与 Actor XML 模式

## 编辑器往返检查单

本地 XML 工作之后、宣布实现完成之前使用。非活动源提取另见 [unit-extraction-from-inactive-xml.md](../../sc2data/sc2data-units-reference/references/unit-extraction-from-inactive-xml.md)。

### 编辑器保存之前

1. 从单位出发，追完整依赖图：技能、武器、效果、行为、验证器、需求、mover、turret、Actor、模型、音效、按钮与本地化。
2. 核验每个被引用的自定义 ID 或非活动源 ID 都存在于当前依赖链或项目模组 catalog 中。
3. 为每个新 catalog 对象加上 `ObjectStrings.txt` 的 `Name` 以及 `EditorPrefix` 或 `EditorSuffix`。
4. `GameStrings.txt` 只加面向玩家的文本，并通过真实的 XML 字段锚定（`Name`、`Tooltip`、`Description`、按钮图标、升级效果引用）。
5. 避免使用非活动的、或仅合作任务才有的按钮图标与辅助 UI 记录，除非确切 ID 就在当前依赖里。
6. 修补战役结构实际打开的那些命令卡层与继承按钮索引；需要时用需求或 `removed="1"` 隐藏继承来的原版按钮。
7. 通用命令（Attack、Stop、Hold Position、Patrol、Rally）用当前战役的按钮图标。
8. 对改动的 catalog 跑一次本地 XML 解析，然后对新引用与本地化覆盖做一次 ID 级审计。

### 编辑器保存之后

1. 扫项目 `.SC2Mod/Base.SC2Data/GameData/` 下的所有文件，找出 `GameData.xml` 与带类型的旁路 catalog（`ModelData.xml`、`SoundData.xml`、`TurretData.xml`、`ValidatorData.xml`、`WeaponData.xml`、`ButtonData.xml` 等）之间重复的 `(catalog type, id)` 对。每个 ID 只保留一行规范行。
2. 把编辑器告警、空白的编辑器名、空白的命令卡图标/文本，以及预载抖动，当作提取图不完整的信号——不是无关紧要的输出。
3. 用忽略空白或 XML ID 级的比对；编辑器保存后的原始行比对往往夸大了实际设计改动（尤其是 `ActorData.xml` 的顺序）。
4. 重新检查子单位的带索引命令卡覆盖。编辑器会把继承的带索引 `LayoutButtons` 字段展开成本地子元素，并保留本不属于该自定义单位的继承需求。保存后确认重要按钮仍指向预期的 `AbilCmd`、行列与需求。
5. 当当前依赖/UI 上下文缺少被引用的 file desc 时，剥掉复制来或继承来的 Actor UI 钩子：
   - `CustomUnitStatusFrame value="Coop_UnitStatus_.../..."`
   - `CustomUnitStatusFrame value="HotS_UnitStatus/..."`
   - `CustomUnitStatusFrame value="LotV_UnitStatus/..."`
   - `StatusBarOn index="Custom" value="1"`
   - `UnitFlags index="SuppressDefaultStatusBar" value="1"`
   - 如果本地 Actor 从父级继承了有问题的状态栏，优先用 `GenericUnitStandard` 这类直接的安全父级加复制来的视觉字段，而不是保留继承的自定义 UI 链。

### 三遍校验

1. **本地静态遍：** 导入器/审计、XML 解析、重复扫描、本地化审计、`tools/sc2-catalog-query.py` 的未解析引用检查。
2. **编辑器往返：** 重新打开/另存为 Components，收集完整告警列表，把 diff 当作规范化信号来审阅。
3. **游戏内遍：** 用能覆盖生产、替换、命令卡、需求、VFX/音频、升级与战役解锁状态的最小任务场景。

# 变形过渡表现 Actor

项目自有的纯视觉变形过渡，优先用通过继承挂在 `_Selectable` 上的 `CActorModel parent="ModelAdditionNoAnims"`。在 `AbilMorph.*.Start` 创建它，在 `ActorCreation` 播放想要的变形动画，在变形结束时销毁它，并把 `ActorOrphan -> Destroy` 作为兜底加上。

避免把视觉叠加层作为 `CActorUnit parent="GenericUnitBaseMorphTransition"` 或 `CActorMissile` 导入。即使它的 `unitName` 指向一个哑变形 ID，从自定义单位的变形事件创建它也可能把它放进真实单位作用域，并报出 `More than one CActorUnit persisting in the same unit scope`。编辑器可能原样保留 XML；这个告警是运行时作用域问题，不是序列化问题。

对变体专属的变形视觉，确认被引用的 `CModel` 在当前依赖链里有显式的 `.m3` 资源路径。从非活动依赖复制来的精简行可能只含半径/缩放元数据，因为它的原始依赖把资源放在别处。那会导致变形不可见，或静默回退成通用原版模型。优先用带显式资源路径的项目自有模型 ID，再把过渡 Actor 指向该 ID。

## 单位 Actor 的带索引建造事件

重定向继承自 `GenericUnitMinimal` 的事件数组的 `CActorUnit` 行，必须保留继承索引的含义：

- 索引 `4`：`UnitConstruction.<unit>.Start`（为建造/折跃表现创建单位 Actor）；
- 索引 `5`：`UnitConstruction.<unit>.Finish`（结束建造表现）。

不要在任一索引上使用裸 `UnitConstruction.<unit>` term，也不要把 `.Start` 放到索引 5。后者会覆盖继承的完成处理器，可能留下两个常驻模型或一个回退折跃球。光有 `BuildModel` 修不好挪位的事件覆盖。编辑器往返之后，要针对这些确切后缀审计项目里所有 `CActorUnit` 行，因为编辑器可能把继承的带索引事件重新具象化出来。
