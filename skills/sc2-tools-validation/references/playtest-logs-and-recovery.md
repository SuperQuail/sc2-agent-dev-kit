# Playtest 日志、问题生命周期与工具故障恢复

## Playtest bug 提取

`extract-playtest-bugreport.py`（以及 `.ps1`）把星际争霸 II `GameLogs` 里的告警与脚本错误提取成干净报告（`bugreport.txt`）。playtest 之后跑它以分诊运行时问题。用之前先核验它的路径配置——不要假设旧机器的 `GameLogs` 路径仍然适用。

## 工具故障恢复

如果某个补丁辅助脚本或工具报告 `os error 206` 或字符串替换错误：

- 用精确文本替换
- 保留编码/换行
- 跑受影响的作用域或回归来核验

## 问题生命周期与反馈

- 项目问题生命周期：`reported → root cause confirmed → source fixed → static validation passed → Editor accepted → packaged runtime passed`。只用有效值证据推进阶段。
- 静态层：XML/schema、Galaxy 规则、ID 与命令卡引用、重复定义、本地化、文档链接。
- 编辑器层：带正确依赖打开模组/地图，审阅告警，另存为 Components，审阅规范化 diff。**不要** 在项目模组作为外部 override 激活时打开原版地图。
- 游戏层：最小场景测试——生产与实际扣费、命令卡、攻击目标、技能、变形、模型/音频、每一级升级与解锁。
- 把反馈记到 `sc2agent-records/<project>/issues.md`，至少包含：地图、玩家/阵营、复现步骤、预期/实际，以及完整告警。
