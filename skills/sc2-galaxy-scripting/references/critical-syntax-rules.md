# 关键语法规则（已确认的编译错误）

```galaxy
// Array declaration — CORRECT:
string[90] arr;
// NOT: string arr[90];

// Dialog/dialogitem variables are primitive int:
int myDialog;
int myButton;

// Global initializers — integer/fixed literals only:
int gMyCount = 0;
// NOT: int gMyDialog = c_invalidDialog;  // compile error

// Trigger action function signature:
bool MyAction(bool testConds, bool runActions) { ... }

// Get triggering unit:
lv_u = EventUnit();
// NOT: UnitFromEvent()  // does not exist

// Call Galaxy function from GUI trigger (use #PARAM):
libMy_SetCurrentMap(#PARAM(name));

// Local variables — MUST be at the TOP of function body, no inline init:
bool MyFunc(bool testConds, bool runActions) {
    trigger orbTrig;   // declared at top, no initializer
    if (!runActions) { return true; }
    orbTrig = TriggerCreate("Foo");  // assigned in body
    // NOT: trigger orbTrig = TriggerCreate("Foo");  // parse error
    // NOT: declared inside if/else/while  // invalid
}

// Unit created event — 4 params:
TriggerAddEventUnitCreated(myTrig, null, null, "");  // any unit
TriggerAddEventUnitCreated(myTrig, null, "Zergling", "");  // specific type
// NOT: TriggerAddEventUnitBirth(myTrig, null)
// NOT: TriggerAddEventUnitCreated(myTrig, null, null)  // needs 4th ""

// Read current max life:
fixed hp = UnitGetPropertyFixed(u, c_unitPropLifeMax, c_unitPropCurrent);
// NOT: c_unitPropMax  // constant does not exist

// Increments/decrements — compound assignment only:
count += 1;
// NOT: count++ or count--  // unsupported

// Loops — while only:
while (i < 10) { i += 1; }
// NOT: for (i=0; i<10; i++)  // unsupported

// Include directives — relative path, no extension:
include "Scripts/libMy_Globals"
// NOT: include "Scripts/libMy_Globals.galaxy"
// NOT: #include "Scripts/libMy_Globals.h"

// Dynamic catalog reflection paths mirror XML element tree:
CatalogFieldValueSet(c_gameCatalogUnit, "Marine", "Armor", player, "1");
CatalogFieldValueSet(c_gameCatalogUnit, "Marine", "CostResource[Minerals]", player, "75");
```

## 保守的循环/自增写法

手写 Custom Script 拿不准时，优先用 `i = i + 1;` 和 `while (...)`，而不是 `i += 1;`。源文档对 `+= ` 和 `break` 是否在所有 Galaxy 上下文里受支持说法不一（`galaxy-gotchas.md` 推荐 `+=`；触发器流程的坑则报告 `ScriptCode` 里会失败）。区分脚本上下文，让编辑器去编译。

## 局部变量初始化

为了兼容源记录中的 Custom Script 环境，把声明与运行时初始化分开——在函数体顶部声明，在可执行语句里赋值。
