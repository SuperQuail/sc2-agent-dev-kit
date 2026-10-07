# 外部 SC2 模组开发资源

把本页当作通往外部参考的快速路由。长期项目决策留在本 wiki；外部站点用于查字段含义、签名与更宽的编辑器概念。

## 资源地图

| 来源 | 最适合 | 备注 |
|---|---|---|
| [SC2Mapster Wiki](https://sc2mapster.wiki.gg/) | 覆盖地形、数据、触发器、UI、地图/模组、脚本、AI 与调试的广义编辑器入门。 | 两者都存在时优先用 `wiki.gg` 而不是遗留的 Fandom。搜索仍可能落到 Fandom 镜像。 |
| [Language Overview](https://sc2mapster.fandom.com/wiki/Language_Overview#List_of_SC2_natives) | Galaxy 语言约束与心智模型。 | 适合查语法陷阱：局部声明放顶部、数组维度写在类型后、没有 `++`、没有块注释、没有动态分配、强类型。 |
| [Talv Galaxy Reference](https://mapster.talv.space/galaxy/reference) | native/库函数签名、GUI 名、返回类型、preset 与分类浏览。 | Galaxy 调用的首要在线查询处。也覆盖 UI 布局文档并链接数据文档。 |
| [Talv Data File List](https://mapster.talv.space/data/files.html) | 数据 schema 的头文件级路由。 | 已知 catalog 族时好用：`Unit.h`、`Abil.h`、`Behavior.h`、`Effect.h`、`Requirement.h`、`Actor.h`、`Weapon.h` 等。 |
| [Talv Data Class List](https://mapster.talv.space/data/annotated.html) | catalog 类/成员浏览与继承形态。 | 本地 XML grep 之后用来查字段含义。例如 `CUnit`、`CAbilTrain`、`CAbilEffect`、`CBehaviorBuff`、`CEffectDamage`、`CRequirement*`、`CActorUnit`。 |
| [sc2-gamedata-documentation](https://github.com/chansey97/sc2-gamedata-documentation/tree/master) | 数据结构用的 Windows 离线参考。 | 上游项目提供编译好的 CHM 发行版。Talv 的在线数据文档致谢了该项目。 |
| [SC2Mapster Triggers](https://sc2mapster.wiki.gg/wiki/Triggers) | 触发器编辑器概念、元素类型、GUI 布局与分类导航。 | 适合人工编辑器操作；外部生成触发器 XML 用本地触发器 XML 页。 |
| [SC2Mapster Errors/Debugging](https://sc2mapster.wiki.gg/wiki/Errors/Debugging) | Galaxy 编译/运行时错误分诊。 | 先查本地 [galaxy-gotchas.md](../../sc2-galaxy-scripting/references/galaxy-gotchas.md)，再用它。 |
| [SC2 Editor Tutorials - Data Editor](https://s2editor-guides.readthedocs.io/New_Tutorials/04_Data_Editor/058_Data_Editor_Introduction/) | 面向 catalog 的数据编辑器概念。 | 单位、Actor、按钮、依赖与数据空间。 |
| [SC2Mapster/SC2GameData](https://github.com/SC2Mapster/SC2GameData) | 公开导出的 GameData/UI/Galaxy 参考。 | 适合交叉核对暴雪的写法。 |

## 工具、语言服务器与 schema

| 工具 / 仓库 | 用途 | 备注 |
|---|---|---|
| [sc2-galaxy-toolkit](https://github.com/sc2-arcade-watcher/sc2-galaxy-toolkit) | Galaxy Script 的 VS Code 语言扩展。 | 语法高亮、符号查找与诊断。 |
| [Talv/plaxtony](https://github.com/Talv/plaxtony) | Galaxy、Triggers 与 GameData XML 的静态分析与 AST 解析器。 | 基于 TypeScript 的 SC2 解析器；构建符号表并校验 schema。 |
| [sc2-arcade-watcher/sc2-xsd](https://github.com/sc2-arcade-watcher/sc2-xsd) | 星际争霸 II GameData XML 的正式 XSD schema。 | 按引擎规范做实时 XML catalog 校验。 |
| [SC2Mapster/sc2layout-schema](https://github.com/SC2Mapster/sc2layout-schema) | SC2Layout 定义的正式 XSD schema。 | 自定义 UI 框体与布局的 schema 校验。 |
| [Talv/sc2-layouts](https://github.com/Talv/sc2-layouts) | SC2Layout 文件的 VS Code 扩展。 | 语法高亮与框体 schema 校验。 |
| [sc2-modkit](https://github.com/sc2-arcade-watcher/sc2-modkit) | SC2 的 VS Code 模组开发套件。 | 项目脚手架、打包与编辑器集成。 |
| [Talv/vscode-sc2-galaxy](https://github.com/Talv/vscode-sc2-galaxy) | VS Code 的 Galaxy 语言支持。 | Galaxy 脚本的语言服务器集成。 |
| [galaxy-parser](https://github.com/rameshvarun/galaxy-parser) | Galaxy Script 的正式 EBNF 文法与解析器。 | 文法规范参考。 |
| [Galaxy Language Fundamentals (LobeHub)](https://lobehub.com/de/skills/kimplaybit-starcraft-2-editor-skills-galaxy-language-fundamentals) | 面向编码智能体的 Galaxy 语法指南。 | 局部变量提升（hoisting）、复合赋值、基础类型规则。 |

## 查询工作流

1. 精确的 XML 字段、索引、ID 或暴雪写法：先 grep `DataEditorXML/`，如果是在提取未使用依赖的单位，再查非活动 XML。
2. 已知数据字段的含义：用 Talv Data Class List 或 File List，然后只把与项目相关的结论记进本 wiki。
3. Galaxy 函数签名：先搜既有 `.galaxy` 文件，再用 Talv Galaxy Reference；生成触发器 XML 时用本地 `triggers-native/` 页。
4. 触发器 GUI 概念或分类：用 SC2Mapster Triggers；schema 规则用本地触发器 XML 页。
5. 编译/运行时错误：先查本地陷阱页，再用 SC2Mapster Errors/Debugging 找更广的错误名。
6. XML/Layout 校验：对照 `sc2-xsd` 与 `sc2layout-schema`。

## 项目规则

外部页面是参考资料，不是真源。当某次查询影响到项目时，把那条小结论加到相关的 wiki 页；理由长期有效时，再补进 `本地开发记录的 decisions.md（sc2.py record）`。
