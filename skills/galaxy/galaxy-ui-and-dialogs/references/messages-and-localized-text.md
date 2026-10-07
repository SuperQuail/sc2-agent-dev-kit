# HUD 消息与本地化文本

`UIDisplayMessage` 的区域、带声音的错误消息，以及本地化字符串的读取/替换/着色。

## 消息与反馈

### 在 HUD 区域显示消息

```galaxy
UIDisplayMessage(PlayerGroupAll(), c_messageAreaChat,     StringToText("GG!"));
UIDisplayMessage(PlayerGroupAll(), c_messageAreaSubtitle, StringToText("Round 1 – Fight!"));
UIDisplayMessage(
    PlayerGroupSingle(lv_player),
    c_messageAreaChat,
    StringToText("Not enough minerals!")
);

// Area constants
c_messageAreaChat         // bottom-left chat area
c_messageAreaSubtitle     // center subtitle
c_messageAreaDefault
c_messageAreaObjective
c_messageAreaWarning
```

### 带声音的错误消息

```galaxy
libNtve_gf_UIErrorMessage(
    PlayerGroupSingle(lv_player),
    StringToText("Cannot afford this upgrade!"),
    SoundLink("UI_GenericError", -1)
);
```

## 本地化文本

```galaxy
// Read from GameStrings.txt / Trig/ namespace
text lv_msg = StringExternal("Trig/Bosskilled");

// With variable replacements (SSF pattern)
text lv_result = Utility_TextExpressionReplacement3(
    "Trig/Bosskilled",
    IntToText(gv_Part_AmountObjectivesDefeated),
    IntToText(gv_Part_AmountObjectivesMax),
    IntToText(gv_Difficulty_Points)
);

// Colorize player name
color playerColor = libNtve_gf_ConvertPlayerColorToColor(PlayerGetColorIndex(playerID, false));
text colored = TextWithColor(StringToText(PlayerName(playerID)), playerColor);

// Combine text
text lv_msg2 = StringToText("+") + FixedToText(amount, 2);
```
