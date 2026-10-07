# 星际争霸 II 编辑器中的 GalaxyScript 使用

## 概览

GalaxyScript 是 SC2 编辑器的编程语言。它大部分藏在触发器编辑器的 GUI 层后面，但你做的所有触发器工作最终都归结为 GalaxyScript —— 由编辑器自动生成到你的 MapScript.galaxy。
编辑器里很多东西归根到底不是字符串就是整数。比如对话框（Dialog）与对话框项，其实就只是整数。单位类型（或者说 90% 的数据引用）其实就只是字符串。编辑器会逼着你在众多内置类型之间来回转换，这种时候非常多。处理游戏链接（Game Link）时尤其烦人，因为它们常常拖慢甚至卡住游戏。

## 使用教程：创建你的第一个脚本

在文件资源管理器里打开地图，建一个叫 scripts 的文件夹。所有 galaxy 文件都放这里。

建一个叫 UI.galaxy 的文件。前面说过，"Create Dialog Item From Template" 函数容易把我的编辑器搞崩。我们写个包装函数，好从字符串创建对话框项。

在文件里复制/粘贴（或手写）下面这段代码：
```galaxy
int CreateDialogItemFromTemplate(int dialog, int type, string template)
{
    return DialogControlCreateFromTemplate(dialog, type, template);
}
```

接下来要把它导入编辑器才能用。先创建一个 Custom Script 对象（Ctrl+Alt+T），命名为 imports。然后加上这一行：
```galaxy
include "scripts/UI"
```

Custom Script 只是在生成时把脚本追加到 MapScript.galaxy 顶部。（如果你写过 C/C++，就相当于 #include <stdio.h>。）这样我们就能在 'main' 脚本以及它之后的所有脚本里使用那个文件里的函数。

现在建一个 native 函数，好真正用上写的脚本。新建函数，名字与脚本里的函数相同：CreateDialogItemFromTemplate。打开函数选项，勾上 "Native Function"。因为它返回对话框项，把返回值设为 Dialog Item。然后创建参数。
-Dialog 是 int，但要记住对话框就只是整数。所以它的类型是 Dialog。
-Type 是预设：对话框项的类型。
-Template 是字符串。

然后测一下。我用这个函数创建了一个对话框项，并在里面创建了一个按钮。启动测试地图，就完成了。
坦白说，上面做的事在 GUI 里也极其容易，不过是为了演示。

决定何时写脚本、何时用 GUI 时，我有几条经验规则：
1. 只写自包含函数。引用全局变量是可行的（只要在名字前加 gv_ 前缀），但通常别这么做。唯一的例外是常量，这应该很明显。
2. 任何与 catalog 相关的工作都用 Galaxy 做。GUI 函数很烦，而且处理字符串比处理游戏链接容易得多。
3. 烦人的类型转换，或频繁重复的东西（循环做不到），应该用 Galaxy 写。
4. 触发器创建永远用 GUI 做。

## 语法规范与句法规则

Galaxy 脚本是静态类型、命令式的语言，语法上像 C。但它缺少很多现代 C/C++ 或 C# 的构造。违反 Galaxy 的约束会导致编译错误或引擎脚本崩溃：

1. **局部变量提升（hoisting）：** 函数内所有局部变量必须定义在函数体的绝对开头，先于任何可执行语句、赋值或函数调用。在语句之后、或在内联块作用域内（例如 `if`、`while`、`for` 块里）声明变量是致命语法错误。
2. **自增 / 自减运算符：** 不支持后置自增（`i++`）与后置自减（`i--`）。必须用显式复合赋值（`i += 1;` 或 `i -= 1;`）。
3. **Include 指令：** 路径必须相对于根目录声明，且**不带文件扩展名**。写 `include "Scripts/Utility.galaxy"` 或 `#include "Scripts/Utility.h"` 会导致编译失败，而 `include "Scripts/Utility"` 才能正确解析。
4. **数据链接的表示：** GUI 触发器用 `gamelink` 这类带类型的包装对象，而原始 Galaxy 脚本用基元 `string` 与 catalog 条目、技能、单位类型打交道，用基元 `int` 与 UI/对话框句柄打交道。
5. **内存与结构体：** 结构体和数组是按引用传递。Galaxy 没有通过指针（`malloc`/`new`）做的动态堆分配。避免无限递归结构。
6. **异步逻辑与多线程：** Galaxy 函数默认在单一主线程上执行。长时间运行的循环或 `Wait()` 之类的阻塞调用会锁住执行，或超出引擎运行限制（`TriggerExecutionLimit`），除非通过 native 触发器或多线程 Action Definition 把它们派生到独立线程。

### 语法对比

| 语言特性 | 标准 C / C++ / C# 语法 | 星际争霸 II Galaxy 脚本标准 | 失败的引擎后果 |
|---|---|---|---|
| **局部变量作用域** | 可在块作用域内任意位置声明。 | 提升到函数体顶部，先于任何语句。 | 致命编译错误（`Unexpected token/symbol`）。 |
| **自增 / 自减** | `i++;` 或 `++i;` | `i += 1;` 或 `i = i + 1;` | 解析阶段语法错误。 |
| **Include 路径** | `#include "Scripts/Header.h"` | `include "Scripts/Header"`（无扩展名） | 文件打开失败 / 找不到 include 目标。 |
| **数据链接表示** | 带类型的类或 Enum 定义 | 基元 `string`（catalog）或 `int`（UI/对话框） | 函数调用时类型不匹配错误。 |
| **异步逻辑** | `async` / `await` 或线程池 | Native 触发器 / 多线程 Action Definition | 引擎脚本线程冻结 / 超出操作数限制。 |

### 局部变量提升（hoisting）示例

```galaxy
// INCORRECT: Variable declared after executable statements (causes fatal compile error)
void gf_ExecuteTacticalStrike_Bad(int lp_player, point lp_target) {
    UnitCreate(1, "Marine", 0, lp_player, lp_target, 270.0);
    unit lv_createdUnit = UnitLastCreated(); // FATAL ERROR: Declaration after statement
    lv_createdUnit += 1;
}

// CORRECT: Compliant Galaxy Script syntax
void gf_ExecuteTacticalStrike_Good(int lp_player, point lp_target) {
    // 1. Variable declarations hoisted to the top of the function
    unit lv_createdUnit;

    // 2. Procedural execution logic
    UnitCreate(1, "Marine", 0, lp_player, lp_target, 270.0);
    lv_createdUnit = UnitLastCreated();

    // 3. Increment operation using compound assignment
    // (Note: variable assignment and operations follow variable declarations)
}
```

## API 架构与命名空间

星际争霸 II 的脚本环境分为三个不同的运行层：

1. **引擎 Native：** 由引擎可执行文件直接暴露的核心原语（例如 `UnitCreate`、`OrderTargetingPoint`、`CatalogReferenceGet`、`UnitBehaviorGet`、`UnitKill`、`ActorSend`）。
2. **标准 Native 辅助（`libNtve`）：** 编译成 `libNtve_gf_*` 的包装函数，为常见操作提供简化接口（例如 `libNtve_gf_SetFacing`、`libNtve_gf_UnitInRegion`、`libNtve_gf_CinematicMode`）。
3. **战役支持库（`libCamp`）：** 处理战役剧情引擎、任务目标、传输、过场动画和行星面板的专用函数（`libCamp_gf_*`）。

### API 模块域参考

| API 模块域 | 主要函数命名空间 | 功能范围与执行上下文 |
|---|---|---|
| **核心引擎逻辑** | `UnitCreate`, `UnitKill`, `OrderApply`, `OrderTargetingPoint` | 实例化单位、施加空间移动命令、管理核心单位生命状态。 |
| **Actor 基础设施** | `ActorSend`, `ActorCreate`, `libNtve_gf_SetFacing` | 控制客户端视觉 actor、模型替换、朝向与动画括号。 |
| **Catalog 反射** | `CatalogReferenceGet`, `CatalogFieldValueSet` | 对 GameData XML catalog 元素做运行时动态查询与修改。 |
| **行为（Behavior）与状态** | `UnitBehaviorAdd`, `UnitBehaviorGet`, `UnitBehaviorHas`, `UnitBehaviorRemove` | 动态查询或修改单位的行为（Behavior）栈、状态效果与属性。 |
| **战役系统** | `libCamp_gf_SendTransmission`, `PlanetCreate` | 驱动剧情引擎序列、行星选择面板、UI 目标与传输。 |
| **UI 与对话框引擎** | `DialogCreate`, `DialogItemCreate`, `DialogSetPosition`, `DialogControlHookup` | 构造自定义用户界面元素、按钮、文本字段与框架挂接。 |

## 运行时 Catalog 反射（CatalogFieldValueSet / CatalogReferenceGet）

用 Galaxy 脚本在运行时动态操作 catalog 值时，字段路径必须使用点号分隔的字符串语法，镜像内部 XML 元素树结构：

```galaxy
// Dynamically adjusting catalog attributes at runtime
// The field path strictly mirrors the XML element tree structure
void gf_UpgradeUnitArmor(string lp_unitType, int lp_player, int lp_bonusArmor) {
    string lv_fieldPath;

    // Constructing raw path: corresponds to <CUnit><Armor value="..."/></CUnit>
    lv_fieldPath = "Armor";

    // CatalogFieldValueSet(Catalog, Entry, FieldPath, Player, Value)
    CatalogFieldValueSet(c_gameCatalogUnit, lp_unitType, lv_fieldPath, lp_player, IntToString(lp_bonusArmor));
}
```

## 从 Galaxy 评估数据验证器

当触发器逻辑或战术 AI 逻辑需要直接运行某个数据验证器（validator）时，用 `ValidatorExecute`。它的参数是验证器链接、来源单位与目标单位：

```galaxy
bool MyValidatorAcceptsTarget(unit source, unit target) {
    return ValidatorExecute("SomeValidator", source, target) > 0;
}
```

当问题是「整个效果（Effect），连同它的验证器链，是否接受某对施法者/目标」时，用 `UnitValidateEffectUnit`。`UnitValidateEffectPoint` 处理点目标；`PlayerValidateEffectUnit` 与 `PlayerValidateEffectPoint` 覆盖没有施法者单位的检查。这些效果（Effect）验证 native 使用命令错误约定，其中 `0` 表示接受：

```galaxy
bool MyEffectAcceptsTarget(unit caster, unit target) {
    return UnitValidateEffectUnit(caster, "SomeEffect", target) == 0;
}
```

不要混用成功判定：直接的 `ValidatorExecute` 与效果验证使用不同的结果约定。

来源与范围：暴雪安装的 Xel'Naga 战术 AI 脚本把 `ValidatorExecute` 结果中小于等于 `0` 的当作拒绝；安装的 native 辅助库则用「效果验证结果是否等于 `0`」来实现它的四个 `Player/Unit Can Create Effect` 条件。仓库生成的 native 表证实了这两个 API 及其参数类型。

## 示例

下面是一些通用、万用的工具/示例动作。所有变量名都意在自解释。以 c_ 开头的东西都是预设。

### 遍历玩家组
```galaxy
void DisplayMessageToPlayerGroup(string message, playergroup group)
{
    int player;

    for(player = 1; player <= CONST_MAX_PLAYERS; player+=1)
    {
        if(!PlayerGroupHasPlayer(group, player)){ continue; }
        UIDisplayMessage(PlayerGroupSingle(player), c_messageAreaChat, StringToText(message));
    }
}
```

### 动态函数注册
```galaxy
void CreateButtonAndRegisterToTrigger()
{
    // Create the Dialog Item
    int dialogitem = libNtve_gf_CreateDialogItemButton(
        CONST_DIALOG,
        300,
        75,
        c_anchorCenter,
        0,
        0,
        StringToText(""),
        StringToText("Click Me!"),
        ""
    );

    // Add it to global trigger TRIGGER_VARIABLE
    TriggerAddEventDialogControl(
        TRIGGER_VARIABLE,
        c_playerAny,
        dialogitem,
        c_triggerControlEventTypeClick
    );
}
```

### 创建触发器 / 创建异步函数
```galaxy
trigger MyGlobalTrigger;

// You can name these variables whatever you want; testConds/runActions is just the standard.
bool MyTrigger(bool testConds, bool runActions)
{
    // Echoes the chat message back to the player
    UIDisplayMessage(PlayerGroupSingle(EventPlayer()), c_messageAreaChat, StringToText(EventChatMessage()));

    return true;
}

void MyTrigger_Init()
{
    // Use the name of the function you want to execute as an argument
    MyGlobalTrigger = TriggerCreate("MyTrigger");
    TriggerAddEventChatMessage(MyGlobalTrigger, c_playerAny, "echo", false);
}
```

### 作用域
```galaxy
// This function cannot be accessed outside of this file
static bool ThisIsTrue()
{
    return true;
}


// This function can
bool TrueIsTrue()
{
    if(!ThisIsTrue())
    {
        return false;
    }
    else
    {
        return true;
    }
}
```

## 外部查阅

用 [../../../sc2-project-entry/references/external-sc2-resources.md](../../../sc2-project-entry/references/external-sc2-resources.md) 在 SC2Mapster、Talv Galaxy 与 Talv 数据文档之间做选择。简而言之：

- 需要精确的 native/库签名与预设名时，用 Talv Galaxy Reference。
- 生成触发器 XML 时用本地 [../../../sc2-map-triggers/references/triggers-native-functions.md](../../../sc2-map-triggers/references/triggers-native-functions.md)，因为它包含 `Ntve` ID 与参数 ID。
- 语言形态与编译器限制用 SC2Mapster Language Overview。

## Galaxy API 参考

**Mapster Galaxy Reference** —— 所有 native 与库函数的可搜索索引，带签名与参数类型：
https://mapster.talv.space/galaxy/reference/

### 有用函数（已确认）

#### 过场模式 —— 完整包装（恢复光标/HUD 就用它）
```galaxy
// Signature: void libNtve_gf_CinematicMode(bool lp_onOff, playergroup lp_players, fixed lp_duration)
// Caches UI state on turn-on and RESTORES it on turn-off (cursor, HUD, UISetMode).
// Use this when turning off cinematic mode before showing a dialog.
libNtve_gf_CinematicMode(false, PlayerGroupAll(), 0.0);
```
Reference: https://mapster.talv.space/galaxy/reference/lib-ntve-gf-cinematic-mode

#### 过场模式按玩家 —— 只切内部标志（不恢复光标/HUD）
```galaxy
// Signature: void libNtve_gf__CineModeTurnOnOffForPlayer(int lp_player, bool lp_onOff)
// Note the double underscore after gf_. Marked internal — only toggles the flag,
// does NOT restore cached UI state. Mouse cursor stays hidden if UISetMode was set
// to fullscreen. Do NOT use this when a dialog must follow.
libNtve_gf__CineModeTurnOnOffForPlayer(1, false);
```
Reference: https://mapster.talv.space/galaxy/reference/lib-ntve-gf-cine-mode-turn-on-off-for-player

#### 战术 AI 函数

单位可以通过两条路径获得自定义 AI 逻辑：

**路径 A —— 数据编辑器（简单规则）：** 在数据编辑器 → Units 页签里找到 `AI: Tactical AI Data`，不用写任何脚本就能建立基于规则的行为（Behavior）（例如「目标护盾 > 100 时用 EMP」）。

**路径 B —— Galaxy 脚本（完全控制）：**

1. 在数据编辑器 → Units 页签里，把 `AI: Tactical AI Function` 设成你的 Galaxy 函数名。
2. 用 Galaxy 写这个函数。签名必须是 `bool MyFunc(unit u, player p)`。采取了动作就返回 `true`（阻止默认 AI 运行），否则返回 `false` 继续往下走。
3. 在地图初始化触发器里用 `libNtve_gf_SetTacticalAIThink` 注册该函数。

```galaxy
bool MyTacticalAI(unit u, player p) {
    if (UnitGetPropertyInt(u, c_unitPropLifePercent, c_playerAny) < 20) {
        AICast(u, Order(AbilityCommand("move", 0), Point(10, 10)));
        return true;
    }
    return false;
}
```

**关键注意事项：**
- 如果触发器给单位下达了命令（例如 `IssueOrder`），该单位会变成 "Script Controlled"，可能忽略战术 AI 数据。
- 一个通用模式：遍历单位的技能并调用 `AIExecuteAbilTactical`，而不是按单位类型逐条写逻辑。

---

#### 任务胜利（模组作用域）
```galaxy
// Use the native call — NOT libSwaC_gf_EndCampaignMission.
// libSwaC is compiled into each HotS map but is NOT a dependency of the mod,
// so the mod compiler cannot resolve libSwaC_* functions.
GameOver(1, c_gameOverVictory, false, true);
```
`libSwaC_gf_EndCampaignMission` 只是包装暴雪自己的战役存档系统。用 CCM 的自定义战役通常用自己的 bank 代替。从模组里结束任务，`GameOver` 就够了。
