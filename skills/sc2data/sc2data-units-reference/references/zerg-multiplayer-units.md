# 虫族多人单位与建筑

标准虫族多人单位与建筑的编辑器 ID、资料片、类型、属性与定位。

## 虫族

### 单位

| 显示名 | 编辑器 ID | 资料片 | 类型 | 属性 | 定位 |
|---|---|---|---|---|---|
| Drone | `Drone` | WoL | 工人 | Light · Biological | 建造/采集；可变形为建筑 |
| Zergling | `Zergling` | WoL | 步兵 | Light · Biological | 快速廉价的近战虫群单位 |
| Baneling | `Baneling` | WoL | 步兵 | Light · Biological | 跳虫变形；自爆造成 AoE 酸液伤害 |
| Roach | `Roach` | WoL | 步兵 | Armored · Biological | 耐打的短程单位，潜地快速回复 |
| Hydralisk | `Hydralisk` | WoL | 步兵 | Light · Biological | 对空/对地远程攻击者 |
| Infestor | `Infestor` | WoL | 步兵 | Light · Biological · Psionic | 真菌滋生、被感染人类、神经寄生 |
| Corruptor | `Corruptor` | WoL | 舰船 | Light · Biological · Psionic | 对空；可变异为巢虫领主 |
| Mutalisk | `Mutalisk` | WoL | 舰船 | Light · Biological | 快速骚扰飞行单位，攻击可弹射 |
| Brood Lord | `BroodLord` | WoL | 舰船 | Armored · Biological · Massive · Psionic | 空中攻城；播撒巢虫 |
| Broodling | `Broodling` | WoL | 步兵 | Light · Biological | 巢虫领主生成的短命近战单位 |
| Ultralisk | `Ultralisk` | WoL | 步兵 | Armored · Biological · Massive · Psionic | 重型近战，凯撒刃 AoE |
| Queen | `Queen` | WoL | 步兵 | Armored · Biological · Psionic | 输血、菌毯肿瘤、孵化幼虫；基地防御 |
| Overlord | `Overlord` | WoL | 舰船 | Armored · Biological · Massive · Psionic | 提供人口；可升级为运输/监督者 |
| Overseer | `Overseer` | WoL | 舰船 | Armored · Biological · Massive · Psionic | 领主变形：探测器/侦察支援 |
| Infested Terran | `InfestedTerran` | WoL | 步兵 | Light · Biological | 感染虫生成的限时攻击者 |
| Changeling | `Changeling` | WoL | 步兵 | Light · Biological | 监督者生成的间谍；可伪装成敌方单位 |
| Nydus Worm | `NydusCanal` | WoL | 建筑 | Armored · Biological · Massive · Structure | 坑道网络的出口 |
| Swarm Host | `SwarmHost` | HotS | 步兵 | Armored · Biological | 攻城单位；潜地后周期性生成蝗虫 |
| Swarm Host (Burrowed) | `SwarmHostBurrowed` | HotS | 步兵 | Armored · Biological | 生成蝗虫的潜地激活状态 |
| Locust | `Locust` | HotS | 步兵 | Light · Biological | 虫群宿主生成的限时近战单位 |
| Viper | `Viper` | LotV | 舰船 | Light · Biological · Psionic | 绑架、致盲云、寄生炸弹的空中支援 |
| Lurker | `Lurker` | LotV | 步兵 | Armored · Biological · Psionic | 刺蛇变形；潜地后成线攻击 |
| Lurker (Burrowed) | `LurkerBurrowed` | LotV | 步兵 | Armored · Biological · Psionic | 潜伏者的潜地/激活状态 |
| Ravager | `Ravager` | LotV | 步兵 | Armored · Biological | 蟑螂变形；腐蚀胆汁炮击 |

### 建筑

| 显示名 | 编辑器 ID | 资料片 | 备注 |
|---|---|---|---|
| Hatchery | `Hatchery` | WoL | 主基地（一级）；孵化幼虫 |
| Lair | `Lair` | WoL | 孵化场变形（二级） |
| Hive | `Hive` | WoL | 巢穴变形（三级） |
| Extractor | `Extractor` | WoL | 瓦斯采集建筑 |
| Spawning Pool | `SpawningPool` | WoL | 解锁跳虫；生产虫后 |
| Evolution Chamber | `EvolutionChamber` | WoL | 升级地面近战/远程/甲壳 |
| Baneling Nest | `BanelingNest` | WoL | 解锁爆虫变形 |
| Roach Warren | `RoachWarren` | WoL | 解锁蟑螂；潜地升级 |
| Hydralisk Den | `HydraliskDen` | WoL | 解锁刺蛇 |
| Infestation Pit | `InfestationPit` | WoL | 感染虫、巢穴的前置 |
| Spire | `Spire` | WoL | 解锁飞龙、腐蚀者、领主变形 |
| Greater Spire | `GreaterSpire` | WoL | 尖塔变形；解锁巢虫领主 |
| Ultralisk Cavern | `UltraliskCavern` | WoL | 解锁雷兽 |
| Nydus Network | `NydusNetwork` | WoL | 可在任意菌毯上创建坑道虫通道 |
| Spine Crawler | `SpineCrawler` | WoL | 仅对地的防御建筑；可拔根移动——拔根形态：`SpineCrawlerUprooted` |
| Spore Crawler | `SporeCrawler` | WoL | 对空探测建筑；可拔根移动——拔根形态：`SporeCrawlerUprooted` |
| Creep Tumor | `CreepTumor` | WoL | 扩散菌毯；由虫后（潜地）放置 |
| Creep Tumor (Burrowed) | `CreepTumorBurrowed` | WoL | 扩散激活状态 |
| Lurker Den | `LurkerDen` | LotV | 解锁刺蛇变形为潜伏者 |

> **花费覆写陷阱——拔根/潜地变体：** 脊针爬虫与孢子爬虫有单独的拔根 CUnit 条目（`SpineCrawlerUprooted`、`SporeCrawlerUprooted`）。多数虫族潜地单位也有独立的 CUnit 目录条目。如果变体继承的花费高于主单位覆写后的花费，SC2 会在状态切换时扣掉差额。两者都要覆写。若不知道变体的目录 ID，先向用户确认再写覆写。完整陷阱表见 `sc2data-units-abilities` 技能。
