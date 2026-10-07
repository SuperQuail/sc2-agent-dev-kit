# 什么必须用 SC2 编辑器 GUI

以下操作智能体做不了，必须由用户完成：

- 打开并保存暴雪地图（需要 SC2 账号登录）
- 地形、区域、doodad、寻路
- 过场动画与影片
- 把模组/地图另存为 Components（用户操作）

智能体准备好一切并交付精确检查单——见 `sc2-editor-handoff`。

## 项目初始化

用户给出主 Components `.SC2Mod` 文件夹路径并要求初始化或选择项目时，直接跑 `python tools/init-project.py "<path>"`。不要让用户去编辑 `agent-config.json` 或列举依赖。只有当工具推断不出非标准布局时，才追问额外路径。初始化之后，从证据核验项目专属身份；绝不猜 Bank 名、Library ID、脚本块或代码前缀。

另见 [references/project-initialization.md](project-initialization.md)、[references/multi-pc-setup.md](multi-pc-setup.md)、[references/external-paths.md](external-paths.md) 与 [references/external-sc2-resources.md](external-sc2-resources.md)。
