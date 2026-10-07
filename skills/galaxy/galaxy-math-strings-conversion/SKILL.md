---
name: galaxy-math-strings-conversion
description: Integer and fixed-point math, trigonometry, random numbers, type conversions, string and text operations, color construction, bitwise operations, and number formatting for display in Galaxy script. Use when performing arithmetic, converting between int/fixed/string/text, building display strings, working with colors, or using NativeLib math helpers (ArithmeticIntClamp, Log, RandomPercent). Do not use for point/geometry math (use galaxy-points-regions-geometry). Start by running "python tools/sc2.py start" to read this project's previous task records.
---

# Galaxy 脚本 —— 数学、字符串与转换

## 何时使用

- 在 Galaxy 里做算术：整数除法/取余、定点精度、Min/Max、取整、对数、钳制。
- 在 `int`、`fixed`、`string`、`text`、`bool` 之间转换。
- 构造或解析显示字符串、本地化 `text`、热键与文本表达式。
- 构造颜色，或把玩家槽位转成其队伍颜色。
- 设置/清除位标志，或使用 `bitmask` 类型。

## 加载边界

- **本子技能拥有：** 整数/定点数学、三角、随机数、类型转换、字符串/text 操作、颜色、位运算、数字格式化
- **父根技能（`sc2-galaxy-scripting`）拥有：** 项目约定、schema/语法规则、真源（source-of-truth）、命名前缀
- **兄弟子技能拥有：** `galaxy-points-regions-geometry`（点/区域数学）、`galaxy-language-fundamentals`（类型系统）、`galaxy-debug-data-catalog`（调试打印/输出）

## 核心规则

1. **前置条件：** 先加载 [`sc2-galaxy-scripting`](../../sc2-galaxy-scripting/SKILL.md) —— 项目级语法约束（局部变量提升（hoisting）、无 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、真源规则、模块脚本边界与命名规范（`libMy_` 函数前缀、`libMy_g_` 全局前缀）都在那里；本子技能只拥有数学/字符串 API 参考。
2. `fixed` 是定点数，不是 IEEE 浮点：精度约 1/65536，最大值约 524287 —— 绝不要假设浮点的范围或精度。
3. 整数 `/` 向零截断；没有 `%` 运算符 —— 取余用 `ModI(a, b)`。
4. Galaxy 没有内置 Pi 常量；需要时直接写字面量（`3.14159`）。
5. 所有三角函数都以**度**为单位收发，不是弧度。
6. `RandomInt(min, max)` 与 `RandomFixed(min, max)` 两端都包含。
7. 数值、字符串、text 之间没有隐式转换 —— 显式转换（`IntToFixed`、`IntToString`、`StringToText` 等）。
8. `StringToInt`/`StringToFixed` 遇到非数字输入不安全；转换用户数据或 bank 数据前先校验。
9. `TextToString` 会丢掉格式；值保持为 `text`，直到它必须变成 `string` 的那一刻。
10. `StringFind` 无匹配时返回 `c_stringNotFound`（-1）—— 要判断它，不要拿结果当下标。
11. `StringSub` 是 1 基的，且两端都包含。
12. `Color` 分量范围是 0.0–1.0；手里是 0–255 时除以 255（没有 0–255 的构造函数）。

## 参考文档

- [references/integer-and-fixed-point.md](references/integer-and-fixed-point.md) —— 做算术、Min/Max/Abs、取整、`libNtve_gf_Log`，或 int/real 钳制辅助函数
- [references/trigonometry-and-random.md](references/trigonometry-and-random.md) —— 计算角度或取随机值时
- [references/type-conversions.md](references/type-conversions.md) —— 在 int / fixed / string / text / bool 之间搬值，或把点存成字符串时
- [references/strings-and-formatting.md](references/strings-and-formatting.md) —— 搜索、比较、切分或替换字符串；构造本地化 text；为显示做补位或取整时
- [references/bitwise-and-color.md](references/bitwise-and-color.md) —— 设置标志位，或构造/读取颜色与玩家队伍颜色时
- [references/external-sources.md](references/external-sources.md) —— 当需要逐函数参考页、NativeLib，或上游指南时
