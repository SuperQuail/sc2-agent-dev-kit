# 外部编辑纪律与重新生成安全

## 大型触发器文件的搜索规则

地图 `Triggers` 文件是庞大的原版树。**不要通读它们。** 用带 `--max-count` 的有界 `rg` 搜特定词（例如 `BankPreLoad`、`BankSave`、`SetCurrentMap`）。

## 编辑纪律

- 外部编辑前，先在编辑器里关闭对应文档并备份——内存中的副本会覆盖外部改动。
- 绝不直接编辑 `MapScript.galaxy`；它会在保存时重新生成。
- 在外部生成可用的触发器 XML，再让 SC2 编辑器打开它。先搭好原生 FunctionCall 树。
- 内嵌 `customscriptaction` 是 GUI 无法合理表达的逻辑的窄口兜底；它绕过类型检查器，所以要小，并核验编辑器编译。
- 外部 XML 改动之后，如果行为异常，检查生成的 `MapScript.galaxy`。

## 参数与 ID 纪律

- 参数必须显式给出——不要假设原生默认值会补齐。`ValueType` 与 gamelink 的 `ValueGameType` 必须准确。
- 原生参数 ID、Preset ID 与 `SubFunctionType` ID 必须从 native 表里查：`Or` 与 `And` 用不同的条件子类型。子动作是调用树的子节点，不是普通 `<Parameter>` 里的槽位。
- 一棵 `<FunctionCall>` 子树不得在多个父引用之间复用同一个 ID——复制成新 ID 并清掉孤儿引用。

## 数组与变量布局

- `ArraySize` 嵌在 `VariableType` 里。源文档指出 GUI 的 `Value=N` 表示索引 `0..N`，与 Galaxy 直接的数组长度写法不同——要区别对待。
- 每个已声明的元素（尤其是变量与 `ParamDef`）都要在 `TriggerStrings.txt` 里加匹配的名字。`TriggerStrings.txt` 是纯文本——**不要** 做 HTML 转义。`Triggers` XML 走标准 XML 转义规则。

## 编辑器保存与重新生成安全

- 来源记录：有事件但没动作的触发器，或空 `<Comment>` 元素，会导致模组编辑器保存失败。事件触发器一律给一个有效动作；`<Comment>` 内容要非空并带名字。
- 保存之后，重新打开地图并强制编辑一次触发器让编辑器重新生成，然后核验 `InitTriggers` 与 `TriggerAddEvent*` 调用。在触发器树里可见并不证明事件已注册。
