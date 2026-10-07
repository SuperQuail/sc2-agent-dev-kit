# 触发器 XML 结构与元素规则

触发器以 XML 存放在 `<Map>/Triggers`（地图）或包在 `<Library Id="XXXXXXXX">` 里的 `<Mod>/Triggers`（模组），显示名另放 `<Map>/enUS.SC2Data/LocalizedData/TriggerStrings.txt`。

每个元素都有 8 字符十六进制 Id。引用用 `Type+Library+Id`；地图本地引用省略 `Library`，跨库引用要带上（例如原生自带函数用 `Library="Ntve"`）。

## 元素结构规则

- `<FunctionDef>` 的主体 = 按顺序排列的一串 `<FunctionCall>` 子元素（有序语句）。**不是** 直接写 `<ScriptCode>`（那种形态只用于带 `<FlagSubFunctions/>` 的子函数模板）。
- `<FlagCall/>` = 返回值（需要 `<ReturnType>`）。`<FlagAction/>` = 动作，返回 void。`<FlagEvent/>` = 事件注册。
- 局部变量：在 FunctionDef 内声明一个 `<Variable Type="Variable" Id="..."/>` 引用，另加一个独立的顶层 `<Element Type="Variable">`，带类型与初始值。

## 参数传递

一个 `<Param>` 元素可以包含以下之一：

- 内联字面量：`<Value>X</Value>` + `<ValueType Type="int"/>`（或 `string`、`fixed`、`bool`）
- 游戏链接：`<Value>Marine</Value>` + `<ValueType Type="gamelink"/>` + `<ValueGameType Type="Unit"/>`
- 子调用：`<FunctionCall Type="FunctionCall" Id="..."/>`
- 变量：`<Variable Type="Variable" Id="..."/>`（跨库要加 `Library=`）
- preset 枚举：`<Preset Type="PresetValue" Library="Ntve" Id="..."/>`
- 父函数参数：`<Parameter Type="ParamDef" Id="..."/>`

## SubFunctionType 模式（容易踩坑）

有些 native 接受子动作（then 分支、循环体、条件）。子 FunctionCall 通过 `<SubFunctionType Library="Ntve" Id="..."/>` 声明自己的角色。子动作住在父级的子元素列表里，**不是** `<Parameter>` 槽位里。

关键 ID：

- `IfThenElse`（Ntve `00000137`）：if=`00000003`，then=`00000004`，else=`00000005`
- `PickEachUnitInGroup`（Ntve `C4DC760C`）：body=`9441B8B5`；取被选中的单位用 `UnitGroupLoopCurrent`（Ntve `19CE733E`）
- `And`（Ntve `00000132`）：条件子类型 `00000002`
- `Or`（Ntve `00000133`）：条件子类型 `00000001` —— **与 And 不同！** 混用会在保存时静默剥离 Comparison。

## 元素标志位

- `Native`（`0x02`）：引擎原生绑定
- `FuncAction`（`0x04`）：动作（void 返回）
- `FuncCall`（`0x08`）：有返回值的函数
- `Event`（`0x10`）：事件注册
- `CustomScript`（`0x100`）：原始 Galaxy 块
- `Hidden` / `Internal` / `Restricted`：编辑器可见性

## 触发器参数类型 → Galaxy 映射

| 触发器类型 | Galaxy 基础类型 | 示例 |
|---|---|---|
| `gamelink`、`anygamelink` | `string` | 单位类型、技能、武器 |
| `catalogentry`、`catalogfieldpath`、`reference` | `string` | 动态 catalog 反射 |
| `preset` | `int` | 枚举值（`c_unitPropLife`） |
| `int`、`fixed`、`bool` | `int`、`fixed`、`bool` | 标准字面量 |
| `point`、`region`、`unit`、`unitgroup` | 引擎句柄 | 空间/游戏性实体 |
