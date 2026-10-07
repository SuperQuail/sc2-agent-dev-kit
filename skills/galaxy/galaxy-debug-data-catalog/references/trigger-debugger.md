# 触发器调试器

触发器调试器是 Galaxy 脚本最主要的运行时分析工具。

> **完整文档：** [Trigger Debugger guide](https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/053_Trigger_Debugger/)

## 启动方式

| 方式 | 做法 |
|---|---|
| 游戏内秘籍 | 测试地图时在聊天框输入 `trigdebug` |
| 编辑器设置 | File → Preferences → Test Document → Show Trigger Debugging Window |
| Galaxy 代码 | `TriggerDebugWindowOpen(lv_player, true);` |
| Galaxy 断点 | 在 Galaxy 代码里加关键字 `Breakpoint;`——执行到该行时调试器打开并跳转到那里 |

> 游戏必须处于 **Windowed（窗口化）** 模式，调试窗口才会出现。

## 推荐的初始配置

右键底部子视图 → 取消勾选所有「Show」选项，**只保留** `Show Errors` 与 `Show User Output`。这会大幅降低调试器带来的性能开销。

## 主要标签页

| 标签页 | 最适合用来 |
|---|---|
| **Variables** | 浏览全部全局变量及其运行时的当前值。按 `Total Memory Size` 排序找出内存大户。复杂类型（数组、结构体）显示「Double Click to Expand」。 |
| **Triggers** | 列出所有触发器，含触发次数、失败次数、运行次数、平均/总运行时间。按 `Total Time` 排序找后台卡顿；按 `Average Run Time` 排序找延迟尖峰。 |
| **Threads** | 所有活动的基于 `Wait` 的线程。显示每个线程当前阻塞在哪种 `Wait`（Real 还是 Game）上。右键 → View Script 跳到对应函数。 |
| **Queue** | 当前正在使用 `TriggerQueueEnter()`/`TriggerQueueExit()` 的触发器。 |
| **Trigger Profiling** | 对线程化触发器的详细分析。区分 **Self-Only Time**（纯 Galaxy 操作，不含子函数调用耗时）与 **Self+Children Time**（含子调用的总耗时）。勾上 `Show Natives` 与 `Show SubCalls` 可获得完整调用栈细节。 |
| **Function Profiling** | 按函数统计调用次数、运行次数、平均/最差/总运行时间。 |
| **Activity** | 已触发事件（绿）、已检查条件（黄）、已执行触发器（红）随时间变化的可视化曲线。适合定位延迟尖峰。 |
| **Script Code** | 完整源码视图，支持断点、局部变量查看、调用栈与 Watch 列表。 |

## 在 Script Code 标签页设置断点

1. 在 Script Code 标签页右键 → 在目标行上 **Add/Remove Breakpoint**。
2. 执行命中断点时，游戏暂停，焦点转到调试器。
3. 用各子视图的下拉菜单在 **Globals**、**Locals**、**Watch**、**Callstack**、**Breakpoints** 之间切换。
4. **Locals** 视图最有用——它显示当前暂停函数里的每个局部变量与事件参数。

## 性能分析流程（来自 Optimizing Code 指南）

1. 打开调试器运行地图。
2. 在 **Triggers Tab** 里按 `Total Time` 排序——这能看出哪个触发器造成最多累计卡顿。
3. 在 **Trigger Profiling Tab** 里勾上 `Show Natives` 与 `Show SubCalls`——这会暴露触发器内部到底哪些 native 函数最热。
4. 优先优化内层循环：热点内循环里哪怕很小的改进，也会成倍放大。

> **注意：** 单位的 Custom Values **不会**出现在 Variables 标签页里。如果你用自定义值做调试，得自己搭调试 UI。
