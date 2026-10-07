# 核心文件职责 —— Enums、GlobalVariables、Header

标准 `scripts/` 布局里每个文件负责什么，以及常量、全局状态与前置声明的写法。

## 文件职责与命名规范

| 文件 | 职责 |
|---|---|
| `MapScript.galaxy` | **编辑器自动生成。绝不编辑。** 入口，负责接线库并调用 `InitMap()`。 |
| `scripts/main.galaxy` | 协调器：所有 include + `void main()`。 |
| `scripts/Enums.galaxy` | 纯 `const int` 常量，用注释头按类别分组。 |
| `scripts/GlobalVariables.galaxy` | 所有全局 `const`、`static`、结构体定义与 `typedef funcref`。 |
| `scripts/Header.galaxy` | 每个跨文件函数的前置声明。 |
| `scripts/Utilities.galaxy` | 被许多其他文件使用的共享辅助函数。 |
| `scripts/MapInit.galaxy` | 地图初始化逻辑（联盟、玩家组、启动序列）。 |
| `scripts/Enemy.galaxy` | 敌人刷兵与行为。 |
| `scripts/Bank.galaxy` | Bank 读写逻辑。 |
| `scripts/Player.galaxy` | 玩家属性、经验、升级。 |
| `scripts/HeroSelection.galaxy` | 英雄选择 UI 与解锁逻辑。 |
| `scripts/HeroAbilities.galaxy` | 英雄技能实现。 |
| `scripts/Parts.galaxy` | 多分块内容的协调器（调用 Part* 文件）。 |
| `scripts/PartTerran.galaxy` | 泰伦分块专属内容。 |
| `scripts/PartProtoss.galaxy` | 神族分块专属内容。 |
| `scripts/PartZerg.galaxy` | 虫族分块专属内容。 |
| `scripts/Debug.galaxy` | 调试辅助、覆盖函数。 |
| `scripts/UI/UI-Main.galaxy` | 初始化所有 UI 子系统。 |
| `scripts/UI/UI-*.galaxy` | 每个 UI 面板/系统一个文件。 |
| `scripts/Lib/Starcode.galaxy` | 第三方库。 |

---

## Enums.galaxy —— 纯常量文件

所有全局整数常量都放进同一个文件：

```galaxy
// parts
const int c_Part_Terran  = 0;
const int c_Part_Protoss = 1;
const int c_Part_Zerg    = 2;

// game modes
const int c_GameMode_Normal  = 0;
const int c_GameMode_Endless = 1;

// bitflags
const int c_BossFightState_Alive   = 1 << 0;
const int c_BossFightState_Enraged = 1 << 1;
const int c_BossFightState_Dead    = 1 << 2;

// boss IDs
const int c_BossFightID_Flamer  = 0;
const int c_BossFightID_Crusher = 1;
```

**命名规范：** `c_CategoryName_IdentifierName`
**分组：** 用 `// category name` 注释头分隔各组。
**位标志：** 用 `1 << n` 模式。

`Enums.galaxy` 要在 `GlobalVariables.galaxy` 之前 include，这样结构体字段才能引用这些常量做数组长度。

---

## GlobalVariables.galaxy —— 结构体与全局状态

所有结构体、全局变量与 `typedef funcref` 都在这里声明：

```galaxy
// global consts
const int gv_MaxAmountPlayers = 6;
const int gv_MaxAmountParts   = 3;

// funcref blueprints
bool blueprint_BossAbility(unitgroup ug, unit boss);
typedef funcref<blueprint_BossAbility> Blueprint_BossAbility;

// structs
struct PlayerStruct {
    bool activeFlag;
    bank bankfile;
    unit heroUnit;
    int points;
    int[gv_MaxAmountParts] wins;   // array sized by const
    int heroUnlocked;              // stored as bitflag
};

// global arrays
PlayerStruct[gv_MaxAmountPlayers + 1] gv_PlayerStats;
```

**命名规范：** 全局用 `gv_SystemName_VariableName`，简单全局用 `gv_VariableName`。
**跨触发器的静态局部：** 在拥有它们的文件里以文件作用域 `static` 声明。

---

## Header.galaxy —— 前置声明

Galaxy 要求函数先声明后调用。当文件 A 调用更晚 include 的文件 B 里定义的函数时，在 `Header.galaxy` 写前置声明：

```galaxy
// Header.galaxy — comment groups match their source file

// UI-PlayerBoard.galaxy
void PlayerBoard_UpdatePlayer(int playerID);

// Bank.galaxy
void Bank_Save_ForcedAll(int playerID);
void Bank_Save_RequestSave(int playerID);

// Parts.galaxy
void Part_PartFinished();
void Part_InitVariables();

// Player.galaxy
void Player_AddExp(int playerID, fixed amount);
```

`Header.galaxy` 要在 `GlobalVariables.galaxy` **之后** include，这样声明才能引用结构体类型。
