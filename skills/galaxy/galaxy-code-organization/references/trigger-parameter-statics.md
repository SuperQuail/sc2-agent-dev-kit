# 通过 `static` 给触发器处理函数传参

触发器处理函数不接参数；这是在跨 `Wait()` 时安全把数据交给它们的文件作用域寄存器模式。

## 用 static 变量传递触发器参数

触发器需要把数据传给处理函数时（触发器函数不接参数），用 `static` 文件作用域变量当参数寄存器：

```galaxy
// In Utilities.galaxy
static int Utility_DelayedTextTagDestroyer_ParamTextTag;
static trigger Utility_DelayedTextTagDestroyer_Trigger;

void Utility_DelayedTextTagCreate(text inText, color inColor, point position, playergroup pg, fixed offset) {
    Utility_DelayedTextTagDestroyer_ParamTextTag = TextTagCreate(...);
    TriggerExecute(Utility_DelayedTextTagDestroyer_Trigger, false, false);
}

bool Utility_DelayedTextTagDestroyer(bool testCond, bool runActions) {
    int textTag = Utility_DelayedTextTagDestroyer_ParamTextTag;  // capture immediately
    Wait(3.5, c_timeGame);
    TextTagDestroy(textTag);
    return true;
}
```

在任何 `Wait()` 调用之前，先把 static 捕获进处理函数顶部的局部变量。
