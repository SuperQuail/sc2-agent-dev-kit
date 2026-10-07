# 编辑器交接检查单

## Galaxy 脚本改动之后

1. `workspace_copy` 模式下，先用 `python tools/deploy-mod.py --dry-run` 预览再用部署命令复制组件模组；`in_place` 模式下跳过部署。
2. 用户在 SC2 编辑器中打开 `<ModName>.SC2Mod`，在触发器编辑器里确认预期的自定义脚本块/include。
3. 编译触发器 → **File → Save**，把模组另存为 **Components**；只有编辑器能重新生成 `Lib*.galaxy`。

## 模组 XML / 触发器 GUI 改动之后

1. 用户在 SC2 编辑器中打开 `<ModName>.SC2Mod`。
2. 查看编辑器输出面板里的 XML 告警；保存前解决 catalog 类型/ID 问题。
3. **File → Save**，把模组另存为 **Components**。
4. 如果在地图触发器 XML 里直接改过仓库中的 `Triggers`，请在编辑器中重新打开受影响的 `.SC2Map` 并另存为 Components，好让 `MapScript.galaxy` 干净地重新生成。
5. 保存之后：跑 `python tools/audit-gamestrings-anchors.py --fill`。

## 地图组件改动之后

1. 用户在 SC2 编辑器中打开配置的 `paths.campaign_maps_dir/<Category>/<MapName>.SC2Map/` 下的地图。
2. 确认 **Modules → Dependencies** 里包含 `<ModName>.SC2Mod`。
3. **绝不** 在项目模组作为外部 override 激活时打开暴雪地图（那样加载的是改过的版本而不是原版）。
4. **File → Save**，把地图另存为 **Components**。

## Playtest 之后

1. 跑 `python tools/extract-playtest-bugreport.py` 提取运行时告警/错误。
2. 按问题生命周期分诊。
3. 把缺陷登记到 `sc2agent-records/<project>/issues.md`。
