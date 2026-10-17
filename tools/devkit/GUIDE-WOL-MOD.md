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

## 8. 任务简报界面：改得动（**我最初的判断是错的**）

### 8.1 先认错

我一开始断定面板单位是 `WarPig`，理由是 `Button/Tooltip/WarPig=通用型步兵。` 与画面一字不差。
**这是巧合。** 复核后：

| 键 | 真实值 | 对不对得上画面 |
|---|---|---|
| `Unit/Name/WarPig` | **战地豪猪** | ✗ 画面标题是「陆战队员」 |
| `Button/Tooltip/WarPig` | 通用型步兵。**<n/><n/>可以对地和对空。** | ✗ 画面只有前半句 |
| `Param/Value/lib_Camp_A8DF01B6` | **通用型步兵。** | ✅ 一字不差 |

**教训：字符串对上一半就下结论，是给自己挖坑。** 要找的是**唯一命中**，不是「像」。

### 8.2 真实来源

面板 = **PlanetPanel（星球面板）的「可用科技」区块**：

| 元素 | 出处 |
|---|---|
| 布局 | `mods/core.sc2mod/…/ui/layout/ui/planetpanel.sc2layout`（`TechnologyUnitModelFrame` / `TechnologyTextLabel` / `TechnologyDescriptionLabel`） |
| 单位 id | `Campaigns/LibertyStory.SC2Campaign` 里 `campaignlib.galaxy:7438` 的 `lv_unitType = "Marine";` |
| 填充 | `campaignlib.galaxy:6384` → `PlanetSetTechnologyUnitLink(PlanetLastCreated(), …)` |

**注意包名**：是 **`LibertyStory.SC2Campaign`**，不是 `Liberty.SC2Campaign` ——
`tstory01` 的 `DocumentHeader` 里有**第三条依赖**就是它。我之前只查了后者，所以什么都找不到。

### 8.3 改法（局部，符合规则 R1）

在**承载任务选择屏的地图**（原版 `tstory01`）里，`AddPlanet` 之后重设那个 Planet 对象：

```galaxy
lv_planet = libCamp_gf_AddPlanet(lp_planetID);
if ((lp_planetID == libCamp_ge_PlanetID_PlanetMarSara)) {
    PlanetSetTechnologyUnitLink(lv_planet, "ZeratulZealot");
    PlanetSetTechnologyName(lv_planet, UnitTypeGetName("ZeratulZealot"));
    PlanetSetTechnologyText(lv_planet, StringExternal("Param/Value/你的文本键"));
}
```

**三条必须一起设** —— 只改 link 会出现「模型是狂热者、标题还写陆战队员」。

```bash
sc2cat inject tstory01.SC2Map \
  --after 'lv_planet = libCamp_gf_AddPlanet(lp_planetID);' \
  --code '…' --code '…'
```

**为什么是局部的**：只写运行期 `PlanetCreate` 出来、随地图卸载消失的一个对象字段；
**完全没碰 catalog** —— `Marine` 还是 `Marine`，战役里其他陆战队员、军械库、科技树都不变；
其他地图加载时 `libCamp` 会重新写回 `Marine`，不外溢。

**被否决的三条**（都是全局）：

| 方案 | 为什么不行 |
|---|---|
| 改 `campaignlib.galaxy:7438` 的 `"Marine"` | 同一张 `tS_TechGroupUnitType` 表还喂 `PurchaseGroupSetUnitLink`（军械库/科技树/战报），全战役一起变 |
| 覆盖 `Marine`/`WarPig` 等自带 id | 规则 R1 |
| 改 `planetpanel.sc2layout` 模板 | 所有星球 + 所有战役共用 |

### 8.4 待实测

- 没跑游戏。方案已把调用放在**面板显示之前**，绕开「显示后能否即时刷新」的不确定性
- 没验证萨尔那加狂热者的 actor 能否在 `UnitModelFrame` 里正常渲染（取决于它的模型/相机）

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