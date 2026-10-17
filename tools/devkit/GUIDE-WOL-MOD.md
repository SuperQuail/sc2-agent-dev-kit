# 自由之翼 mod 制作指导

> 全部结论来自实测（做「自由日」替换图的过程），不是推测。每条后面标注了可信度。

## 0. 一句话流程

```text
从 CASC 取原图 → normalize（源形态转分发形态）→ 改内容 → pack（单文件 MPQ）→ package（可导入的 zip）→ doctor（自检）
```

对应命令见 `tools/devkit/repro.sh`。**别手工改任何内部文件**——每一步都有命令。

## 1. 地形：地图到底在哪

| 你想要的 | 在哪 | 可信度 |
|---|---|---|
| 战役关卡（如自由日） | `SC2Data` 的 CASC 里，路径形如 `campaigns\liberty.sc2campaign\base.sc2maps\maps\campaign\traynor01.sc2map\` | ✅ 实测 |
| 它是**目录**还是 MPQ | CASC 里是**解开目录**（编辑器组件形态），不是 MPQ | ✅ 实测 |
| 同名文件可能是别人的改版 | 参考目录里的 `traynor01.SC2Map` 全是第三方改过的（依赖一堆无关 mod、单位被改名） | ✅ 实测 |

**取图的唯一可靠方式**：`casc extract` + **递归通配** `…traynor01.sc2map/**`。
用 `*` 会漏掉嵌套子目录——正好是最关键的本地化与本地 GameData。

## 2. 源形态 vs 分发形态（这是最容易栽的一步）

| | CASC 源形态 | 能跑的成品 |
|---|---|---|
| 语系目录名 | `zhcn.sc2data` 全小写 | `zhCN.SC2Data` **驼峰** |
| 语系数量 | 只带当前语系（中文版只有 `zhcn`） | **12 个全带** |
| `ComponentList` | 声明 12 个、文件可能缺 | 声明与文件一致 |

按源形态打包，**文本组件不生效，画面全是裸键**。`sc2cat normalize` 一次修好。

> 症状特征：游戏自身的 UI 字符串（`等级: 信徒`、`轻甲 - 生物单位`）是中文，只有地图/mod 的文本是裸键。

## 3. 改单位：**两份文件都要改**

| 文件 | 是什么 | 谁在用 |
|---|---|---|
| `Triggers` | 编辑器的**源** | 只有编辑器 |
| `MapScript.galaxy` | 从触发器编译出的 Galaxy | **游戏运行时执行的是它** |

只改 `Triggers` = 进游戏**毫无变化**。这是本项目最贵的一次踩坑。

`sc2cat set-unit --in-trigger <Id> --galaxy-fn <函数名>` 两边一起改。
- 图**没有 TriggerStrings** 时 `Id` 推不出函数名，只能手工给 `--galaxy-fn`
- 两个参数**必须一起给**（只给 `--galaxy-fn` 会把 Triggers 全图替换）

**怎么找 galaxmy 函数名**：`MapScript.galaxy` 里有 `// Trigger: <显示名>` 注释，函数名是 `gt_<显示名去掉空格>_Func`。

## 4. 单位可能被造**两次**

自由日的开场陆战队员：

| 触发器 | 处数 | 作用 |
|---|---|---|
| `Intro Cinematic` | 1 | `UnitCargoCreate(dropship, "Marine", 5)` —— 装进运输机 |
| `Intro Cleanup -  Escape` | 5 | 过场结束后**再造 5 个**给玩家操作 |
| `Briefing Scene 00` | 4 | 简报场景（敌方红方） |
| `Briefing Scene 01` | 4 | 简报场景（演员蓝方） |

**只改一处 = 只改一半**：过场里是狂热者，落地又变回陆战队员。

## 5. 协同作战单位需要「指挥官上下文」

移植 `ZeratulZealot` 后，技能面板是空的。原因：

```xml
<CRequirement id="ZeratulLevel03">
  <NodeArray index="Show" Link="AndCountUpgradeZeratulCommanderCompleteOnly GT3 CountUpgradeCommanderLevelCompleteOnly"/>
</CRequirement>
```

要求**指挥官身份 + 等级**——战役图里都没有。而 `GT3` 是 `3 > 等级`，所以**等级保持 0** 才显示锁定图标。

**修法**：在地图 `gt_Init01Technology_Func` 的动作段里插：

```galaxy
TechTreeUpgradeAddLevel(gv_p1_USER, "ZeratulCommander", 1);
TechTreeUpgradeAddLevel(gv_p1_USER, "CommanderLevel", 0);
```

`sc2cat inject --fn gt_Init01Technology_Func --code ...`
**注意 `inject` 只写 galaxy，不写 Triggers**——编辑器打开并保存这张图时会丢。

> ⚠️ 别自己定义同名 `CUpgrade`。先查依赖里有没有：这个 mod 的 `ZeratulCommander` 定义在
> `commanders/commanderzeratul.xml`、`CommanderLevel` 在 `UpgradeData.xml`。重复定义会**覆盖**掉它们。

## 6. 用游戏自带的合作模组（蒙斯克为例）

**蒙斯克单位在本地 CASC 里，不用打包**：

```text
mods\starcoop\commanders\arcturusmengsk.sc2mod\base.sc2data\gamedata\unitdata.xml   (181 KB, 64 个单位)
  TrooperMengsk / TrooperMengskImproved / ArtilleryMengsk / BattlecruiserMengsk / CommandCenterMengsk …
```

地图加一条依赖即可：

```text
file:Mods\StarCoop\Commanders\ArcturusMengsk.SC2Mod
```

**别再被这两个骗**（实测）：
- `mods/alliedcommanders.sc2mod` —— 只有 12 个语系的文本，**没有 GameData**
- `mods/liberty.sc2mod` 里 14 个 `*Mengsk*` 单位 —— **全是 UI 假单位**（`*ACGluescreenDummy`）和雕像

## 7. 本地化：三个坑

1. **文件要带 UTF-8 BOM**（Blizzard 的都有，手写的不带就读不到）
2. **一个语系目录里要放整套文件**（`ObjectStrings` / `GameStrings` / `ConversationStrings` / `GameHotkeys` / `TriggerStrings`），只放一个不生效
3. **设置字符串是替换不是追加**——文件里已有同名键时，**先出现的赢**，追加等于没改

`sc2cat localize` 处理 1 和 3。

### 依赖 mod 可能**把战役原文抄进了自己**

第三方 mod 常是「在某个战役基础上做的」，导出时会把**整个基础战役的文本**一起抄进来（实测那份 25,885 行、**0 个中文字**）。

后果：打进包后**战役原文变英文**。

```bash
sc2cat prune-locale <mod> --drop-prefix Conversation/ --drop-prefix ConversationState/ --drop-prefix Param/
```

裁掉 36,216 个键后恢复中文 ✓。**但哪些前缀算「战役专属」要判断，不能自动推。**

## 8. 任务简报界面：**改不了**（如实说）

你看到的「任务选择」界面右侧那个 3D 单位面板：

- 面板里的单位是 **`WarPig`**（WoL 战役里陆战队员的 id）——证据：`Button/Tooltip/WarPig=通用型步兵。` 与画面上那句一字不差 ✅
- **那一屏的字符串不在任何地图里**：`tstory01`（休伯利安号）、`traynor01` 的本地化里都搜不到「通用型步兵」「任务选择」「可用科技」 ✅
- 它们来自**战役包**（`Campaigns/Liberty.SC2Campaign`）✅

**结论：交付一张地图改不到那一屏。** 可行路径只剩两条，都还没验证：

| 路径 | 代价 |
|---|---|
| 用 mod 覆盖 `WarPig` 的 Actor/Model | **整场战役的陆战队员都会变** |
| 用 mod 覆盖战役包里的任务/UI 数据 | 需要先定位到具体文件（`campaigndata.xml` 只有 55 字节，是空壳，任务定义不在那儿） |

地图里的 `Briefing Scene 00/01` 触发器**不是那一屏**——它们是 `MovieStartRecording("Briefing_Video_TRaynor01_00")`，
**把简报录制成影片**；玩家看到的是 CASC 里已渲染的 `briefing_video_traynor01_0{0,1,2}.ogv`。

## 8. 任务简报界面：定位到一半（2026-10-17 追记）

### 8.1 已经证实的

| 结论 | 证据 |
|---|---|
| 面板里的单位是 `WarPig`（WoL 战役里陆战队员的 id） | 战役包写着 `Button/Tooltip/WarPig=通用型步兵。`，与画面一字不差 |
| **不在任何地图的脚本里** | `tstory01` 的 galaxy 里 `WarPig` 出现 **0 次**；`traynor01` 也没有 |
| **不在任何地图的 Objects 里** | `tstory01` 的 `Objects` 有 `Marine` / `SMMarine01` 等角色，**没有 `WarPig`** |
| **不在战役 GameData 的配置里** | 战役根 GameData（110 文件 / 5.2 MB）里 `WarPig` 只出现在常规表（unit/actor/model/abil/requirement/upgrade） |
| **不在战役的 layout 里** | 战役包只有 2 个 `.SC2Layout`：`wol_techtreepanel` / `descindex`，都不是 |
| 地图里的 `Briefing Scene 00/01` **不是那一屏** | 它们是 `MovieStartRecording("Briefing_Video_TRaynor01_00")` —— 把简报**录制成影片**；玩家看到的是 CASC 里已渲染的 `.ogv` |

### 8.2 还差最后一步

**「`WarPig` 是怎么绑到那个面板上的」没查出来。**

一个一次性定位实验（**还没做**）：

```text
做一个只改字符串、不改数据的 mod：
  Unit/Name/WarPig   = 萨尔那加狂热者
  Button/Tooltip/WarPig = <狂热者的说明>
跟着包一起装，进「任务选择」界面看：
  面板文字变了 → 绑定方式就是「按单位 id 取名字」，那换模型只需覆盖 WarPig 的 actor
  没变         → 绑定在别处，得继续找
```

### 8.3 为什么不能简单覆盖 `WarPig` 的 actor

因为**整个战役的陆战队员都是 `WarPig`** —— 覆盖它 = 全战役陆战队员换外观，不只是那一屏。
而 `tstory01` 是**独立地图**，它不会加载我们的 mod（mod 只被「依赖它的地图」加载）。

**所以那一屏要改，大概率只能交一张改过的 `tstory01.SC2Map`** ——
而它的面板单位不在脚本、不在 `Objects` 里，说明还得先搞清绑定方式。

## 9. 交付前的自检

`sc2cat doctor <zip 或目录>` 查五件事：

- 触发同步（Triggers 与 galaxy 都改了没）
- 语系结构（12 个驼峰目录）
- 依赖能否解析（磁盘找不到的按**警告**报——可能游戏自带在 CASC 里）
- `metadata.json` 形状（`mods` 必须是**数组**）
- 地图是不是单文件

**它查不出**「该改 14 处只改了 1 处」——那要写进任务书，且**可数**。

## 10. 分工：谁做不了什么

| 只能人做 | 原因 |
|---|---|
| 编辑器打开 / 另存为 Components | 需要账号登录 |
| 看编辑器告警 | GUI |
| 进游戏实测 | 没有可跑的客户端 |

**静态全绿 ≠ 能玩。** 判据分三层：`static → Editor accepted → packaged runtime passed`，不许越级。