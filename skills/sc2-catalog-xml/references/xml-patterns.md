# 已验证 XML 模式枢纽

经过实战检验的星际争霸 II 数据编辑器 XML 模式、catalog 规则、生产配置与编辑器保存行为。

## XML 模式子页

- [references/catalog-rules.md](catalog-rules.md) — 正式的 XML schema 规则、继承机制、数组操作（`index`、`removed`）与默认值行为。
- [references/core.md](core.md) — 单位创建检查单、武器链、伤害效果、持续效果、增益、需求与验证器。
- [references/production.md](production.md) — `CAbilTrain`、`CAbilWarpTrain`、`CAbilMorph`、命令卡费用显示与多层升级链接。
- [references/data-editor-xml.md](data-editor-xml.md) — 打包的 `DataEditorXML/` 转储索引及每个文件的内容。
- [references/xml-schema-basics.md](xml-schema-basics.md) — 取值子元素、行为嵌套、命令卡布局、数组索引、ASCII 规则。
- [references/inheritance-and-parent-pitfalls.md](inheritance-and-parent-pitfalls.md) — `parent=` 语义、Actor 变体、变形变体、验证器、伤害字段。
- [references/footprints-and-editor-categories.md](footprints-and-editor-categories.md) — 足迹层与安全的 `EditorCategories` 取值。
- [references/production-command-cards-and-costs.md](production-command-cards-and-costs.md) — 生产核验顺序、子菜单、费用显示、折跃、升级。
- [references/catalog-authoring-workflow.md](catalog-authoring-workflow.md) — 强制的编辑前查询/取证顺序与编辑后校验。
- [references/source-caveats.md](source-caveats.md) — 源文档中已记录的矛盾及其解决方式。
- [编辑器往返与 Actor](../../sc2-actor-system/references/editor-roundtrip-and-actors.md) — SC2 编辑器保存规范化、专用 catalog 提升、Actor 事件绑定与模型/音效串联。

## 新自定义单位速查

1. **`UnitData.xml`：** 建一个自定义 `CUnit`，带唯一 ID、标志位、武器链接、技能与卡布局。
2. **`AbilData.xml` / `WeaponData.xml` / `EffectData.xml`：** 克隆或自建专用的技能/武器/效果链。
3. **`ActorData.xml` / `ModelData.xml` / `SoundData.xml`：** 创建显式的单位与动作 Actor，链接到你的自定义效果/事件。
4. **`LocalizedData/`：** 面向玩家的字符串加到 `GameStrings.txt`，编辑器元数据加到 `ObjectStrings.txt`。
5. **核验：** 打开编辑器之前先跑 `python tools/test-suite.py`。
