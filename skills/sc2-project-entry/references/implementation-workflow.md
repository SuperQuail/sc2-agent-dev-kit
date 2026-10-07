# 实现流程

1. **记录有效值：** 改数值前，记录对象 ID、父级、准确的字段/数组索引、活动依赖提供的有效值，以及打算做的覆盖。本地缺字段 ≠ 取值为零。
2. **先查询再落笔：** 通过 catalog 图查询当前项目与活动依赖；如果它报告输入过期，先用 `python tools/build-sc2-catalog-graph.py --sqlite-only` 刷新一次再依赖结果。活动依赖缺失或损坏会中止当前数据索引。只有显式的 `--allow-incomplete-dependencies` 才允许带标记的部分只读调查，未解决的问题会保存在清单里并在每次查询时显示；不要据此声称拿到完整有效值或据此写入。然后只看相关的原始 XML。要具体先例，用 `sc2-reference-query.py object <Family:Id> --component <name>` 或有界的 `find`。本任务期间复用这份证据。快照样例无法建立活动依赖或有效值。
3. **项目唯一 ID：** 变体用项目唯一 ID；只有当用户明确要求全局改动时才覆盖同名原版 ID。同步检查游戏性、生产入口、表现与本地化链。
4. **只改组件源：** 不要手改 `publish/`、自动生成的 `MapScript.galaxy` 或编译库输出。遵守当前项目手写脚本的入口。
5. **先静态后运行时：** 对真实目标跑静态检查并确认扫描范围；然后安排编辑器打开/保存与游戏内场景验证。没有证据，不要把静态通过说成编辑器接受或游戏通过。
6. **记录决策：** 长期设计事实写进 `DesignDocument.md`，实现约定写进对应技能的 `references/` 主题页。只有当某项决策的理由否则会丢失时，才单独记录它。

## 大型查询源（不要通读）

- `DataEditorXML/*.txt` — 图已覆盖的选定 catalog 转储；对某一个已识别的文件 grep 精确字段
- `DataEditorXML/SC2GameDataComponents/` — 用 `tools/sc2-reference-query.py` 查询，限定族、组件与区域
- `sc2-catalog-graph-out/` — 用 `tools/sc2-catalog-query.py` 查询
- `skills/sc2-map-triggers/references/triggers-native-functions.md` 与 `skills/sc2-map-triggers/references/triggers-native/` — native 函数签名
- Git 历史（`git log -n <N> -- <path>`）查历史出处
