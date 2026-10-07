# 类型转换

在 int / fixed / string / text / bool 之间转换，以及 NativeLib 的 bool-点-字符串辅助函数。

## 类型转换

```galaxy
// int ↔ fixed ↔ string ↔ text
fixed lv_f  = IntToFixed(5);
int   lv_i  = FixedToInt(3.7);           // truncates towards zero
int   lv_i2 = FloorI(FixedToInt(3.7));   // same here
string lv_s = IntToString(42);
string lv_sf = FixedToString(3.14);
text   lv_t  = IntToText(42);
text   lv_tf = FixedToText(3.14, c_fixedPrecisionAny);
text   lv_tfa = FixedToTextAdvanced(3.14159, 2, true, false); // 2 decimals
int    lv_si = StringToInt("99");
fixed  lv_sf2 = StringToFixed("3.14");
text   lv_ts  = StringToText("hello");
string lv_st  = TextToString(lv_t);       // loses formatting

// bool conversions
int  lv_bi = BoolToInt(true);            // 1 or 0
bool lv_ib = (lv_bi != 0);

// NativeLib bool/string/point conversion helpers:
text   lv_bt  = libNtve_gf_ConvertBooleanToText(true);    // returns text "True"/"False"
string lv_bs  = libNtve_gf_ConvertBooleanToString(true);  // returns string "true"/"false"
bool   lv_sb  = libNtve_gf_ConvertStringToBoolean("true"); // string → bool

// Point ↔ string (useful for bank storage of positions)
string lv_ps  = libNtve_gf_ConvertPointToString(lv_point);   // "(x,y)"
point  lv_sp  = libNtve_gf_ConvertStringToPoint("(16.0,32.0)");
```
