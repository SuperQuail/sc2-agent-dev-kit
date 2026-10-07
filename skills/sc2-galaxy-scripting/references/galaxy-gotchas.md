# Galaxy 语言陷阱

星际争霸 II Galaxy 脚本中已确认的编译错误与 API 意外行为。写任何 Galaxy 代码前先查这里。

完整的 Galaxy 语言入门见 [skills/galaxy/galaxy-language-fundamentals/references/galaxy-language.md](../../galaxy/galaxy-language-fundamentals/references/galaxy-language.md)。  
SC2 Bank 函数签名见 [skills/sc2-bank-system/references/galaxy-bank.md](../../sc2-bank-system/references/galaxy-bank.md)。

---

## 已确认的编译错误与语法规则

### Custom Script 的 XML 缩进

在当前 Triggers 布局里写 FunctionCall 的 `<ScriptCode>` 时，
给每一行代码加上编辑器自写脚本块所用的 16 个空格前缀。
编辑器的代码生成会去掉这个缩进；没有缩进的代码可能丢掉每条语句最前面的 16 个字符。
一次已上报的 Bank 诊断编译错误显示 `if (EventChatMessage(false)` 变成了 `sage(false)`。
编辑器保存后检查生成的代码。XML 解析与普通的
catalog/Galaxy 校验器不会编译内嵌的 Custom Script。

```galaxy
// Array declaration — CORRECT syntax:
string[90] arr;
// NOT: string arr[90];

// Dialog/dialogitem variables — always primitive int:
int myDialog;
int myButton;

// Global variable initializers — integer/fixed literals only:
int gMyCount = 0;
// NOT: int gMyDialog = c_invalidDialog;  <-- causes compile error

// Trigger action function signature:
bool MyAction(bool testConds, bool runActions) { ... }

// Get the unit that triggered a unit event:
lv_u = EventUnit();
// NOT: UnitFromEvent()  <-- does not exist, causes syntax error

// Calling a galaxy function from a GUI trigger (use #PARAM):
MySetCurrentMap(#PARAM(name));  // use your Galaxy function prefix from AGENTS.md

// Local variable declarations — must be at the TOP of the function body:
bool MyFunc(bool testConds, bool runActions) {
    trigger orbTrig;   // correct: declared at top, no initializer
    if (!runActions) { return true; }
    orbTrig = TriggerCreate("Foo");  // correct: assigned in body
    // NOT: trigger orbTrig = TriggerCreate("Foo");  <-- inline init causes parse error
    // NOT: declared inside an if/else/while block   <-- also invalid
}

// Unit created event — correct function and parameter count (4 params):
TriggerAddEventUnitCreated(myTrig, null, null, "");  // any unit
TriggerAddEventUnitCreated(myTrig, null, "Zergling", "");  // specific type
// NOT: TriggerAddEventUnitBirth(myTrig, null)  <-- function does not exist
// NOT: TriggerAddEventUnitCreated(myTrig, null, null)  <-- wrong param count (needs 4th "" arg)

// Reading a unit's current max life (or any max-value property):
fixed hp = UnitGetPropertyFixed(u, c_unitPropLifeMax, c_unitPropCurrent);
// NOT: c_unitPropMax  <-- constant does not exist, causes "Invalid parameter"

// Increments / Decrements — use compound assignment:
count += 1;
// NOT: count++; or count--;  <-- unsupported operator, causes parse error

// Loops — use while (...):
while (i < 10) {
    // ...
    i += 1;
}
// NOT: for (i = 0; i < 10; i++)  <-- unsupported in SC2 custom script

// Include directives — relative path without extension:
include "Scripts/MyGlobals"
include "Scripts/MyBank"
// NOT: include "Scripts/MyGlobals.galaxy";  <-- causes file open failure
// NOT: #include "Scripts/MyGlobals.h";      <-- unsupported C preprocessor directive

// Dynamic catalog reflection paths must mirror XML element tree structure:
CatalogFieldValueSet(c_gameCatalogUnit, "Marine", "Armor", player, "1");
CatalogFieldValueSet(c_gameCatalogUnit, "Marine", "CostResource[Minerals]", player, "75");
```

---

## 模块化脚本边界规则

1. **顶层声明：** 每个被 include 的 `.galaxy` 文件必须从花括号深度 0 的声明开始，并以闭合的函数花括号 `}` 结束。把函数体拆到文件边界之外会产生编译错误。
2. **重复函数：** 在多个被 include 的脚本里定义同名函数会以 `"function already defined: <function_name>"` 编译失败。
3. **前向引用：** 在脚本块内，函数必须 **先定义后调用**。来自 GUI 触发器的调用独立链接，但脚本到脚本的内部调用要求被调用者在编译顺序中出现在前面。

---

## 社区错误分诊

- `Can only pass basic types`：结构体与数组不能当普通函数参数传。传标量 ID/索引，或改用全局数据结构。
- `struct forward declaration not supported`：把结构体定义放在使用它的函数或变量之前。
- `Bulk copy not supported`：按值赋整个数组/结构体会失败。显式逐个复制标量元素。
- `Could not allocate Global Memory` / `e_globalsTooLarge`：全局数组会消耗真实的 VM 堆。让项目状态保持紧凑。
- `failed: 32k - 1 size limit to local variables`：避免巨大的局部数组；改用小全局变量或拆分计算。
- `Registry overflow`：单个表达式里 string、text 或 point 引用太多。拆成更小的语句。
- `Internal compiler error`：可能由单行或单个字符串超过约 2046 字符引起。
