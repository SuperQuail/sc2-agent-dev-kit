# 来源、代码库与前置要求

## 前置要求

先加载 [`sc2-galaxy-scripting`](../../../sc2-galaxy-scripting/SKILL.md)，了解项目级 Galaxy 语法约束（局部变量提升（hoisting）、没有 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、唯一真源规则、模块化脚本边界与命名约定（如 `libMy_` 函数前缀、`libMy_g_` 全局前缀）。本子技能只拥有深层 API 参考；项目约定在路由技能里。

## 关键资源

| 资源 | URL |
|---|---|
| Native 函数参考 | https://mapster.talv.space/galaxy/reference |
| Galaxy 语法定义 | https://github.com/Talv/vscode-sc2-galaxy/blob/master/syntaxes/galaxy.json |
| Banks 指南 | https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/051_Banks/ |
| Custom Values 指南 | https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/050_Custom_Values/ |
| **SC2-IngameDevTools（首选——第一代码库）** | https://github.com/abrahamYG/SC2-IngameDevTools/tree/main/DevToolsIngame.SC2Mod/Script |
| SSF 代码库（次选风格） | https://github.com/Cristall/SC2-SwarmSpecialForces/tree/main/SwarmSpecialForces.SC2Map/scripts |
| Alcyone Frontlines 代码库 | https://github.com/KimPlaybit/Alcyone_Frontlines/tree/master/ProximaFrontlines.SC2Mod/scripts |
| NativeLib | `TriggerLibs/NativeLib.galaxy`（sc2galaxy VS Code 扩展） |
| SC2Mapster wiki | https://sc2mapster.wiki.gg/ |
