# 问题受理规则与证据

- **数值改动：** 编辑 XML 数值前，记录本地 catalog 条目、父级、继承值与打算做的覆盖。
- **UI 与文本修复：** 改 `GameStrings.txt` 或 `ObjectStrings.txt` 之前，先确定确切的 UI 面（世界悬停、选择面板、命令卡按钮、提示框或编辑器文本）及其生效的本地化锚点。
- **运行时日志：** playtest 之后跑 `python tools/extract-playtest-bugreport.py`，把 `Alerts.txt` 与 `ScriptError.txt` 提取到 `bugreport.txt`。

## 证据纪律

- 只用与该阶段匹配的证据推进阶段；绝不跳闸门。
- 静态校验成功只是 `static validation passed`——它不是编辑器接受，也不是运行时验证。
- 编辑器接受需要一次真正的打开/保存往返，且没有 schema、依赖或触发器编译失败。
- 打包后运行时接受需要游戏内复现或验证。
