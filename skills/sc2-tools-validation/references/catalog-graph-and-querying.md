# Catalog 图与查询

## `sc2-catalog-query.py`（主要查询工具）

基于 `sc2-catalog-graph-out/catalog.sqlite`（或 `graph.json`）的省 token catalog 查询。用它，**不要** 去读原始 `DataEditorXML/` 转储。

```bash
python tools/sc2-catalog-query.py find <term> --source-class local_mod
python tools/sc2-catalog-query.py unit-chain <Unit:Id>
python tools/sc2-catalog-query.py production-chain <Unit:Id>
python tools/sc2-catalog-query.py actor-chain <Actor:Id>
python tools/sc2-catalog-query.py show <Family:Id> --limit 30 --depth 2
python tools/sc2-catalog-query.py unresolved --contains <term>
```

- 技能链用 `ability-chain Abil:<ActualId>`。
- 可疑的未解析引用用 `unresolved --contains <ActualPrefix> --limit 30`。
- 给 `find` / 链查询加 `--source-class local_mod` 可收窄到本地记录。**查询基础对象时不要用 `local_mod` 过滤**——它会把基础依赖条目藏掉。

## 完整 PowerShell 查询序列

```powershell
python tools/sc2-catalog-query.py stats
python tools/sc2-catalog-query.py find Marine --family Unit --limit 10
python tools/sc2-catalog-query.py providers Unit:Marine
python tools/sc2-catalog-query.py show Unit:Marine --depth 2 --limit 20
python tools/sc2-catalog-query.py unit-chain Unit:Marine --limit 10
python tools/sc2-catalog-query.py production-chain Unit:Marine --limit 10
python tools/sc2-catalog-query.py actor-chain Unit:Marine --limit 10
```

## `build-sc2-catalog-graph.py`

从参考导出、配置的主 Mod，以及递归解析到的本地组件依赖重建 catalog 数据。要当前查询数据库就加 `--sqlite-only`；还需要 JSON、GraphML 与摘要报告时就不要加。主定义用 `local_mod`；已解析的依赖定义用 `active_component_dependency`。

查询索引缺失或过期时，用 `python tools/build-sc2-catalog-graph.py --sqlite-only` 刷新。先确认它的输入配置覆盖了目标模组。不要为单次样例查询重建。

## 数据库注意事项

- 默认 catalog 查询即使没有活动模组，也会检查清单的新增、删除与输入变更。配置的源缺失/非法会挡住当前数据查询；修好配置之后才重建。`--allow-stale` 允许显式的历史查看，并打印「仅历史」警告。`reference_export` 是 TXT 参考源，而 `active_component_dependency` 来自已解析的组件链。交叉核对当前 XML、依赖声明与源路径。
- 只有当图和选定的转储回答不了某个问题时，才去搜组件快照；把来源、组件、族、区域与结果上限都收窄。

## `sc2-reference-query.py`

对 `DataEditorXML/SC2GameDataComponents/` 的只读、有界搜索。用 `components` 识别包；用带 `--area gamedata|enus|zhcn`、`--family`、`--component`、`--limit` 的 `find` 搜一行/一个字符串；或用 `object <Family:Id> --component <name>` 取一个完整 catalog 对象。顶层 `DataEditorXML/*.txt` 转储已含部分相同 XML；先查图。样例快照不包含在图里，也不意味着存在活动依赖。
