# 整数与定点数学

`int` 与 `fixed` 上的算术、Min/Max/Abs、取整、对数，以及 NativeLib 的钳制辅助函数。

## 整数数学

```galaxy
int lv_a = 10;
int lv_b = 3;

int lv_sum  = lv_a + lv_b;
int lv_diff = lv_a - lv_b;
int lv_mul  = lv_a * lv_b;
int lv_div  = lv_a / lv_b;      // integer division — truncates
int lv_mod  = ModI(lv_a, lv_b); // remainder (no % operator)
int lv_min  = MinI(lv_a, lv_b);
int lv_max  = MaxI(lv_a, lv_b);
int lv_abs  = AbsI(lv_a);
```

---

## 定点数学

Galaxy 用 `fixed` 而不是 `float`。精度通常是 1/65536。

```galaxy
fixed lv_x = 3.5;
fixed lv_y = 1.25;

fixed lv_sum  = lv_x + lv_y;
fixed lv_mul  = lv_x * lv_y;
fixed lv_min  = MinF(lv_x, lv_y);
fixed lv_max  = MaxF(lv_x, lv_y);
fixed lv_abs  = Abs(lv_x);
fixed lv_sqrt = Sqrt(lv_x);
fixed lv_pow  = Pow(lv_x, 2.0);
fixed lv_log2 = Log2(lv_x);
fixed lv_fl   = Floor(lv_x);
fixed lv_ceil = Ceiling(lv_x);
fixed lv_pi   = 3.14159;                 // no built-in Pi constant

// NativeLib math helpers (from TriggerLibs/NativeLib.galaxy):
// libNtve_gf_Log(x, base) — computes log base `base` of `x`
fixed lv_log10 = libNtve_gf_Log(lv_x, 10.0);   // log base 10
fixed lv_ln    = libNtve_gf_Log(lv_x, 2.71828); // natural log (approx)

// Clamp helpers:
int   lv_ci = libNtve_gf_ArithmeticIntClamp(lv_value, 0, 100);   // clamps int to [0,100]
fixed lv_cf = libNtve_gf_ArithmeticRealClamp(lv_value, 0.0, 1.0); // clamps fixed to [0,1]
```
