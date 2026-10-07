# SC2 native 函数参考

完整的 `Ntve` native 函数查询，按字母桶拆成一页一个，便于智能体快速检索。

| 页面 | 范围 |
|---|---|
| [triggers-native/a.md](triggers-native/a.md) | A |
| [triggers-native/b.md](triggers-native/b.md) | B |
| [triggers-native/c.md](triggers-native/c.md) | C |
| [triggers-native/d.md](triggers-native/d.md) | D |
| [triggers-native/e.md](triggers-native/e.md) | E |
| [triggers-native/f.md](triggers-native/f.md) | F |
| [triggers-native/g.md](triggers-native/g.md) | G |
| [triggers-native/h.md](triggers-native/h.md) | H |
| [triggers-native/i.md](triggers-native/i.md) | I |
| [triggers-native/k.md](triggers-native/k.md) | K |
| [triggers-native/l.md](triggers-native/l.md) | L |
| [triggers-native/m.md](triggers-native/m.md) | M |
| [triggers-native/n.md](triggers-native/n.md) | N |
| [triggers-native/o.md](triggers-native/o.md) | O |
| [triggers-native/p.md](triggers-native/p.md) | P |
| [triggers-native/q.md](triggers-native/q.md) | Q |
| [triggers-native/r.md](triggers-native/r.md) | R |
| [triggers-native/s.md](triggers-native/s.md) | S |
| [triggers-native/t.md](triggers-native/t.md) | T |
| [triggers-native/u.md](triggers-native/u.md) | U |
| [triggers-native/v.md](triggers-native/v.md) | V |
| [triggers-native/w.md](triggers-native/w.md) | W |
| [triggers-native/other.md](triggers-native/other.md) | `_other` |

## native 函数完整参考（3,196 条）

`Ntve`（`Core.SC2Mod` nativelib）里的每一个 `FunctionDef`。按名字
字母序排列；按首字母分桶，方便跳转。

各列含义：
- **名称（Name）** — Galaxy 标识符（多数情况下也是 TriggerStrings
  本地化后编辑器显示的名字）。在 XML 中这样引用：
  `<FunctionDef Type="FunctionDef" Library="Ntve" Id="<ID>"/>`。
- **ID** — 8 字符十六进制。
- **Kind** — `call`（有返回值，带 `<ReturnType>`）、`action`
  （void / 语句）、`event`（注册用 native——在 `<Event>` 引用的
  FunctionCall 元素内调用）。
- **返回（Returns）** — 返回类型，catalog 链接类型带 `<gameType>` 后缀
  （`gamelink<Unit>` 等）。动作/事件为 `—`。
- **参数（Params）** — 逗号分隔的 `name:type` 列表。类型是 Galaxy
  基础类型（`int`、`fixed`、`string`、`bool`、`unit`、`point`、
  `region`、`text` 等），catalog 引用为 `gamelink<X>`，枚举槽位为
  `preset`。无参数为 `—`。`+Nsub` 后缀表示该 native 有 N 个
  SubFunctionType 槽位（例如 IfThenElse 的 then/else、循环体、条件）——
  嵌入子 FunctionCall 之前，先在 nativelib 里查 SubFuncType ID。

提醒（来自陷阱 #11）：nativelib 里的参数默认值 **不会** 自动生效。
即使某个参数有 `<Default>` 元素，你构造的 `<FunctionCall>` 里也必须为
每个槽位显式写出 `<Parameter Type="Param" Id="…"/>`。


