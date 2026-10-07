# 来源与指南

本子技能的上游代码库、原生参考与编辑器指南。自己造 API 名字之前先读这里。

## 前置

先加载 [`sc2-galaxy-scripting`](../../../sc2-galaxy-scripting/SKILL.md)，获取项目级 Galaxy 语法约束（局部变量提升、不支持 `++`/`--`、用 `while` 而非 `for`、include 路径不带扩展名）、真源规则、模块化脚本边界与命名约定（例如 `libMy_` 函数前缀、`libMy_g_` 全局前缀）。本子技能只负责深度 API 参考；项目约定在路由技能里。

## 关键参考

| 资源 | URL |
|---|---|
| 原生函数参考 | https://mapster.talv.space/galaxy/reference |
| Galaxy 语法定义 | https://github.com/Talv/vscode-sc2-galaxy/blob/master/syntaxes/galaxy.json |
| **SC2-IngameDevTools（首选 —— 第一代码库）** | https://github.com/abrahamYG/SC2-IngameDevTools/tree/main/DevToolsIngame.SC2Mod/Script |
| SSF 代码库（次要风格参考） | https://github.com/Cristall/SC2-SwarmSpecialForces/tree/main/SwarmSpecialForces.SC2Map/scripts |
| Alcyone Frontlines 代码库 | https://github.com/KimPlaybit/Alcyone_Frontlines/tree/master/ProximaFrontlines.SC2Mod/scripts |
| NativeLib | `TriggerLibs/NativeLib.galaxy`（sc2galaxy VS Code 扩展） |
| 自定义值指南 | https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/050_Custom_Values/ |
| 单位选中事件指南 | https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/048_Unit_Selection_Events/ |
| SC2Mapster wiki | https://sc2mapster.wiki.gg/ |
| **SC2 单位类型 ID** | 全部对战单位类型 ID 字符串（例如 `"SiegeTank"`、`"HighTemplar"`、`"BroodLord"`）、变形/蜕变的 ID 配对、种族、属性与资料片清单见技能 `sc2data-units-reference`——在调用 `UnitCreate`、`UnitGetType` 或判断单位属性时使用。 |
