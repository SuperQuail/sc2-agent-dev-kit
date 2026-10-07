# 字符串与颜色 —— 语言层速查

读普通 Galaxy 代码时会遇到的少数几个字符串/颜色调用。完整数学/字符串 API 见兄弟技能 `galaxy-math-strings-conversion`。

## 字符串操作

```galaxy
StringContains(haystack, needle, c_stringAnywhere, c_stringNoCase)  // → bool
StringEqual(a, b, c_stringNoCase)                                   // → bool
IntToString(lv_count)                                               // → string
FixedToText(lv_fixed, c_fixedPrecisionAny)                          // → text
TextToString(lv_text)                                               // → string
StringExternal("Param/Value/lib_5A1C9904_KeyName")                  // → text (localized)
```

拼接用 `+`：

```galaxy
string lv_key = lp_key + IntToString(lv_index);
text lv_msg = lv__KilledTextBuilder + StringExternal("Param/Value/lib_5A1C9904_956C1A6F") + lv__KillerTextBuilder;
```

---

## 颜色

```galaxy
color lv_allyColor  = Color(28*100/255, 167*100/255, 234*100/255);
color lv_enemyColor = Color(100.00, 0.00, 0.00);
color lv_transparent = ColorWithAlpha(0, 0, 0, 0);
color lv_playerColor = libNtve_gf_ConvertPlayerColorToColor(PlayerGetColorIndex(lp_player, false));
```
