# 虫族建筑参考指南（HotS — 可选）

HotS 虫族建筑的示例参考。其他种族可替换或补充；见 `setup.md`。

虫族建筑 ID、建造/变形时间，以及如何通过升级正确修改它们的参考。

---

## 建造时间实际是怎么运作的

**`Unit.BuildTime` 并不控制建造速度。** SC2 在升级中会忽略 `Reference="Unit,X,BuildTime"`——既不生效，也不报错。这里有两套独立系统：

| 建造方式 | 机制 | 升级引用的写法 |
|---|---|---|
| 工蜂变形为建筑 | `CAbilBuild` 的 `ZergBuild` 技能，`InfoArray[BuildX].Time` | `Abil,ZergBuild,InfoArray[Build1].Time` |
| 工蜂变形为特殊建筑 | 自定义 `CAbilBuild` 技能，`InfoArray[Build1].Time` | `Abil,<YourPrefix>BuildCustomStructure,InfoArray[Build1].Time`（自定义 `CAbilBuild`） |
| 建筑升级为建筑 | `CAbilMorph` 技能，`InfoArray[0].SectionArray[Y].DurationArray[Delay]` | `Abil,UpgradeToLair,InfoArray[0].SectionArray[Abils].DurationArray[Delay]` |
| 建筑生成建筑 | 生成方建筑上的 `CAbilBuild`，`InfoArray[Build1].Time` | `Abil,BuildNydusCanal,InfoArray[Build1].Time` |

`Operation="Multiply" Value="0.5"` 对两者都有效（已由 SC2 战役数据确认）。

---

## ZergBuild 技能 — 工蜂建造

所有工蜂→建筑的建造都走 `ZergBuild` 这一个技能。表中时间为 HotS 战役取值（Swarm Campaign 覆写了 Liberty Mod 的基础值）。

| InfoArray 索引 | 单位 ID | 建造时间（秒） | 来源 |
|---|---|---|---|
| `Build1` | `Hatchery` | 60 | Swarm Campaign 覆写（Liberty: 100） |
| `Build3` | `Extractor` | 30 | Liberty Mod |
| `Build4` | `SpawningPool` | 30 | Swarm Campaign 覆写（Liberty: 65） |
| `Build5` | `EvolutionChamber` | 40 | Swarm Campaign 覆写 |
| `Build6` | `HydraliskDen` | 40 | Liberty Mod |
| `Build7` | `Spire` | 40 | Swarm Campaign 覆写（Liberty: 100） |
| `Build8` | `UltraliskCavern` | 50 | Swarm Campaign 覆写（Liberty: 65） |
| `Build9` | `InfestationPit` | 40 | Swarm Campaign 覆写（Liberty: 50） |
| `Build10` | `NydusNetwork` | 50 | Liberty Mod |
| `Build11` | `BanelingNest` | 30 | Swarm Campaign 覆写（Liberty: 60） |
| `Build14` | `RoachWarren` | 40 | Swarm Campaign 覆写（Liberty: 55） |
| `Build15` | `SpineCrawler` | 30 | Swarm Campaign 覆写（Liberty: 50） |
| `Build16` | `SporeCrawler` | 30 | Liberty Mod |
| `Build17` | `LurkerDen` | 40 | Swarm Campaign 新增（仅 HotS） |
| `Build19` | `AutomatedExtractor` | 30 | Swarm Campaign 新增（仅 HotS） |
| `Build21` | `Digester` | 30 | Swarm Mod 新增 |

**LurkerDen 与 ImpalerDen：** 不存在 `ImpalerDen` 建筑。潜伏者与穿刺者两条路线都用 `LurkerDen`（Build17）。区别在于可训练的刺蛇单位是 `HydraliskLurker` 还是 `HydraliskImpaler`，而不在巢穴本身。

**指令面板花费显示：** 自定义工蜂建造按钮必须以 `Type="AbilCmd"` 链接到确切的建造命令（例如自定义单位用 `ZergBuild,Build22`，自定义 `CAbilBuild` 用 `<YourPrefix>Build...,Build1`）。悬停时显示的花费来自产出单位的 `CostResource` 加上资源提示标志；不要在按钮提示里手工加花费文字。修补工蜂建造子菜单时，继承的索引与原版卡片 ID 都要保留（`CardLayouts index="1" CardId="ZBl1"`、`index="2" CardId="ZBl2"`），这样补丁修改的是被打开的子菜单，而不是创建一个没人用的副本。

---

## CAbilMorph — 建筑到建筑的升级

这些变形**不涉及**工蜂。时间分摊在多个 SectionArray 上；要对游戏性生效，`Abils` 与 `Stats` 都要改（Actor 只控制动画）。

| 技能 ID | 从 → 到 | Abils 延时（秒） | Stats 延时（秒） | 来源 |
|---|---|---|---|---|
| `UpgradeToLair` | Hatchery → Lair | 60 | 60 | Swarm Campaign 覆写（Liberty: 80） |
| `UpgradeToHive` | Lair → Hive | 60 | 60 | Swarm Campaign 覆写（Liberty: 100） |
| `UpgradeToGreaterSpire` | Spire → GreaterSpire | 100 | 100 | Liberty Mod（Swarm 未覆写） |
| `UpgradeToLurkerDenMP` | HydraliskDen → LurkerDenMP | 100 | 100 | Swarm Mod（仅多人对战） |

**变形的升级引用写法：**
```xml
<EffectArray Operation="Multiply" Reference="Abil,UpgradeToLair,InfoArray[0].SectionArray[Abils].DurationArray[Delay]" Value="0.5"/>
<EffectArray Operation="Multiply" Reference="Abil,UpgradeToLair,InfoArray[0].SectionArray[Stats].DurationArray[Delay]" Value="0.5"/>
```

---

## 建筑的单位 ID

| 建筑 | CUnit ID | 备注 |
|---|---|---|
| Hatchery | `Hatchery` | |
| Lair | `Lair` | 由 Hatchery 变形而来 |
| Hive | `Hive` | 由 Lair 变形而来 |
| Spawning Pool | `SpawningPool` | |
| Baneling Nest | `BanelingNest` | |
| Roach Warren | `RoachWarren` | |
| Hydralisk Den | `HydraliskDen` | |
| Lurker Den | `LurkerDen` | 战役单位（Build17）；同时覆盖潜伏者/穿刺者两条路线 |
| Infestation Pit | `InfestationPit` | |
| Spire | `Spire` | |
| Greater Spire | `GreaterSpire` | 由 Spire 变形而来 |
| Ultralisk Cavern | `UltraliskCavern` | |
| Evolution Chamber | `EvolutionChamber` | |
| Spine Crawler | `SpineCrawler` | |
| Spore Crawler | `SporeCrawler` | |
| Extractor | `Extractor` | |
| Automated Extractor | `AutomatedExtractor` | 仅 HotS 战役 |
| Nydus Network | `NydusNetwork` | 由工蜂建造（ZergBuild Build10，50 秒）；由战略抉择解锁 |
| Greater Nydus Worm | `GreaterNydusWorm` | 由 NydusNetwork 经 `BuildNydusCanal` 与 `BuildGreaterNydusWorm` 生成（5 秒，10 秒冷却） |

---

## 常见陷阱

- **`Unit.BuildTime` 在升级里毫无作用** —— 已由它在全部 SC2 战役数据中的缺席确认。
- **指令面板花费显示需要真实的技能-命令接线** —— 按钮必须指向产出它的 `CAbilBuild`/`CAbilTrain` InfoArray 槽位，且产出单位/按钮不能隐藏资源。在 `Button/Tooltip` 里手写花费文字只对非标准命令作为兜底，应当避免。
- **`Operation="Multiply"` 有效**，无论 `Abil` 时间还是 DurationArray 引用（与 `CostResource` 不同，后者上它无效）。
- **建筑变形时间** 每个变形需要两条 EffectArray 条目：一条给 `SectionArray[Abils]`，一条给 `SectionArray[Stats]`。漏掉任一条都会有一半计时器不受影响。
- **`UpgradeToLurkerDenMP`** 是多人对战的变形技能——HotS 战役不使用。战役改用 ZergBuild Build17。
- **Nydus Worm** 有两个生成技能（`BuildNydusCanal` 与 `BuildGreaterNydusWorm`）。要让建造时间缩减生效就必须两个都引用。`BuildNydusCanal` 是战役里常用的覆写目标；`BuildGreaterNydusWorm` 为 5 秒、10 秒冷却。
