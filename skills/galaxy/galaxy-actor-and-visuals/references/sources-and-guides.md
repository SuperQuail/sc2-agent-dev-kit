# 来源与指南

本子技能的上游代码库、NativeLib actor 辅助函数，以及 actor 事件与术语的 wiki 页面。

## 前置

先加载 [`sc2-galaxy-scripting`](../../../sc2-galaxy-scripting/SKILL.md)，获取项目级 Galaxy 语法约束（局部变量提升（hoisting）、不支持 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、真源规则、模块化脚本边界与命名约定（例如 `libMy_` 函数前缀、`libMy_g_` 全局前缀）。本子技能只负责深度 API 参考；项目约定在路由技能里。

## 关键参考

| 资源 | URL |
|---|---|
| 原生函数参考 | https://mapster.talv.space/galaxy/reference |
| Galaxy 语法定义 | https://github.com/Talv/vscode-sc2-galaxy/blob/master/syntaxes/galaxy.json |
| **SC2-IngameDevTools（首选 —— 第一代码库）** | https://github.com/abrahamYG/SC2-IngameDevTools/tree/main/DevToolsIngame.SC2Mod/Script |
| SSF 代码库（次要风格参考） | https://github.com/Cristall/SC2-SwarmSpecialForces/tree/main/SwarmSpecialForces.SC2Map/scripts |
| NativeLib actor 辅助函数 | `TriggerLibs/NativeLib.galaxy` — 全部 `libNtve_gf_Attach*`、`libNtve_gf_Create*`、`libNtve_gf_PlayAnimation*`、`libNtve_gf_Set*`、`libNtve_gf_SendActorMessage*` 函数 |
| SC2 编辑器指南 | https://s2editor-guides.readthedocs.io |
| SC2Mapster wiki | https://sc2mapster.wiki.gg/ |
| Actor 事件（wiki） | https://sc2mapster.wiki.gg/wiki/Data/Actors/Events |
| Actor 术语（wiki） | https://sc2mapster.wiki.gg/wiki/Data/Actors/Terms |
