# SC2 Native 函数：I

各列含义见 [../triggers-native-functions.md](../triggers-native-functions.md)。在 XML 中这样引用：`<FunctionDef Type="FunctionDef" Library="Ntve" Id="<ID>"/>`。

`nativelib` 里的参数默认值不会自动生效；生成的 `<FunctionCall>` XML 中每个槽位都要显式写出 `<Parameter Type="Param" Id="..."/>`。

### I

<a id="i"></a>

| 名称 | ID | Kind | 返回 | 参数 |
|---|---|---|---|---|
| `IfThen` | `BB1891ED` | action | `—` | — +2sub |
| `IfThenElse` | `00000137` | action | `—` | — +3sub |
| `IfThenMultiple` | `C0083258` | action | `—` | — +1sub |
| `ImageToString` | `00FEA644` | call | `string` | val:filepath |
| `IncrementInteger` | `A0F31305` | action | `—` | var:anyvariable, operator:preset, val:int |
| `IncrementReal` | `DB01ECDC` | action | `—` | var:anyvariable, operator:preset, val:fixed |
| `InitialDateTimeGet` | `CF3C0572` | call | `datetime` | — |
| `InShrub` | `2489EDDB` | call | `bool` | point:point |
| `IntersectionOfPlayerGroups` | `0DE37AD9` | call | `playergroup` | groupA:playergroup, groupB:playergroup |
| `IntLoopCurrent` | `D02F7ACF` | call | `int` | — |
| `IntLoopCurrentDeprecated` | `A66EFCA4` | call | `int` | — |
| `IntToDateTime` | `406A7727` | call | `datetime` | uNIXEpoch:int |
| `IntToFixed` | `00000001` | call | `fixed` | val:int |
| `IntToFixed` | `3EA2DE0F` | call | `int` | val:preset |
| `IntToString` | `00000002` | call | `string` | val:int |
| `IntToText` | `EAC465A1` | call | `text` | val:int |
