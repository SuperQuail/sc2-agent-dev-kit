# 向导字符串求值

求值字符串中使用的 token 语法、目录（catalog）引用、算术与索引。

## 字符串求值

求值字符串使用 `^tokens^`：

- `^InputId^`：输入值。
- `^MacroId^`：宏值。
- `ENTRYINDEX`、`VALUEINDEX`、`ITEMINDEX`：索引。
- 目录引用：`ref=TYPE,ENTRYID,FIELDPATH`；末尾加 `#` 表示取数量。
- 算术：`^Value1^ + ^Value2^`。

示例：`<value>^BaseHP^ * 2 + 10</value>`
