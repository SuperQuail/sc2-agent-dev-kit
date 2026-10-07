# 异步执行与多线程

覆盖 *Multithreading / Async Execution* 一节，以及 SSF 的静态参数包装模式。

## 包装需要异步执行的函数（SSF 模式）

函数需要 `Wait()` 或长时间运行逻辑时，把它包进触发器。用 `static` 文件作用域变量当参数寄存器，并立即捕获到局部变量：

```galaxy
// Declare param register at file scope
static int Utility_DelayedTextTagDestroyer_ParamTextTag;
static trigger Utility_DelayedTextTagDestroyer_Trigger;

void Utility_DelayedTextTagCreate(text inText, color inColor, point position, playergroup pg, fixed offset) {
    // Store param in static, then kick the trigger
    Utility_DelayedTextTagDestroyer_ParamTextTag = TextTagCreate(TextWithColor(inText, inColor), 24, position, offset, true, false, pg);
    TextTagSetVelocity(Utility_DelayedTextTagDestroyer_ParamTextTag, 1.0, 90.0);
    TriggerExecute(Utility_DelayedTextTagDestroyer_Trigger, false, false);
}

bool Utility_DelayedTextTagDestroyer(bool testCond, bool runActions) {
    // Capture static into local BEFORE any Wait() — the static may be overwritten
    int textTag = Utility_DelayedTextTagDestroyer_ParamTextTag;
    Wait(3.5, c_timeGame);
    TextTagDestroy(textTag);
    return true;
}

// Init: register the trigger once
void Utilities_Init() {
    Utility_DelayedTextTagDestroyer_Trigger = TriggerCreate("Utility_DelayedTextTagDestroyer");
}
```

**关键规则：** 一定要在处理函数体的最顶部、任何 `Wait()` 之前，把静态变量拷贝进局部变量。否则并发调用会在你的线程读取之前覆盖它。

## Galaxy 的「多线程」如何工作（时间片）

Galaxy **没有**真正的并行执行。编辑器用时间片实现**协作式多线程**：

1. 线程化的触发器 / action 定义正常执行，直到遇到 `Wait()`。
2. 在 `Wait()` 处控制权交回父线程（或下一个待处理事件）。
3. 等待结束后，控制权回到被挂起的线程。

它靠在 `Wait()` 边界上快速切换线性控制流来「伪造」并行。它**不比**顺序代码快——实际上因为要管理线程状态，线程化代码更慢。

> **来源：** [Multithreading With Action Definitions](https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/057_Multithreading_with_Action_Definitions/)

## TriggerExecute——在独立线程中触发

```galaxy
// Run trigger NOW in the SAME thread (blocks until done)
TriggerExecute(myTrigger, false, false);

// Run trigger in its OWN thread (returns immediately; trigger runs concurrently)
TriggerExecute(myTrigger, false, true);   // third arg = separate thread
```

静态参数模式（见上文异步一节）是向基于触发器的异步函数传参的正确做法。

## 线程化 action 定义的自动生成实现

GUI 编辑器创建线程化 action 定义时，会生成：

```galaxy
// 1. Global parameter registers (one per action-def parameter)
int auto_gf_MyActionDef_lp_param;

// 2. Global trigger variable (created once, reused)
trigger auto_gf_MyActionDef_Trigger = null;

// 3. The trigger function body — captures params into locals BEFORE any Wait
bool auto_gf_MyActionDef_TriggerFunc(bool testConds, bool runActions) {
    int lp_param = auto_gf_MyActionDef_lp_param; // capture to local immediately
    Wait(5.0, c_timeGame);
    // use lp_param here safely
    return true;
}

// 4. Wrapper called at each invocation — copies args into global registers, fires trigger
void gf_MyActionDef(int lp_param) {
    auto_gf_MyActionDef_lp_param = lp_param;       // write to global register
    if (auto_gf_MyActionDef_Trigger == null) {
        auto_gf_MyActionDef_Trigger = TriggerCreate("auto_gf_MyActionDef_TriggerFunc");
    }
    TriggerExecute(auto_gf_MyActionDef_Trigger, false, true); // run in new thread
}
```

**关键规则：** 在线程函数的**第一行**、任何 `Wait()` 之前，把全局参数寄存器捕获进局部变量。否则新的一次调用可能在线程读取之前覆盖该全局变量。

## 性能提示

- 多线程 action 定义明显慢于非线程版本。
- 只有在确实需要基于 `Wait()` 的并发时间线时才用线程（例如 5 个陆战队员各自 5 秒生命周期并行推进）。
- 一次性延时通常用基于 `Timer` 的触发器更简单也更省。
