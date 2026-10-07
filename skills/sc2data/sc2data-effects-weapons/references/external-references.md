# 外部参考 — 效果（Effect）与武器

效果、武器与升级的权威 schema、wiki 与上游 XML 来源。

## 主要参考

| 资源 | URL |
|---|---|
| Data Editor Introduction | https://s2editor-guides.readthedocs.io/New_Tutorials/04_Data_Editor/058_Data_Editor_Introduction/ |
| SC2Mapster wiki | https://sc2mapster.wiki.gg/ |
| Effects（wiki） | https://sc2mapster.wiki.gg/wiki/Data/Effects |
| Targets for Effects（wiki） | https://sc2mapster.wiki.gg/wiki/Targets_for_Effects |
| Weapons（wiki） | https://sc2mapster.wiki.gg/wiki/Data/Weapons |
| SC2Mapster mkdocs reference | https://sc2mapster.github.io/mkdocs/galaxy/ |
| ShadowDragon Base.SC2Data（真实 GameData XML 参考） | https://github.com/ShadowDragonSC2/Base.SC2Data/tree/main/GameData |
| **唯一真源：CatalogsData XSD Schema** | https://github.com/ShadowDragonSC2/Base.SC2Data/raw/refs/heads/main/.vscode/schemas/catalogsData.xsd — 效果、武器与升级的确切字段、属性、结构一律以此为准。不要假设字段存在，去 schema 里核对。 |
| **推荐的 VS Code 扩展** | Red Hat XML (redhat.vscode-xml) — 安装后可用 catalogsData.xsd 做 XML 校验、自动补全与错误检测。在 .vscode/settings.json 中配置即可自动校验。 |
