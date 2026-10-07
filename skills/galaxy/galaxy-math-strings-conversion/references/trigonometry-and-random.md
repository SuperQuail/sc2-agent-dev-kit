# 三角与随机数

基于角度的三角（用度，不是弧度）与随机数辅助函数。

## 三角

Galaxy 里角度是**度**（不是弧度）。

```galaxy
fixed lv_sin = Sin(45.0);
fixed lv_cos = Cos(90.0);
fixed lv_tan = Tan(45.0);
fixed lv_asin = ASin(0.5);   // returns degrees
fixed lv_acos = ACos(0.5);
fixed lv_atan = ATan(1.0);
fixed lv_atan2 = ATan2(lv_dy, lv_dx);   // angle from delta components
```

---

## 随机数

```galaxy
int   lv_ri    = RandomInt(1, 10);             // [min, max] inclusive
fixed lv_rf    = RandomFixed(0.0, 1.0);        // [min, max]

// NativeLib random helpers:
fixed lv_pct   = libNtve_gf_RandomPercent();  // random fixed 0.0–100.0
fixed lv_angle = libNtve_gf_RandomAngle();    // random fixed 0.0–360.0
```
