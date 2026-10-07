# 外部参考 — 单位与技能

单位与技能数据的权威 schema、wiki 与上游 XML 来源。

## 主要参考

| 资源 | URL |
|---|---|
| Data Editor Introduction | https://s2editor-guides.readthedocs.io/New_Tutorials/04_Data_Editor/058_Data_Editor_Introduction/ |
| Units guide | https://s2editor-guides.readthedocs.io/New_Tutorials/04_Data_Editor/059_Units/ |
| Validators guide | https://s2editor-guides.readthedocs.io/New_Tutorials/04_Data_Editor/071_Validators/ |
| Buttons guide | https://s2editor-guides.readthedocs.io/New_Tutorials/04_Data_Editor/075_Buttons/ |
| SC2Mapster wiki | https://sc2mapster.wiki.gg/ |
| Abilities（wiki） | https://sc2mapster.wiki.gg/wiki/Data/Abilities |
| Units（wiki） | https://sc2mapster.wiki.gg/wiki/Data/Units |
| Data Editor settings（wiki） | https://sc2mapster.wiki.gg/wiki/Data_Types |
| ShadowDragon Base.SC2Data（真实 GameData XML 参考） | https://github.com/ShadowDragonSC2/Base.SC2Data/tree/main/GameData |
| **唯一真源：CatalogsData XSD Schema** | https://github.com/ShadowDragonSC2/Base.SC2Data/raw/refs/heads/main/.vscode/schemas/catalogsData.xsd — 单位、技能、Mover、Turret、Requirement 与 Race 的确切字段、属性、结构一律以此为准。不要假设字段存在，去 schema 里核对。 |
| **推荐的 VS Code 扩展** | Red Hat XML (redhat.vscode-xml) — 安装后可用 catalogsData.xsd 做 XML 校验、自动补全与错误检测。在 .vscode/settings.json 中配置即可自动校验。
| **SC2 单位目录 ID** | 全部多人单位/建筑目录 ID（编辑器 ID）、种族、属性与所属资料片的完整清单见 `sc2data-units-reference` 技能——查 parent 或既有单位该引用哪个 `id` 字符串时使用。
