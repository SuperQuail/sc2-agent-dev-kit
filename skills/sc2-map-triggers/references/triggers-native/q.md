# SC2 Native 函数：Q

各列含义见 [../triggers-native-functions.md](../triggers-native-functions.md)。在 XML 中这样引用：`<FunctionDef Type="FunctionDef" Library="Ntve" Id="<ID>"/>`。

`nativelib` 里的参数默认值不会自动生效；生成的 `<FunctionCall>` XML 中每个槽位都要显式写出 `<Parameter Type="Param" Id="..."/>`。

### Q

<a id="q"></a>

| 名称 | ID | Kind | 返回 | 参数 |
|---|---|---|---|---|
| `QueryPersistent` | `06FA2BA0` | call | `actormsg` | enterResponseActor:gamelink<Actor>, leaveResponeActor:gamelink<Actor> |
| `QueryRadius` | `B26FF700` | call | `actormsg` | radius:fixed, responseActor:gamelink<Actor> |
| `QueryRegion` | `8782BEB2` | call | `actormsg` | regionActor:gamelink<Actor>, responseActor:gamelink<Actor> |
| `QueuedBehaviorTypeInTrainingQueueSlot` | `51E35D58` | call | `gamelink<Behavior>` | unit:unit, slot:int, item:int |
| `QueuedUnitTypeInTrainingQueueSlot` | `36426157` | call | `gamelink<Unit>` | unit:unit, slot:int, item:int |
| `QueuedUpgradeTypeInTrainingQueueSlot` | `B97B1F15` | call | `gamelink<Upgrade>` | unit:unit, slot:int, item:int |
