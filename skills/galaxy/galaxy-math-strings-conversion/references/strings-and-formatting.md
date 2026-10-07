# 字符串、Text 与数字格式化

字符串比较、搜索与替换；富 `text` 处理与本地化；构造显示数字。

## 字符串

```galaxy
// Concatenation (+ operator)
string lv_full = "Hello" + ", " + PlayerName(lv_player) + "!";

// Length
int lv_len = StringLength(lv_str);

// Equality
bool lv_eq = StringEqual(lv_a, lv_b, c_stringNoCase);  // or c_stringCase

// Compare (lexicographic)
int lv_cmp = StringCompare(lv_a, lv_b, c_stringNoCase);  // -1, 0, 1

// Substring tests
bool lv_has = StringContains(lv_str, "zerg", c_stringAnywhere, c_stringNoCase);
int  lv_pos = StringFind(lv_str, "zerg", c_stringAnywhere, c_stringNoCase);
// returns c_stringNotFound (-1) if not found

// Extract
string lv_sub  = StringSub(lv_str, 1, 4);     // chars 1..4 (1-based)
string lv_word = StringWord(lv_str, 2);        // 2nd whitespace-delimited word

// Replace
string lv_rep  = StringReplace(lv_str, "old", "new", c_stringReplaceAll);
string lv_rep2 = StringReplaceWord(lv_str, "colour", "color");

// Containment location constants
c_stringAnywhere
c_stringBeginsWith
c_stringEndsWith

// Case constants
c_stringCase      // case-sensitive
c_stringNoCase    // case-insensitive
```

---

## Text（本地化、富文本）

```galaxy
// Build text
text lv_t = StringToText("plain string");

// Concatenate text
text lv_combined = lv_t1 + StringToText(" ") + lv_t2;

// Colorize text
text lv_colored = TextWithColor(lv_t, ColorWithAlpha(1.0, 0.4, 0.4, 1.0));

// Localized string from GameStrings.txt
text lv_loc = StringExternal("Param/Value/libXXXXXXXX_MyKey"); // replace prefix with your mod's prefix

// Text replace
text lv_tr = TextReplaceWord(lv_t, "old", StringToText("new"));

// Case conversion — text type
text lv_upper = TextCase(lv_t, true);   // UPPERCASE
text lv_lower = TextCase(lv_t, false);  // lowercase

// Case conversion — string type
string lv_up = StringCase(lv_s, true);  // UPPERCASE
string lv_lo = StringCase(lv_s, false); // lowercase

// Localized hotkey or asset path (companion to StringExternal)
text lv_hk  = StringExternalHotkey("Param/Hotkey/MyAbility"); // e.g. "Q"
string lv_asset = StringExternalAsset("Param/Asset/MyIcon");  // asset path string

// Text expression tokens (build parameterized display strings)
TextExpressionSetToken("Param/Expression/MyExpr", "UNIT", TextCase(UnitTypeGetName(UnitGetType(lv_unit)), true));
text lv_result = TextExpressionAssemble("Param/Expression/MyExpr");
```

---

## 为显示格式化数字

```galaxy
// Fixed with N decimal places
text lv_pct = FixedToTextAdvanced(66.667, 1, false, false);  // "66.7"

// Integer to padded string
string lv_padded = "00" + IntToString(lv_n);  // manual zero-pad
lv_padded = StringSub(lv_padded, StringLength(lv_padded) - 1, StringLength(lv_padded));

// Combine for display (e.g. kill counter)
text lv_display = IntToText(lv_kills) + StringToText(" kills");
```
