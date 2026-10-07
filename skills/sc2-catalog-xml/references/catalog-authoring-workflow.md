# Catalog 编写流程

任何 GameData XML 改动的强制顺序。跳过查询或证据步骤，通常就是白跑一轮的原因。

## 编辑前（强制）

1. **先查询：** 对目标 ID、单位或族跑 `python tools/sc2-catalog-query.py`。确认当前项目与活动依赖的提供者。不要猜字段名或 ID——它们区分大小写。
2. **看一份源：** 打开相关的当前 XML 片段。如果还需要字段形态或已验证的写法，搜某一个已识别的 `DataEditorXML/*.txt` 转储。要组件级的官方/合作先例，跑 `python tools/sc2-reference-query.py find <term> --area gamedata --family <Family> --component <name> --limit 20`。复用查到的结果，不要重复大范围搜索。快照里存在并不证明存在活动依赖。
3. **读模式：** 查 [references/xml-patterns.md](xml-patterns.md) 与 [references/catalog-rules.md](catalog-rules.md) 的相应章节。
4. **有效值证据：** 改数值前先证明有效值——记录本地单位、父级、精确字段/索引处的继承值，以及打算做的覆盖。父级提供取值时，子级缺字段可能仍是对的。
5. **新行写进带类型的 catalog：** `AbilData.xml`、`UnitData.xml`、`ButtonData.xml` 等——不要写进 `GameData.xml`。避免跨文件出现重复的 `(catalog type, id)`。

## 编辑后校验

1. `python tools/validate-mod.py` — XSD schema + Galaxy 语法
2. `python tools/test-suite.py` — 完整预检套件
3. 编辑器交接：在 SC2 编辑器中打开模组，审阅 XML 告警，另存为 Components
4. 保存之后跑 `python tools/audit-gamestrings-anchors.py --fill`

静态通过不是编辑器接受，也不是运行时验证——见 `sc2-editor-handoff`。

## 查重

加验证器或实体之前，先查它在依赖链里是否已存在。`python tools/sc2-catalog-query.py find <term>` 与未解析引用报告是最省事的查法。
