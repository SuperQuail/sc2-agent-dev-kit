# SC2 Native 函数：L

各列含义见 [../triggers-native-functions.md](../triggers-native-functions.md)。在 XML 中这样引用：`<FunctionDef Type="FunctionDef" Library="Ntve" Id="<ID>"/>`。

`nativelib` 里的参数默认值不会自动生效；生成的 `<FunctionCall>` XML 中每个槽位都要显式写出 `<Parameter Type="Param" Id="..."/>`。

### L

<a id="l"></a>

| 名称 | ID | Kind | 返回 | 参数 |
|---|---|---|---|---|
| `LastCreatedActor` | `AF651D06` | call | `actor` | — |
| `LastCreatedActorScope` | `423CB348` | call | `actorscope` | — |
| `LastReplacedUnit` | `10EBC0A6` | call | `unit` | — |
| `ListAdd` | `6CBE8D94` | call | `actormsg` | actorRefName:string |
| `ListRemove` | `1B47B527` | call | `actormsg` | actorRefName:string |
| `LoadUserDataValueIntoVariable` | `B70F7D91` | action | `—` | userType:gamelink<User>, instance:userinstance, field:userfield, index:int, var:anyvariable, userDataType:preset |
| `LoadVariableValueIntoUserData` | `39051196` | action | `—` | userType:gamelink<User>, instance:userinstance, field:userfield, index:int, var:anyvariable, userDataType:preset |
| `Log` | `16259F45` | call | `fixed` | x:fixed, x2:fixed |
| `Log2` | `3692F869` | call | `fixed` | x:fixed |
| `Log2I` | `B12493DA` | call | `int` | x:fixed |
| `LookAtTargetFromPointWithZOffset` | `B2D4AC17` | call | `actor` | point:point, z:fixed |
| `LookAtTargetFromUnitAttachPoint` | `7DBB5796` | call | `actor` | unit:unit, attachPoint:preset |
