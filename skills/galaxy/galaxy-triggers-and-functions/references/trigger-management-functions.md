# 触发器管理函数

用于创建、执行、查询与销毁触发器的 native 函数。

| 函数 | 签名 | 用途 |
|---|---|---|
| `TriggerCreate` | `trigger (string funcName)` | 按函数名字符串创建触发器 |
| `TriggerExecute` | `void (trigger t, bool checkConds, bool runActions)` | 立即执行触发器 |
| `TriggerEnable` | `void (trigger t, bool enabled)` | 启用/停用 |
| `TriggerIsEnabled` | `bool (trigger t)` | 查询是否启用 |
| `TriggerDestroy` | `void (trigger t)` | 移除触发器 |
| `TriggerStop` | `void (trigger t)` | 停止正在执行的触发器 |
| `TriggerGetCurrent` | `trigger ()` | 当前正在运行的触发器 |
| `TriggerEvaluate` | `bool (trigger t)` | 只跑条件检查 |
| `TriggerGetExecCount` | `int (trigger t)` | 已触发次数 |
| `TriggerActiveCount` | `int ()` | 当前活动触发器数量 |
