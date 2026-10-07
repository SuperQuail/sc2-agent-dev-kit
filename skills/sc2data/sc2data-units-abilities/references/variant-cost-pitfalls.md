# 变体花费陷阱（升空、交替状态、变形）

覆写人类或虫族单位花费时，必须同步对齐花费的那些单位配对；漏掉一个就会在游戏里静默扣费或退费。

## 人类建筑升空的花费陷阱

若干人类建筑可以**升空**飞行。每个都有**单独的** `CUnit` 目录条目，带 `Flying` 后缀。如果飞行形态继承的花费高于你覆写后的地面形态花费，**SC2 会在建筑升空或降落时扣掉差额**。

**规则：** 只要覆写人类建筑的花费，就必须一并覆写它的 `Flying` 变体使其对齐。

**已确认的升空变体**（PlanetaryFortress 无法升空，没有飞行变体）：

| 地面 CUnit | 飞行 CUnit |
|---|---|
| `CommandCenter` | `CommandCenterFlying` |
| `OrbitalCommand` | `OrbitalCommandFlying` |
| `Barracks` | `BarracksFlying` |
| `Factory` | `FactoryFlying` |
| `Starport` | `StarportFlying` |

## 人类建筑交替状态的花费陷阱

若干人类建筑有**交替状态的 CUnit 条目**，它们继承原版基础模组的花费。如果交替状态继承的花费与你覆写后的不同，SC2 会在状态切换时扣费或退费。

**SupplyDepotLowered** — 补给站降入地下时成为单独的 `CUnit`。务必让它的花费与 `SupplyDepot` 一致。

| 主 CUnit | 交替 CUnit | 触发 |
|---|---|---|
| `SupplyDepot` | `SupplyDepotLowered` | 玩家降下/升起补给站 |

**TechLab 挂件变体** — 每个能建造科技实验室的建筑都有自己的 CUnit（如 `BarracksTechLab`）。务必把三者都与通用 `TechLab` 的花费对齐。

| 通用 CUnit | 建筑专属 CUnit |
|---|---|
| `TechLab` | `BarracksTechLab` |
| `TechLab` | `FactoryTechLab` |
| `TechLab` | `StarportTechLab` |

**Reactor 挂件变体** — 与 TechLab 同一模式。务必把三者都与通用 `Reactor` 的花费对齐。

| 通用 CUnit | 建筑专属 CUnit |
|---|---|
| `Reactor` | `BarracksReactor` |
| `Reactor` | `FactoryReactor` |
| `Reactor` | `StarportReactor` |

**规则：** 只要覆写 `SupplyDepot`、`TechLab` 或 `Reactor`，就必须对齐它们全部的变体 CUnit ID。

## 虫族变形/潜地单位的花费陷阱

当虫族单位可以**拔根、潜地或变形**时，SC2 会为变形后的形态使用**单独的** `CUnit` 目录条目。如果该交替形态的 `CostResource`（继承自基础模组）高于主形态覆写后的花费，**SC2 会在单位状态切换时扣掉晶体矿/高能瓦斯差额**。

**规则：** 只要在 `UnitData.xml` 里覆写虫族单位的花费，就必须同步覆写它**所有**变形目标变体的花费使其对齐。

**已确认的变体 ID：**

| 主 CUnit | 变体 CUnit | 切换 |
|---|---|---|
| `SpineCrawler` | `SpineCrawlerUprooted` | 拔根/重新扎根 |
| `SporeCrawler` | `SporeCrawlerUprooted` | 拔根/重新扎根 |
| `SwarmHost` | `SwarmHostBurrowed` | 潜地攻击 |
| `Lurker` | `LurkerBurrowed` | 潜地攻击 |

**疑似存在变体的单位——写覆写之前先向用户确认目录 ID：**

| 主 CUnit | 疑似变体 | 需确认？ |
|---|---|---|
| `Zergling` | `ZerglingBurrowed` | 是 |
| `Drone` | `DroneBurrowed` | 是 |
| `Queen` | `QueenBurrowed` | 是 |
| `Roach` | `RoachBurrowed` | 是 |
| `Baneling` | `BanelingBurrowed` | 是 |
| `Hydralisk` | `HydraliskBurrowed` | 是 |
| `Infestor` | `InfestorBurrowed` | 是 |
| `Ultralisk` | `UltraliskBurrowed` | 是 |
| `Viper` | `ViperBurrowed` | 是 |

**工作流：** 修改任何虫族单位的花费时，到 sc2data-units-reference 查已知变体。如果变体 ID 不在上面的已确认清单里，**先请用户**在 SC2 数据编辑器中核实该目录 ID，再写 `CUnit` 覆写。

## 种族人口上限（`RaceData.xml`）

```xml
<CRace id="Prot">
    <StartingUnitArray index="1" Count="12"/>   <!-- starting workers -->
    <FoodCeiling value="275"/>                  <!-- max supply -->
</CRace>
```
