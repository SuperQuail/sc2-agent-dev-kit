# 虫族单位参考指南（HotS — 可选）

作为虫群之心虫族战役的**示例**提供。若使用人类/星灵或非 HotS 基底，请另建 `skills/sc2data/sc2data-units-reference/references/units.md` 放入你自己种族的 ID。当你的设计与原版 HotS 分道扬镳时，更新或替换本页。

> ⚠️ **关键命名陷阱：** 玩家可训练的那个可建造「虫群虫后」单位，其 `CUnit id="Queen"`。而 `CUnit id="SwarmQueen"` 是**完全不同的**单位——它是 zexpedition03 中 Niadra 的英雄形态，玩家无法建造。不要把两者搞混。

---

## 基础可建造单位

这些是标准 HotS 战役的可建造单位（原版 HotS 中，进化抉择后由变种取代基础单位）。

| 设计名 | CUnit ID | 潜地 CUnit ID | 备注 |
|---|---|---|---|
| Drone | `Drone` | `DroneBurrowed` | 工人 |
| Zergling | `Zergling` | `ZerglingBurrowed` | 进化任务后被变种取代 |
| Swarm Queen | `Queen` | `QueenBurrowed` | ⚠️ 不是 `SwarmQueen`——那是 Niadra |
| Roach | `Roach` | `RoachBurrowed` | 进化任务后被变种取代 |
| Hydralisk | `Hydralisk` | `HydraliskBurrowed` | 进化任务后由 `HydraliskLurker` 或 `HydraliskImpaler` 取代 |
| Baneling | `Baneling` | `BanelingBurrowed` | 进化任务后被变种取代 |
| Aberration | `InfestedAbomination` | `InfestedAbominationBurrowed` | 设计文档称其为 "Aberration" |
| Mutalisk | `Mutalisk` | *（空中——无潜地）* | 进化任务后由 `MutaliskBroodlord` 或 `MutaliskViper` 取代 |
| Swarm Host | `SwarmHost` | `SwarmHostBurrowed` | 进化任务后被变种取代 |
| Ultralisk | `Ultralisk` | `UltraliskBurrowed` | 进化任务后被变种取代 |

---

## 进化任务变种替换

完成进化任务后，所选变种会在此后所有任务中**永久取代**基础单位。未选择的变种无法建造。潜地变体遵循同样的 `<ID>Burrowed` 模式。

### 跳虫 → 猛禽或虫群（zevolutionzergling）
| 变种 | CUnit ID | 潜地 |
|---|---|---|
| Raptor Strain | `HotSRaptor` | `HotSRaptorBurrowed` |
| Swarmling Strain | `HotSSwarmling` | `HotSSwarmlingBurrowed` |

### 爆虫 → 猎手或裂变（zevolutionbaneling）
| 变种 | CUnit ID | 潜地 |
|---|---|---|
| Hunter Strain | `HotSHunter` | `HotSHunterBurrowed` |
| Splinterling Strain | `HotSSplitterlingBig` | `HotSSplitterlingBigBurrowed` |

### 蟑螂 → 尸兽或邪毒（zevolutionroach）
| 变种 | CUnit ID | 潜地 |
|---|---|---|
| Corpser Strain | `RoachCorpser` | `RoachCorpserBurrowed` |
| Vile Strain | `RoachVile` | `RoachVileBurrowed` |

### 刺蛇 → 潜伏者或穿刺者形态（zevolutionhydralisk）

刺蛇的视觉表现与技能都会改变。从游戏性角度看，新单位仍是「刺蛇」，但现在可以变形为所选的高级单位。

| 选择 | 刺蛇形态 CUnit ID | 潜地 | 变形为 | 变形后 CUnit ID |
|---|---|---|---|---|
| 潜伏者路线 | `HydraliskLurker` | `HydraliskLurkerBurrowed` | Lurker | `Lurker` |
| 穿刺者路线 | `HydraliskImpaler` | `HydraliskImpalerBurrowed` | Impaler | `Impaler` |

> `HydraliskLurker` 与 `HydraliskImpaler` 是**刺蛇单位**（可从孵化场训练）。`Lurker` 与 `Impaler` 是**变形后的形态**（类似蟑螂→破坏者的变形）。潜伏者潜地：`LurkerBurrowed`。穿刺者潜地：`ImpalerBurrowed`。潜地机制见下文说明。

### 飞龙 → 巢虫领主或毒蛇形态（zevolutionmutalisk）

与刺蛇同一模式。飞龙的形态改变，且可变形为所选单位。

| 选择 | 飞龙形态 CUnit ID | 变形为 | 变形后 CUnit ID | 备注 |
|---|---|---|---|---|
| 巢虫领主路线 | `MutaliskBroodlord` | Broodlord | `BroodLord` | 两者都是空中单位——无潜地 |
| 毒蛇路线 | `MutaliskViper` | Viper | `Viper` | 两者都是空中单位——无潜地 |

### 虫群宿主 → 变种 A 或 B（zevolutionswarmhost）
| 变种 | CUnit ID | 潜地 |
|---|---|---|
| Strain A | `SwarmHostSplitA` | `SwarmHostSplitABurrowed` |
| Strain B | `SwarmHostSplitB` | `SwarmHostSplitBBurrowed` |

### 雷兽 → 剧毒或 Torrasque（zevolutionultralisk）
| 变种 | CUnit ID | 潜地 |
|---|---|---|
| Noxious Strain | `HotSNoxious` | `HotSNoxiousBurrowed` |
| Torrasque Strain | `HotSTorrasque` | `HotSTorrasqueBurrowed` |

---

## 空中单位（无潜地技能）

| 单位 | CUnit ID | 备注 |
|---|---|---|
| Mutalisk | `Mutalisk` | 进化任务后被变种取代 |
| Mutalisk (Broodlord path) | `MutaliskBroodlord` | 变形前形态 |
| Mutalisk (Viper path) | `MutaliskViper` | 变形前形态 |
| Broodlord | `BroodLord` | 由 `MutaliskBroodlord` 变形而来 |
| Viper | `Viper` | 由 `MutaliskViper` 变形而来 |
| Overlord | `Overlord` | 人口 + 运输 |
| Overseer | `Overseer` | 由 Overlord 变形而来；探测器 |
| Brood Queen | `QueenClassic` | 仅 KaldirChoice=1 时 |

---

## 特殊 / 英雄单位（玩家不可建造）

| 单位 | CUnit ID | 使用位置 |
|---|---|---|
| Niadra (Stage 1) | `SwarmQueen` | 仅 zexpedition03——⚠️ 不是可建造的虫群虫后 |
| Niadra (Stage 2) | `LargeSwarmQueen` | 仅 zexpedition03 |
| Niadra (Final) | `HugeSwarmQueen` | 仅 zexpedition03 |
| Niadra (Larva) | `LarvalQueen` | zexpedition03 开局——做挑战时可忽略 |
| Zagara | `ZaGara` | 泽鲁斯战略英雄 / 战役英雄；潜地形态为 `ZaGaraBurrowed` |
| Kerrigan (Ghost Lab) | `KerriganGhostLab` | 特定 HotS 实验室任务 |
| Kerrigan (Char form) | `KerriganChar` | 查尔 / 挑战变体 |
| Kerrigan (standard) | `K5Kerrigan` | 多数 HotS 战役任务 |

---

## 潜地机制

所有虫族地面单位都有潜地技能。潜地后，单位会：
- 切换到它的潜地 `CUnit id`（例如 `Roach` → `RoachBurrowed`）
- 变为**隐形**（无探测器时不可见）
- 可被其他单位**穿过**（无碰撞）
- 潜地状态下**无法攻击**——只有两个例外：

| 单位 | 攻击行为 |
|---|---|
| 其他所有地面单位 | 未潜地时可攻击；潜地时**不能**攻击 |
| `Lurker` / `LurkerBurrowed` | 未潜地时**不能**攻击；只能以 `LurkerBurrowed` 攻击 |
| `Impaler` / `ImpalerBurrowed` | 未潜地时**不能**攻击；只能以 `ImpalerBurrowed` 攻击 |

> **对 Galaxy 的影响：** 想搜出「所有蟑螂」来施加增益时，必须同时查询 `Roach` 与 `RoachBurrowed`，因为在单位组过滤中它们被视为不同单位类型。每个地面单位及其全部变种都是如此。

---

## Galaxy 单位组遍历规则

写「所有玩家的跳虫」（或任何可能已被变种取代的单位）的遍历代码时，必须包含全部可能的 ID：

```galaxy
// Example: apply buff to all Zergling-type units
// Must cover base unit + both strains + burrowed variants of each
string[6] zerglingTypes;
int i;
unitGroup ug;

zerglingTypes[0] = "Zergling";
zerglingTypes[1] = "ZerglingBurrowed";
zerglingTypes[2] = "HotSRaptor";
zerglingTypes[3] = "HotSRaptorBurrowed";
zerglingTypes[4] = "HotSSwarmling";
zerglingTypes[5] = "HotSSwarmlingBurrowed";
for (i = 0; i < 6; i += 1) {
    ug = UnitGroup(zerglingTypes[i], 1, UnitFilter(0,0,0,0), RegionEntireMap(), 200);
    // apply buff to each unit in ug...
}
```

> 实践中，对于任务开始时施加的挑战效果，用宽泛的 `UnitFilter` 配 `c_playerAny`，再在循环体里判断 `UnitGetType(u) == "Zergling"` 之类，可能更简单。具体挑战用哪种更清爽就用哪种。

---

## 单位 ID 的使用位置

| 场景 | 要编辑的文件 |
|---|---|
| `AffectedUnitArray`（单位卡上的按钮可见性） | `UpgradeData.xml` —— 每个单位 ID + 潜地变体 + 全部变种各加一条 |
| `EffectArray Reference="Unit,ID,..."`（属性修改） | `UpgradeData.xml` —— 必须逐条列出每个 ID；父升级**不会**自动覆盖变种 |
| Galaxy 单位组遍历 | `<configured mods_dir>/AeonOfIhanrii.SC2Mod/Base.SC2Data/Epi_Main.galaxy` —— 覆盖基础单位 + 变种 + 全部潜地/状态变体 |
| 针对玩家部队的挑战 / 增益代码 | 在项目模组自己的 `Base.SC2Data/Scripts/` 中实现 |

**SwarmHost 状态提醒：** SwarmHost 有 3 种状态 × 3 个单位变体 = 9 个 ID：`SwarmHost`、`SwarmHostRooted`、`SwarmHostBurrowed`、`SwarmHostSplitA`、`SwarmHostSplitARooted`、`SwarmHostSplitABurrowed`、`SwarmHostSplitB`、`SwarmHostSplitBRooted`、`SwarmHostSplitBBurrowed`。

---
