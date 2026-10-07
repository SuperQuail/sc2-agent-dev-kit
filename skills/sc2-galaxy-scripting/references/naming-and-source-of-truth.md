# 命名约定与真源

## 真源

- **只编辑：** `<SC2_MODS_PATH>/AeonOfIhanrii.SC2Mod/Base.SC2Data/Epi_Main.galaxy`（主自定义脚本块），或 `*.SC2Mod/Base.SC2Data/Scripts/` 下的模块化脚本。
- **不要编辑：** `Lib*.galaxy`（编译库包装，自动生成，编辑器保存时被覆盖）或地图里的 `MapScript.galaxy`（自动生成）。
- **部署：** `workspace_copy` 模式下先预览，再用 `python tools/deploy-mod.py` 把组件源复制到 SC2。`in_place` 模式下不需要部署。保存时重新生成编译包装的是 SC2 编辑器，不是这个工具。

## 命名约定（AeonOfIhanrii）

- Galaxy 函数前缀：`libEpi_`
- Galaxy 全局变量前缀：`libEpi_g_`
- Bank 名：`"AeonOfIhanriiBank"`

写代码前对照当前项目身份确认实际前缀——见 `sc2-project-entry`。

## 来源注意事项

**Galaxy 运算符冲突：** `galaxy-gotchas.md` 推荐 `+=`；触发器流程的坑则报告 `+=` 在 `ScriptCode` 上下文里失败，且 `break` 的行为随上下文而异。新写的手写 Custom Script 用显式赋值和 `while`；先区分脚本上下文，再让编辑器编译。
