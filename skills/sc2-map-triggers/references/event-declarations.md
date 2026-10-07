# 事件声明（最常见的静默失败）

事件 **不是** `<Element Type="Event">`。它们是 `<Element Type="FunctionCall" Id="X">`（与动作同形），Trigger 通过 `<Event Type="FunctionCall" Id="X"/>` 引用它们。编辑器会 **静默剥离** 任何 `<Event Type="Event"/>` 引用。

**布局要求：** 事件 `<Element Type="FunctionCall">` 必须紧跟在它所属 Trigger 的收尾 `</Element>` 之后（与兄弟元素相邻）。放得太远，`_Init` 代码生成会静默 **丢掉** `TriggerAddEvent*(...)` 调用——触发器注册了却永不触发。读生成的 `MapScript.galaxy` 来核验。

```xml
<Element Type="Trigger" Id="D23DC42E">
    <Event Type="FunctionCall" Id="859460FA"/>
    <Action Type="FunctionCall" Id="47FB5D16"/>
</Element>
<Element Type="FunctionCall" Id="859460FA">      <!-- event, immediately after -->
    <FunctionDef Type="FunctionDef" Library="Ntve" Id="6D565EB4"/>
    <Parameter Type="Param" Id="DurParam"/>
    <Parameter Type="Param" Id="TimeTypeParam"/>
</Element>
```

条件遵循同样的模式：`<Condition Type="FunctionCall" Id="X"/>`。
