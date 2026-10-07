# 智能体上下文效率

当任务有加载过多仓库上下文的风险时用本页。目标是让实现过程有界、可复现，并聚焦在所需的最小证据上。

## 上下文膨胀的来源

1. **原始 catalog 转储与组件样例。** `DataEditorXML/*.txt` 与 `DataEditorXML/SC2GameDataComponents/` 有部分重叠，一起搜会产生大量重复结果。
2. **生成的图产物。** `sc2-catalog-graph-out/graph.json` 与对象摘要是生成的，可能远大于任务所需。
3. **历史日志。** 旧的修正历史对识别模式有用，但在查主题页之前先读整份归档/日志页，会重复过时的实现细节。
4. **无界搜索结果。** 对所有 XML 与 wiki 文件反复 `rg`，会在依赖、本地模组行与生成输出之间产生重复命中。

## 默认导航顺序

1. 读 `AGENTS.md`，然后直接按任务路由表跳转。只有当任务路由表没有点出所需页面时，才去翻目录。
2. SC2 catalog 问题，先跑确定性 catalog 查询：

```powershell
python tools/sc2-catalog-query.py find <text> --source-class local_mod --limit 20
python tools/sc2-catalog-query.py show <Family:Id> --limit 30 --depth 2
python tools/sc2-catalog-query.py providers <Family:Id>
python tools/sc2-catalog-query.py unresolved --contains <text> --limit 40
python tools/sc2-catalog-query.py path <Family:Id> <Family:Id> --max-depth 4
python tools/sc2-catalog-query.py unit-chain <Unit:Id> --source-class local_mod
python tools/sc2-catalog-query.py production-chain <Unit-or-Abil:Id> --source-class local_mod
```

3. 某个 catalog 实现需要具体的官方/合作先例或中英文字符串时，对组件快照跑一次有界查询。已知时尽量指定 catalog 族与候选组件：

```powershell
python tools/sc2-reference-query.py components --source CM
python tools/sc2-reference-query.py find 'id="Marine"' --area gamedata --family Unit --component liberty --limit 20
python tools/sc2-reference-query.py find 'Unit/Name/Marine' --area zhcn --component liberty --limit 10
python tools/sc2-reference-query.py object Unit:Marine --component liberty --max-chars 6000
```

只打开返回的文件片段。记下它所在的组件与行号，并在同一任务里复用。快照是示例来源，不是活动依赖或有效值的证据。刷新过快照不需要重建 catalog 图。
如果 catalog 查询报告索引过期，先跑一次 `python tools/build-sc2-catalog-graph.py --sqlite-only`，再用当前项目结果。只有 `--allow-stale` 才允许有意查看旧数据。

4. 需要历史实现背景时，查相关路径的 Git 历史：

```powershell
git log --oneline --all -- <path>
```

先读当前正页；历史用于了解先前状态，不是工作契约。

5. 等持久 wiki 或某次有界查询定位到 catalog 族、ID、文件或 wiki 页之后，再用 `rg`。把它限定在一个选定的转储或组件文件夹里。

## 常见 XML 失败类别

1. **提取图不完整**——单位/技能/效果/行为/Actor 行复制了，但模型、音效、炮塔、验证器、辅助武器、需求节点、按钮或本地化缺失。
2. **编辑器规范化不一致**——编辑器保存后，`GameData.xml` 与带类型的旁路 catalog 之间出现重复的 `(catalog type, id)`。
3. **本地化归属错位**——面向玩家的文本没有锚定在 `GameStrings.txt`；编辑器标签没进 `ObjectStrings.txt`。
4. **Actor 与命令卡继承陷阱**——带父级的 Actor 保留祖先事件；继承的命令卡把原版父级的按钮漏进来。

本地 XML 解析抓不到编辑器 schema 告警、缺失的活动行、错误的命令卡层、Actor 作用域冲突或被剪掉的文本。见 [xml-patterns.md](../../sc2-catalog-xml/references/xml-patterns.md#editor-round-trip-checklist)。

## 实用规则

- 绝不编辑编辑器自动生成的文件（`MapScript.galaxy`、`Lib*.galaxy`、`ComponentList.xml`）。新触发器 GUI 优先；如果用户明确要求独立 Galaxy 实现，或既有脚本需要维护，把手写代码留在 `*_ScriptBlock.galaxy` 或 `Base.SC2Data/Scripts/`。
- 绝不把整份原始 XML 转储粘进或加载进模型上下文。
- 编辑器交接前按引擎规则校验 XML 与 Galaxy 语法：`python tools/test-suite.py`。
- 避免直接读 `sc2-catalog-graph-out/graph.json` 或 `catalog.sqlite`；用 `tools/sc2-catalog-query.py`。
- 用 `tools/sc2-reference-query.py` 查组件快照；不要为每个字段反复跑整树 `rg`。
- 在生成目录/catalog 目录里 grep 时，尽量用 `--glob` 与 `--max-count` 约束搜索。
- 把 `本地开发记录的 decisions.md（sc2.py record）` 当作简短的决策记录，而不是当前真源。

## Catalog 图工具

用 `python tools/build-sc2-catalog-graph.py --sqlite-only` 刷新查询数据库。只有在需要完整 JSON、GraphML 与对象报告时才去掉该标志。查询一律通过 `tools/sc2-catalog-query.py`。

## 工具故障恢复

如果补丁辅助脚本报错或替换失败，重开会话，或用精确字符串替换并保留 UTF-8 换行。跑 `python tools/test-suite.py` 核验格式与语法。绝不编辑 `publish/` 里的文件。
