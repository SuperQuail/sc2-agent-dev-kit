# 函数与作用域

覆盖父技能的 *Functions* 与 *Scope / Visibility* 两节。

## 函数形态

```galaxy
// Basic function
int Plus(int i, int j) {
    int result = i + j;   // locals declared at TOP of function, before any logic
    return result;
}

// void function (no return value)
void DoSomething(string lp_name, int lp_count) {
    // implementation
}

// File-private function (cannot be called from other files)
static bool IsInternal() {
    return true;
}

// Native function declaration (maps to engine internals)
native int AIGetRawGasNumSpots(int player, int town);

// Custom typedef / function pointer type
void MyCallback_t();
typedef funcref<MyCallback_t> MyCallbackRef;
```

## 规则

- 所有局部变量**必须**声明在函数最顶部，位于任何语句或函数调用之前。
- `var++` 非法，要写 `var += 1`。
- `static` 让函数变成文件私有。
- 单行不得超过 2048 字符。
- 没有 `/* */` 块注释——只有 `//` 行注释。

## 作用域 / 可见性

```galaxy
// File-private (not callable from outside)
static bool HelperFunc() { return true; }

// Public (callable from any file that is compiled together)
bool PublicFunc() { return HelperFunc(); }
```

> 函数可以先声明后定义（前向声明）。
> 只要所有文件都被 include 进 `MapScript.galaxy`，调用任意文件里的函数都不需要在本文件写 `include`。
