# DataEditorXML —— 字段名参考

本文件夹存放从 SC2 数据编辑器导出的 XML 转储。为自定义战役模组写任何 XML 之前，用这些文件查正确的字段名、大小写、属性名与既有 ID。

> **规则：** 写任何 XML 之前先 grep 这些文件。不要猜字段名——它们区分大小写，而且编辑器之外没有文档。

需要非活动依赖（例如诺娃隐秘行动、合作指挥官或自定义扩展）的 catalog 时，可以从 SC2 数据编辑器把对应的 `.txt` XML 转储导出到 `DataEditorXML/` 或一个自定义参考文件夹。见 [`../../sc2data/sc2data-units-reference/references/unit-extraction-from-inactive-xml.md`](../../sc2data/sc2data-units-reference/references/unit-extraction-from-inactive-xml.md)。

要组件形态的原始参考数据，用 [`DataEditorXML/SC2GameDataComponents/`](../../../DataEditorXML/SC2GameDataComponents/README.md)。该快照保留组件路径，含 26 个官方与合作 `.SC2Mod` / `.SC2Campaign` 文件夹里可用的 `Base.SC2Data/GameData/`、`enUS.SC2Data/` 与 `zhCN.SC2Data/` 内容。把它当参考证据，不要当成「该组件是活动依赖」的证明。

---

## 这些文件是什么

每个 `.txt` 文件都是从某个具体 SC2 数据源（模组或战役层）导出的原始 XML 转储。文件里是 `<Catalog>` 块加数据条目（`CUpgrade`、`CBehaviorBuff`、`CEffectDamage` 等），展示暴雪究竟怎么命名字段、怎么组织数组、用了什么取值。

你可以在暴雪 catalog 旁边加上自己的工作示例 `.txt` 转储（例如一个已验证的升级写法）——在下表里登记它们。

要跨参考导出、非活动源、当前项目与已解析依赖做图式查询，跑：

```powershell
python tools/build-sc2-catalog-graph.py --sqlite-only
```

上面这条只刷新 `catalog.sqlite`。需要完整的 `graph.json`、`graph.graphml`、重复/未解析引用审计、单对象 Markdown 摘要与 graphify 摄取页时，去掉 `--sqlite-only`。

用本地查询工具做省 token 的智能体查询：

```powershell
python tools/sc2-catalog-query.py stats
python tools/sc2-catalog-query.py find Marine --source-class local_mod --limit 20
python tools/sc2-catalog-query.py show Unit:Marine --limit 30 --depth 2
python tools/sc2-catalog-query.py providers Button:Marine
python tools/sc2-catalog-query.py unresolved --contains My --limit 40
python tools/sc2-catalog-query.py path Abil:MyGatewayTrain Unit:MyCommando --max-depth 3
python tools/sc2-catalog-query.py unit-chain Unit:Marine --source-class local_mod
python tools/sc2-catalog-query.py ability-chain Abil:MyGatewayTrain --source-class local_mod
python tools/sc2-catalog-query.py actor-chain Unit:Marine --source-class local_mod
python tools/sc2-catalog-query.py production-chain Unit:MyCommando --source-class local_mod
```

优先用这个确定性查询工具，而不是把原始 XML 转储或 graphify 报告加载进模型上下文。有 `catalog.sqlite` 时它读它，否则回退到 `graph.json`，并且只返回所请求的范围内节点、边、提供者或未解析引用信息。

顶层 `.txt` 转储与组件快照有重叠：156 个顶层转储里有 36 个与快照 XML 文件逐字节相同（8,155,558 字节，测于 2026-09-24）。这是精确文件计数，不是共享 catalog 对象的度量。ID 与关系仍以图为首要查询入口；只有当需要一份匹配的实现、组件出处或中英文字符串时才去查原始组件示例：

```powershell
python tools/sc2-reference-query.py components --source CM
python tools/sc2-reference-query.py find 'id="Marine"' --area gamedata --family Unit --component liberty --limit 20
python tools/sc2-reference-query.py find 'Unit/Name/Marine' --area zhcn --component liberty --limit 10
python tools/sc2-reference-query.py object Unit:Marine --component liberty --max-chars 6000
```

用返回的组件路径与行号去看原始文件的一小段；`object` 会连组件路径一起返回确切的 XML 对象。快照在 catalog 图之外，不能确立活动依赖或有效值。图里的顶层 TXT 定义标为 `reference_export`；只有已解析的组件依赖标为 `active_component_dependency`。

实现或调试单位时，先用手工整理的链命令，再用原始 grep：

- `unit-chain`：单位的技能、武器、行为、命令按钮、生产技能与监听它的 Actor。
- `ability-chain`：产出单位、被产出/引用的单位、按钮、需求、效果与验证器。
- `actor-chain`：某个 Actor 或单位的 Actor 事件目标、模型、音效，以及被创建/发消息的 Actor。
- `production-chain`：把被产出单位，或训练/建造技能连同生产按钮与需求分在一组。

正常实现工作期间，不要对原始 catalog XML 转储跑 graphify。对生成的 catalog 页做宽泛的 graphify/Gemini 摄取，已经证明对本语料太吵、配额消耗太大。把确定性图与 `sc2-catalog-query.py` 当作 catalog 导航的主要层面；graphify 留给更小的仓库/wiki 图或范围极紧的实验。

## 已包含的转储覆盖

当前依赖的转储集现在超出了最初以单位/效果/Actor/技能为主的集合，纳入了更广的 Button、Sound、Requirement Node、Texture、Game UI Data、Data Collection、Core 战役元数据、Void Campaign Story 与合作指挥官元数据文件。完整覆盖请用 `tools/build-sc2-catalog-graph.py`，不要只依赖下面手写的表。

单独的 `SC2GameDataComponents/` 快照补充了官方与合作组件包的原始 GameData 与中英文本地化树。它的组件层级还覆盖了下面这份整理过的 `.txt` 转储索引里没有的包与文件。

高价值新增：

- Core、Liberty、Swarm 与 Void 层的活动 `* Buttons.txt`。
- Core、Liberty、Swarm、Void 与 Void Campaign Story 层的活动 `* Sounds.txt`。
- Core、Liberty Mod/Campaign 与 Swarm Mod 的额外 `* Requirement Nodes.txt`。
- Liberty、Swarm Campaign、Void Campaign 与 Void Campaign Story 的活动 `* Textures.txt`。
- 活动 `* Game UI Data.txt`、`* Data Collection.txt`、Core 战役地图/目标，以及 Void Campaign Story 的部队/地图/Bank/目标/位置文件。
- `Coop Commanders.txt` 用于识别要排除的指挥官专属系统，不是活动依赖源。

## 外部数据文档

精确 XML 用本地转储。字段含义用外部数据文档：

- [Talv Data File List](https://mapster.talv.space/data/files.html) 按头文件/catalog 族路由（`Unit.h`、`Abil.h`、`Behavior.h`、`Effect.h`、`Actor.h`、`Requirement.h`、`Weapon.h` 等）。
- [Talv Data Class List](https://mapster.talv.space/data/annotated.html) 按 catalog 类路由，如 `CUnit`、`CAbilTrain`、`CAbilEffect`、`CBehaviorBuff`、`CEffectDamage`、`CRequirement*`、`CActorUnit`。
- [sc2-gamedata-documentation](https://github.com/chansey97/sc2-gamedata-documentation/tree/master) 是这些数据结构文档的上游/离线 CHM 来源。

重要边界：外部文档解释字段是什么意思，但不能替代 grep `DataEditorXML/` 来确认大小写、数组索引拼写、继承覆盖或暴雪示例。

---

## 按来源划分的文件索引

### Core（基础游戏默认值 —— 所有种族/模组）
| 文件 | 内容 |
|---|---|
| `Core Units.txt` | 默认 `CUnit*` 字段 schema |
| `Core Abilities.txt` | 默认 `CAbil*` 字段 schema 与标志位 |
| `Core Actors.txt` | 默认 Actor 字段 schema |
| `Core Behaviors.txt` | 默认 `CBehavior*` 字段 schema |
| `Core Models.txt` | 默认模型字段 schema 与基础模型 ID |
| `Core Validators.txt` | 默认验证器字段 schema |

### Liberty Mod（自由之翼基础模组）
| 文件 | 内容 |
|---|---|
| `Liberty Mod Units.txt` | 自由之翼模组单位 |
| `Liberty Mod Abilities.txt` | 自由之翼模组技能 |
| `Liberty Mod Actors.txt` | 自由之翼模组 Actor |
| `Liberty Mod Behaviors.txt` | 自由之翼模组行为 |
| `Liberty Mod Effects.txt` | 自由之翼模组效果 |
| `Liberty Mod Footprints.txt` | 自由之翼模组足迹——建造放置、菌毯检查、轮廓形状与寻路层的首要参考 |
| `Liberty Mod Models.txt` | 自由之翼模组模型 |
| `Liberty Mod Movers.txt` | 自由之翼模组 mover |
| `Liberty Mod Requirements.txt` | 自由之翼模组需求 |
| `Liberty Mod Turrets.txt` | 自由之翼模组炮塔 |
| `Liberty Mod Upgrades.txt` | 自由之翼模组升级 |
| `Liberty Mod Validators.txt` | 自由之翼模组验证器 |
| `Liberty Mod Weapons.txt` | 自由之翼模组武器 |
| `Liberty Tactical AI.txt` | 自由之翼战术 AI 数据 |

### Liberty Campaign（自由之翼战役层）
| 文件 | 内容 |
|---|---|
| `Liberty Campaign Abilities.txt` | 自由之翼战役技能 |
| `Liberty Campaign Actors.txt` | 自由之翼战役 Actor |
| `Liberty Campaign Attach Methods.txt` | 自由之翼战役挂载方式 |
| `Liberty Campaign Behaviors.txt` | 自由之翼战役行为 |
| `Liberty Campaign Effects.txt` | 自由之翼战役效果 |
| `Liberty Campaign Footprints.txt` | 自由之翼战役足迹与仅战役的放置/寻路示例 |
| `Liberty Campaign Models.txt` | 自由之翼战役模型 |
| `Liberty Campaign Movers.txt` | 自由之翼战役 mover |
| `Liberty Campaign Requirements.txt` | 自由之翼战役需求 |
| `Liberty Campaign Tactical AI.txt` | 自由之翼战役战术 AI |
| `Liberty Campaign Turrets.txt` | 自由之翼战役炮塔 |
| `Liberty Campaign Units.txt` | 自由之翼战役单位 |
| `Liberty Campaign Upgrades.txt` | 自由之翼战役升级——**升级写法的关键参考** |
| `Liberty Campaign Validators.txt` | 自由之翼战役验证器 |
| `Liberty Campaign Weapons.txt` | 自由之翼战役武器 |

### Swarm Mod（虫群之心基础模组）
| 文件 | 内容 |
|---|---|
| `Swarm Mod Abilities.txt` | 虫群之心模组技能 |
| `Swarm Mod Actors.txt` | 虫群之心模组 Actor |
| `Swarm Mod Behaviors.txt` | 虫群之心模组行为——在这里查既有的异虫行为 ID |
| `Swarm Mod Effects.txt` | 虫群之心模组效果 |
| `Swarm Mod Movers.txt` | 虫群之心模组 mover |
| `Swarm Mod Requirements.txt` | 虫群之心模组需求 |
| `Swarm Mod Turrets.txt` | 虫群之心模组炮塔 |
| `Swarm Mod Units.txt` | 虫群之心模组单位 |
| `Swarm Mod Upgrades.txt` | 虫群之心模组升级 |
| `Swarm Mod Validators.txt` | 虫群之心模组验证器 |
| `Swarm Mod Weapons.txt` | 虫群之心模组武器 |

### Swarm Campaign（虫群之心战役层）
| 文件 | 内容 |
|---|---|
| `Swarm Campaign Abilities.txt` | 虫群之心战役技能——变形技能、训练技能等 |
| `Swarm Campaign Actors.txt` | 虫群之心战役 Actor |
| `Swarm Campaign Attach Methods.txt` | 虫群之心战役挂载方式 |
| `Swarm Campaign Behaviors.txt` | 虫群之心战役行为——变种行为、限时效果 |
| `Swarm Campaign Effects.txt` | 虫群之心战役效果——伤害量、搜索区域、施加行为 |
| `Swarm Campaign Footprints.txt` | 虫群之心战役足迹，含菌毯源与战役建筑足迹 |
| `Swarm Campaign Locations.txt` | 战役位置数据 |
| `Swarm Campaign Maps.txt` | 战役地图元数据 |
| `Swarm Campaign Models.txt` | 虫群之心战役模型 |
| `Swarm Campaign Movers.txt` | 虫群之心战役 mover |
| `Swarm Campaign Objectives.txt` | 任务目标定义 |
| `Swarm Campaign Requirements.txt` | 虫群之心战役需求——**在这里查可复用的既有 CRequirement ID** |
| `Swarm Campaign Requirement Nodes.txt` | 虫群之心战役需求节点（例如利维坦升级机制） |
| `Swarm Campaign Story Army Units.txt` | 剧情模式部队单位条目 |
| `Swarm Campaign Story Bank Conditions.txt` | 剧情模式 Bank 条件 |
| `Swarm Campaign Story Locations.txt` | 剧情模式位置数据 |
| `Swarm Campaign Story Map Data.txt` | 剧情模式地图数据 |
| `Swarm Campaign Story Objectives.txt` | 剧情模式目标 |
| `Swarm Campaign Tactical AI.txt` | 虫群之心战役战术 AI |
| `Swarm Campaign Turrets.txt` | 虫群之心战役炮塔 |
| `Swarm Campaign Units.txt` | 虫群之心战役单位——变种 ID、单位字段定义 |
| `Swarm Campaign Upgrades.txt` | 虫群之心战役升级——**最重要的文件；既有的虫群之心天赋升级 XML** |
| `Swarm Campaign Validators.txt` | 虫群之心战役验证器 |
| `Swarm Campaign Weapons.txt` | 虫群之心战役武器——武器 ID、伤害字段、攻击速度 |

### Swarm Story（虫群之心剧情/过场层）
| 文件 | 内容 |
|---|---|
| `Swarm Story Units.txt` | 剧情单位定义 |

### Void Mod（虚空之遗基础模组）
| 文件 | 内容 |
|---|---|
| `Void Mod Abilities.txt` | 虚空之遗模组技能 |
| `Void Mod Actors.txt` | 虚空之遗模组 Actor |
| `Void Mod Behaviors.txt` | 虚空之遗模组行为——含 `SourceIsNotStationary` 等可复用验证器 |
| `Void Mod Effects.txt` | 虚空之遗模组效果 |
| `Void Mod Footprints.txt` | 虚空之遗模组足迹与后续依赖的覆盖 |
| `Void Mod Movers.txt` | 虚空之遗模组 mover |
| `Void Mod Requirements.txt` | 虚空之遗模组需求 |
| `Void Mod Turrets.txt` | 虚空之遗模组炮塔 |
| `Void Mod Units.txt` | 虚空之遗模组单位 |
| `Void Mod Upgrades.txt` | 虚空之遗模组升级 |
| `Void Mod Validators.txt` | 虚空之遗模组验证器——重新定义之前先在这里查已有验证器 |
| `Void Mod Weapons.txt` | 虚空之遗模组武器 |

### Void Campaign（虚空之遗战役层）
| 文件 | 内容 |
|---|---|
| `Void Campaign Abilities.txt` | 虚空之遗战役技能 |
| `Void Campaign Actors.txt` | 虚空之遗战役 Actor |
| `Void Campaign Army Categories.txt` | 虚空之遗部队面板分类定义 |
| `Void Campaign Army Units.txt` | 虚空之遗部队面板单位条目与战役解锁元数据 |
| `Void Campaign Attach Methods.txt` | 虚空之遗战役挂载方式 |
| `Void Campaign Bank Conditions.txt` | 虚空之遗战役 Bank 条件定义 |
| `Void Campaign Behaviors.txt` | 虚空之遗战役行为 |
| `Void Campaign Effects.txt` | 虚空之遗战役效果 |
| `Void Campaign Footprints.txt` | 虚空之遗战役足迹与任务专属放置/寻路示例 |
| `Void Campaign Locations.txt` | 虚空之遗战役位置数据 |
| `Void Campaign Maps.txt` | 虚空之遗战役地图元数据 |
| `Void Campaign Models.txt` | 虚空之遗战役模型 |
| `Void Campaign Movers.txt` | 虚空之遗战役 mover |
| `Void Campaign Objectives.txt` | 虚空之遗战役目标定义 |
| `Void Campaign Requirements.txt` | 虚空之遗战役需求 |
| `Void Campaign Turrets.txt` | 虚空之遗战役炮塔 |
| `Void Campaign Units.txt` | 虚空之遗战役单位 |
| `Void Campaign Weapons.txt` | 虚空之遗战役武器 |

---

## 怎么用

1. **找一个既有 ID** —— grep 相关源文件。例：查 `HotSAdrenalGlands` 怎么定义，就 grep `Swarm Campaign Upgrades.txt`。
2. **查字段名** —— grep 该类型的文件。例：查 `CWeaponLegacy` 上武器射程的正确属性名，就 grep `Swarm Campaign Weapons.txt` 或 `Swarm Mod Weapons.txt`。
3. **查数组索引名** —— 看既有条目。例：`InfoArray[Train2]` 的索引名来自 `Swarm Campaign Abilities.txt`。
4. **找已存在的验证器/需求** —— 新建之前先查 `Void Mod Validators.txt` 与 `Swarm Campaign Requirements.txt`。
5. **确认 `EffectArray` 引用字符串** —— 在 `Swarm Campaign Upgrades.txt` 里 grep `Reference=`，看确切的路径格式（`Weapon,WeaponID,FieldName`、`Abil,AbilID,InfoArray[Slot].Field` 等）。
6. **查足迹/放置行为** —— 新建或覆盖 `Footprint` / `PlacementFootprint` 取值之前，grep 新的 `* Footprints.txt` 文件。`Liberty Mod Footprints.txt` 含关键基线示例，如 `Footprint2x2IgnoreCreepContour`、`Footprint3x3IgnoreCreepContour`、`Footprint5x5DropOff` 与封顶气泉足迹。
7. **查虚空之遗战役外壳数据** —— 战争议会、任务与持久化相邻的元数据用 `Void Campaign Army Units.txt`、`Void Campaign Army Categories.txt`、`Void Campaign Bank Conditions.txt`、`Void Campaign Maps.txt`、`Void Campaign Locations.txt` 与 `Void Campaign Objectives.txt`。
8. **查模型引用** —— 复制 Actor/模型 ID 之前，grep `Core Models.txt`、`Liberty Campaign Models.txt`、`Liberty Mod Models.txt`、`Swarm Campaign Models.txt` 或 `Void Campaign Models.txt`。
9. **查非活动的模型/音效支撑** —— 选中的单位来自 `XMLFromDependenciesWeDontUse/` 时，顺着 Actor 追进该源的 `Models.txt` 与 `Sounds.txt` 导出。只复制最终 Actor 链引用到的模型/音效 ID；跳过指挥官语音、过场、威望、突变因子、顶栏与无关的 UI 音效/模型。

---

## 单位 catalog —— 实用语义

本节配合对 `Core Units.txt`、`Swarm Mod Units.txt` 或 `Swarm Campaign Units.txt` 的 grep 使用，以取得 **确切的 XML 名字**。转储展示字段；下面这些要点记录容易漏掉的行为。

- **每个单位 32 个技能** —— `AbilArray` 上限 32。放在变形 **目标** 上的技能也算进 **源** 单位的配额。
- **Attack / Move / Warpable** —— 没有 **Attack**，武器不开火。没有 **Move**，单位不能移动。从能量场折跃入场需要 **Warpable**。技能可以待在 `AbilArray` 里而没有命令卡按钮，仍可通过 **Issue Order** 或生成/创建效果使用。
- **出生行为** —— 单位默认行为在出生时生效，且 **不能** 用 **Transfer Behavior** 挪走（需要单位之间迁移就用施加/移除，或复制定义）。
- **单位上的建造时间** —— 单位的建造时间字段会被 **CAbilBuild / Train / Morph** 消费用于计时；同时始终要确认 **技能**——它不是唯一可能覆盖计时的地方。
- **加速度与减速度** —— `1000` 加速度实际上等于瞬时；`0` 表示不能移动。如果 **Deceleration** 是 `0`，引擎会一次性拷贝 **Acceleration**；分层依赖可能让 Deceleration 相对后来的 Acceleration 改动变陈旧（多补丁单位会出现诡异的转向）。
- **死亡时间** —— `DeathTime = -1` 让单位引用为 Actor/长尾效果保持存活；这类单位多了会伤性能。过长的正数死亡时间可能让折跃训练/「建造中就死了」的 Actor 清理不同步，除非该 Actor 显式处理单位死亡。
- **Tech Alias + 需求** —— 社区报告（约 **5.0+ 补丁**）从 **Requirement / Validator** 查 **Tech Alias** 时，对 **自定义** 单位变体可能失败（别名没有按预期给形态分组）。把 Tech Alias 视为对自定义单位变体不可靠；优先用显式单位列表或游戏内验证。
- **武器** —— **武器** 要开火，需要一个可用的 **Attack** 技能。开 **Linked Cooldown** 时，**武器索引最低**（最接近 0）者优先。
