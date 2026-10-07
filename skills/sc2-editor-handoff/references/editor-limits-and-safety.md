# 编辑器安全规则与交接边界

## 智能体做不到的事（需要 SC2 编辑器 GUI）

- 打开并保存暴雪地图（需要 SC2 账号登录）
- 地形、区域、doodad、寻路
- 过场动画与影片
- 把模组/地图另存为 Components（用户操作）

这些步骤必须由用户执行。智能体准备好一切，并给出精确的交接检查单。

## 编辑器安全规则

- **绝不** 在项目模组作为外部 override 激活时打开地图——那样加载的是改过的版本而不是原版。
- **添加依赖：** Map → Modules → Dependencies → `<ModName>.SC2Mod`。
- 编辑器保存之后，跑本地化审计以恢复被剪掉的字符串锚点。

另见 [references/editor-guide.md](editor-guide.md) 的 SC2 编辑器工作流笔记。
