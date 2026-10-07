# 外部参考 — Actor

Actor 数据与 Actor 事件的权威 schema、wiki 与教程来源。

## 主要参考

| 资源 | URL |
|---|---|
| Actors guide | https://s2editor-guides.readthedocs.io/New_Tutorials/04_Data_Editor/060_Actors/ |
| Actor Messages Rundown | https://s2editor-guides.readthedocs.io/New_Tutorials/04_Data_Editor/061_Actor_Messages_Rundown/ |
| SC2Mapster wiki | https://sc2mapster.wiki.gg/ |
| Actors（wiki） | https://sc2mapster.wiki.gg/wiki/Data/Actors |
| Actor Events（wiki） | https://sc2mapster.wiki.gg/wiki/Data/Actors/Events |
| Actor Terms（wiki） | https://sc2mapster.wiki.gg/wiki/Data/Actors/Terms |
| Blizzard Tutorials – Actors.md | https://github.com/SC2Mapster/blizzard-tutorials/blob/master/docs/New_Tutorials/04_Data_Editor/060_Actors.md |
| ShadowDragon Base.SC2Data（真实 GameData XML 参考） | https://github.com/ShadowDragonSC2/Base.SC2Data/tree/main/GameData |
| **唯一真源：CatalogsData XSD Schema** | https://github.com/ShadowDragonSC2/Base.SC2Data/raw/refs/heads/main/.vscode/schemas/catalogsData.xsd — 每种 Actor 类型的确切字段、属性、结构一律以此为准。不要假设字段存在，去 schema 里核对。 |
| **推荐的 VS Code 扩展** | Red Hat XML (redhat.vscode-xml) — 安装后可用 catalogsData.xsd 做 XML 校验、自动补全与错误检测。在 .vscode/settings.json 中配置即可自动校验。 |
