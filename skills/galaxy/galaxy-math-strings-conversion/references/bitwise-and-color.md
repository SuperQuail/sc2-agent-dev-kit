# 位运算与颜色

位标志与 `bitmask` 类型，以及颜色构造、分量读取与玩家颜色转换。

## 位运算

```galaxy
int lv_flags = 0;
lv_flags = lv_flags | (1 << 3);   // set bit 3
lv_flags = lv_flags & ~(1 << 3);  // clear bit 3
bool lv_set = ((lv_flags & (1 << 3)) != 0);

// BitMask type (for unit filters etc.)
bitmask lv_mask = BitMaskMakeDefaultMask();
BitMaskSetIndex(lv_mask, 5, true);
bool lv_bit = BitMaskTrueIndex(lv_mask, 5);
```

---

## 颜色

```galaxy
// Define colors (components 0.0–1.0)
color lv_red   = Color(1.0, 0.0, 0.0);
color lv_white = ColorWithAlpha(1.0, 1.0, 1.0, 1.0);  // r, g, b, a

// Get a component
fixed lv_r = ColorGetComponent(lv_red, c_colorComponentRed);

// Player team color — convert player slot to color
color lv_pc = libNtve_gf_ConvertPlayerColorToColor(lv_player);
// (int playerSlot) — returns the player's team color as a color value

// From index
color lv_ci = ColorFromIndex(3, c_teamColorType);

// Convert 0-255 integer to 0.0-1.0
fixed lv_norm = Color255FromFixed(128) / 255.0;

// NativeLib color/string helpers:
string lv_colorStr = libNtve_gf_ConvertColorToString(lv_red);
// Returns the color as a string (e.g. for debug or bank storage)
```
