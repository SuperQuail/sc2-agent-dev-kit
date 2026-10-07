# 建造与训练槽位（Void.SC2Mod）

已验证的 InfoArray 槽位到单位/建筑的映射，含 CAbilTrain 与 CAbilBuild 的标准时间，以及累计式变形花费陷阱。

## 训练技能（CAbilTrain）— 单位生产

`CAbilTrain` 决定建筑能生产哪些单位、各需多久。与 `CAbilBuild` 一样，每个单位占一个**顺序数字槽位**（`Train1`、`Train2` 等）——是槽位编号，**不是**单位的目录 ID。

> **关键陷阱：** 一律用 `Train1`、`Train2` 等。绝不要写单位目录 ID（例如 `index="Marine"`）——那会静默创建一个无人使用的条目。

```xml
<!-- Override Barracks train times -->
<CAbilTrain id="BarracksTrain">
    <InfoArray index="Train1" Time="2.5"/>   <!-- Marine -->
    <InfoArray index="Train2" Time="3.2"/>   <!-- Reaper -->
</CAbilTrain>
```

### Void.SC2Mod 标准训练槽位映射（据数据编辑器截图确认）

> **所有 (None) 槽位已确认是空的**——不要给它们赋值。在 XML 覆写中跳过它们是正确的、安全的。

**人类 — `CommandCenterTrain`**（CC / OC / PF）：

| 槽位 | 单位 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Train1` | SCV | `SCV` | 17 |

**人类 — `BarracksTrain`**：

| 槽位 | 单位 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Train1` | Marine | `Marine` | 25 |
| `Train2` | Reaper | `Reaper` | 32 |
| `Train3` | Ghost | `Ghost` | 29 |
| `Train4` | Marauder | `Marauder` | 30 |

**人类 — `FactoryTrain`**：

| 槽位 | 单位 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Train1` | *(None)* | — | — |
| `Train2` | Siege Tank | `SiegeTank` | 32 |
| `Train3` | *(None)* | — | — |
| `Train4` | *(None)* | — | — |
| `Train5` | Thor | `Thor` | 60 |
| `Train6` | Hellion | `Hellion` | 30 |
| `Train7` | Hellbat (Battle Mode) | `Hellbat` | 30 |
| `Train8` | Cyclone | `Cyclone` | 36 |
| `Train9`–`Train24` | *(None)* | — | — |
| `Train25` | Widow Mine | `WidowMine` | 40 |

**人类 — `StarportTrain`**：

| 槽位 | 单位 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Train1` | Medivac | `Medivac` | 30 |
| `Train2` | Banshee | `Banshee` | 43 |
| `Train3` | Raven | `Raven` | 43 |
| `Train4` | Battlecruiser | `Battlecruiser` | 90 |
| `Train5` | Viking (Fighter Mode) | `Viking` | 32 |
| `Train6` | *(None)* | — | — |
| `Train7` | Liberator (AA) | `Liberator` | 43 |

**星灵 — `NexusTrain`**：

| 槽位 | 单位 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Train1` | Probe | `Probe` | 17 |

**星灵 — `GatewayTrain` 与 `WarpGateTrain`**（槽位布局完全相同；两者必须一起覆写）：

| 槽位 | 单位 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Train1` | Zealot | `Zealot` | 38 |
| `Train2` | Stalker | `Stalker` | 42 |
| `Train3` | *(None)* | — | — |
| `Train4` | High Templar | `HighTemplar` | 55 |
| `Train5` | Dark Templar | `DarkTemplar` | 55 |
| `Train6` | Sentry | `Sentry` | 42 |
| `Train7` | Adept | `Adept` | 40 |

**星灵 — `RoboticsFacilityTrain`**：

| 槽位 | 单位 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Train1` | Warp Prism (Transport Mode) | `WarpPrism` | 36 |
| `Train2` | Observer | `Observer` | 21 |
| `Train3` | Colossus | `Colossus` | 54 |
| `Train4` | Immortal | `Immortal` | 39 |

**星灵 — `StargateTrain`**：

| 槽位 | 单位 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Train1` | Phoenix | `Phoenix` | 35 |
| `Train2` | *(None)* | — | — |
| `Train3` | Carrier | `Carrier` | 86 |
| `Train4` | *(None)* | — | — |
| `Train5` | Void Ray | `VoidRay` | 43 |
| `Train6` | *(None)* | — | — |
| `Train7` | *(None)* | — | — |
| `Train8` | *(None)* | — | — |
| `Train9` | Oracle | `Oracle` | 37 |
| `Train10` | Tempest | `Tempest` | 54 |

**虫族 — `LarvaWormhole`**（所有幼虫变形共用一个技能）：

| 槽位 | 单位 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Train1` | Drone | `Drone` | 17 |
| `Train2` | Zergling | `Zergling` | 24 |
| `Train3` | Overlord | `Overlord` | 25 |
| `Train4` | Hydralisk | `Hydralisk` | 33 |
| `Train5` | Mutalisk | `Mutalisk` | 33 |
| `Train6` | *(None)* | — | — |
| `Train7` | Ultralisk | `Ultralisk` | 55 |
| `Train8` | *(None)* | — | — |
| `Train9` | *(None)* | — | — |
| `Train10` | Roach | `Roach` | 27 |
| `Train11` | Infestor (Spellcaster) | `Infestor` | 50 |
| `Train12` | Corruptor | `Corruptor` | 40 |
| `Train13` | Viper | `Viper` | 40 |
| `Train14` | *(None)* | — | — |
| `Train15` | Swarm Host | `SwarmHost` | 36 |

## 建造技能（CAbilBuild）— 工人施工

`CAbilBuild` 决定工人单位能建造哪些建筑、各需多久。每个可建造建筑在 InfoArray 中占一个**顺序数字槽位**：

> **关键陷阱：** `index` 属性一律是 `Build1`、`Build2`、`Build3` 等——位置性槽位编号。它**不是**建筑的目录 ID。写 `index="CommandCenter"` 或 `index="Nexus"` 会静默创建一个新条目，既不会覆写正确的槽位，也没有任何游戏内效果。始终使用从数据编辑器 InfoArray 面板确认的数字 `BuildN` 格式。

```xml
<!-- Override Terran build times — only specify slots you change -->
<CAbilBuild id="TerranBuild">
    <InfoArray index="Build1"  Time="71"/>   <!-- CommandCenter -->
    <InfoArray index="Build4"  Time="46"/>   <!-- Barracks -->
    <InfoArray index="Build11" Time="43"/>   <!-- Factory -->
</CAbilBuild>
```

### Void.SC2Mod 标准建造槽位映射

三个工人建造技能（`TerranBuild`、`ProtossBuild`、`ZergBuild`）定义在 `Mods/Void.SC2Mod`。下表列出每个有效槽位、它对应的建筑，以及据数据编辑器确认的标准建造秒数。标为 *(None)* 的槽位在编辑器数组中存在但没有分配建筑——任何 XML 覆写都可以省略它们。

**TerranBuild** — SCV（`id="TerranBuild"`）：

| 索引 | 建筑 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Build1` | Command Center | `CommandCenter` | 71 |
| `Build2` | Supply Depot | `SupplyDepot` | 21 |
| `Build3` | Refinery | `Refinery` | 21 |
| `Build4` | Barracks | `Barracks` | 46 |
| `Build5` | Engineering Bay | `EngineeringBay` | 25 |
| `Build6` | Missile Turret | `MissileTurret` | 18 |
| `Build7` | Bunker | `Bunker` | 29 |
| `Build8` | Refinery (Rich) | `RefineryRich` | 21 |
| `Build9` | Sensor Tower | `SensorTower` | 18 |
| `Build10` | Ghost Academy | `GhostAcademy` | 29 |
| `Build11` | Factory | `Factory` | 43 |
| `Build12` | Starport | `Starport` | 36 |
| `Build13` | *(None)* | — | — |
| `Build14` | Armory | `Armory` | 46 |
| `Build15` | *(None)* | — | — |
| `Build16` | Fusion Core | `FusionCore` | 46 |

**ProtossBuild** — Probe（`id="ProtossBuild"`）：

| 索引 | 建筑 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Build1` | Nexus | `Nexus` | 71 |
| `Build2` | Pylon | `Pylon` | 21 |
| `Build3` | Assimilator | `Assimilator` | 21 |
| `Build4` | Gateway | `Gateway` | 46 |
| `Build5` | Forge | `Forge` | 25 |
| `Build6` | Fleet Beacon | `FleetBeacon` | 43 |
| `Build7` | Twilight Council | `TwilightCouncil` | 36 |
| `Build8` | Photon Cannon | `PhotonCannon` | 29 |
| `Build9` | *(None)* | — | — |
| `Build10` | Stargate | `Stargate` | 43 |
| `Build11` | Templar Archives | `TemplarArchive` | 36 |
| `Build12` | Dark Shrine | `DarkShrine` | 100 |
| `Build13` | Robotics Bay | `RoboticsBay` | 46 |
| `Build14` | Robotics Facility | `RoboticsFacility` | 46 |
| `Build15` | Cybernetics Core | `CyberneticsCore` | 36 |

**ZergBuild** — Drone（`id="ZergBuild"`）：

| 索引 | 建筑 | 编辑器 ID | 标准时间（秒） |
|---|---|---|---|
| `Build1` | Hatchery | `Hatchery` | 71 |
| `Build2` | Creep Tumor | `CreepTumor` | 15 |
| `Build3` | Extractor | `Extractor` | 21 |
| `Build4` | Spawning Pool | `SpawningPool` | 46 |
| `Build5` | Evolution Chamber | `EvolutionChamber` | 25 |
| `Build6` | Hydralisk Den | `HydraliskDen` | 33 |
| `Build7` | Spire | `Spire` | 71 |
| `Build8` | Ultralisk Cavern | `UltraliskCavern` | 46 |
| `Build9` | Infestation Pit | `InfestationPit` | 36 |
| `Build10` | Nydus Network | `NydusNetwork` | 36 |
| `Build11` | Baneling Nest | `BanelingNest` | 43 |
| `Build12` | *(None)* | — | — |
| `Build13` | *(None)* | — | — |
| `Build14` | Roach Warren | `RoachWarren` | 39 |
| `Build15` | Spine Crawler | `SpineCrawler` | 36 |
| `Build16` | Spore Crawler | `SporeCrawler` | 21 |

> **变形走 CAbilMorph，不走 CAbilBuild：** 虫族科技升级（Lair、Hive、Greater Spire）与人类主基地升级（Orbital Command、Planetary Fortress）都是 `CAbilMorph` 条目，不是 `CAbilBuild`。它们有各自的技能 ID 和不同的 InfoArray 键格式。

> **变形花费陷阱——CUnit 花费是累计的：** SC2 对一次变形收取的晶体矿/瓦斯为 `目标 CUnit 花费 − 源 CUnit 花费`。如果你调低了源单位的花费（例如 CommandCenter → 80M），却把变形目标（OrbitalCommand）只设成升级增量（30M），游戏会**退还**差额（30−80 = −50M）。**变形目标的 CUnit 花费一律设为 `源花费 + 升级增量`：**
> - `OrbitalCommand` = CC(80M) + upgrade(30M) = **110M**
> - `PlanetaryFortress` = CC(80M) + upgrade(30M) = **110M** 晶体矿，0V + 8V = **8V**
> - `Lair` = Hatchery(60M) + upgrade(30M) = **90M**，0V + 5V = **5V**
> - `Hive` = Lair(90M) + upgrade(40M) = **130M**，5V + 8V = **13V**
> - `GreaterSpire` = Spire(40M) + upgrade(20M) = **60M**，10V + 8V = **18V**
