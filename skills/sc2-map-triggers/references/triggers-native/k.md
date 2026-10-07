# SC2 Native 函数：K

各列含义见 [../triggers-native-functions.md](../triggers-native-functions.md)。在 XML 中这样引用：`<FunctionDef Type="FunctionDef" Library="Ntve" Id="<ID>"/>`。

`nativelib` 里的参数默认值不会自动生效；生成的 `<FunctionCall>` XML 中每个槽位都要显式写出 `<Parameter Type="Param" Id="..."/>`。

### K

<a id="k"></a>

| 名称 | ID | Kind | 返回 | 参数 |
|---|---|---|---|---|
| `KickFromGame` | `B06831BB` | action | `—` | playerGroup:playergroup |
| `KillDoodadsInRegion` | `9C8E59A4` | action | `—` | target:region, doodadType:gamelink<Actor> |
| `KillingPlayer` | `CAD5046C` | call | `int` | — |
| `KillingUnit` | `ADB84019` | call | `unit` | — |
| `KillModel` | `04454841` | action | `—` | model:actor |
