# 来源与指南

本子技能的上游代码库、NativeLib 同盟辅助函数与编辑器指南。

## 前置

先加载 [`sc2-galaxy-scripting`](../../../sc2-galaxy-scripting/SKILL.md)，获取项目级 Galaxy 语法约束（局部变量提升（hoisting）、不支持 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、真源规则、模块化脚本边界与命名约定（例如 `libMy_` 函数前缀、`libMy_g_` 全局前缀）。本子技能只负责深度 API 参考；项目约定在路由技能里。

## 关键参考

| 资源 | URL |
|---|---|
| 原生函数参考 | https://mapster.talv.space/galaxy/reference |
| Galaxy 语法定义 | https://github.com/Talv/vscode-sc2-galaxy/blob/master/syntaxes/galaxy.json |
| **SC2-IngameDevTools（首选 —— 第一代码库）** | https://github.com/abrahamYG/SC2-IngameDevTools/tree/main/DevToolsIngame.SC2Mod/Script |
| SSF 代码库（次要风格参考） | https://github.com/Cristall/SC2-SwarmSpecialForces/tree/main/SwarmSpecialForces.SC2Map/scripts |
| Alcyone Frontlines 代码库 | https://github.com/KimPlaybit/Alcyone_Frontlines/tree/master/ProximaFrontlines.SC2Mod/scripts |
| NativeLib 同盟辅助函数 | `TriggerLibs/NativeLib.galaxy` — `libNtve_gf_SetAlliance`、`libNtve_gf_SetPlayerGroupAlliance`、`libNtve_gf_SetAllianceBetweenTwoPlayerGroups`、`libNtve_gf_PlayerIsEnemy` |
| 变量指南 | https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/037_Variables/ |
| 记录（Record）指南 | https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/040_Records/ |
| SC2Mapster wiki | https://sc2mapster.wiki.gg/ |
