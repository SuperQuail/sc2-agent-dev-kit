---
name: sc2data-wizards
description: SC2 Data Editor — Wizards for automating complex or repetitive data creation/modification in XML. Use when creating .BlizWiz XML files to define templates for generating catalog entries (units, abilities, effects, actors, etc.) with user inputs, conditions, validations, and macros. Covers wizard elements (input, entry, condition, validate, macro), string evaluation (tokens, catalog references, arithmetic), and file placement. Always reference the Data Wizard Documentation for syntax. Do not use for direct data editing (use other sc2data-* skills) or Galaxy scripting. Start by running "python tools/sc2.py start" to read this project's previous task records.
---

## 何时使用

写 `.BlizWiz` 向导（Wizard）模板时加载本技能——用对话框、输入项、条件、校验和宏来自动创建或修改目录（catalog）条目（单位、技能、效果、Actor 等），其中包含带 token、目录引用和算术的字符串求值。向导给用户一个对话框，一次生成多个相互关联的条目，强制规则，并把重复的数据工作流标准化；向导文件从用户文件夹、地图或模组加载，在数据编辑器菜单里运行。

## 加载边界

- **本子技能拥有：** .BlizWiz XML 向导模板——input/entry/condition/validate/macro 元素、字符串求值、文件放置
- **父根技能（`sc2-catalog-xml`）拥有：** 项目约定、schema/语法规则、唯一真源、命名前缀
- **兄弟子技能拥有：** `sc2data-units-abilities`（直接编辑单位/技能数据）、`sc2data-effects-weapons`（直接编辑效果/武器）、`sc2data-actors-visuals`（直接编辑 Actor）

## 核心规则

1. 先加载 [`sc2-catalog-xml`](../../sc2-catalog-xml/SKILL.md)，取得项目级的 Catalog XML schema 规则；向导必须产出符合 schema 的条目。
2. 向导文件是 `.BlizWiz` XML，根为 `<wizardfile>`，内含一个或多个 `<wizard>` 元素。放在 `EditorWizards/`（用户游戏目录、地图或模组归档）；文件变更后自动重载。
3. 在 `<objecttypes create="Unit;Actor;Effect"/>` 中声明向导可以触碰的类型——未在此声明的类型，其条目不会被创建。
4. 求值字符串用 `^tokens^`：`^InputId^`、`^MacroId^`、`ENTRYINDEX`、`VALUEINDEX`、`ITEMINDEX`，目录引用 `ref=TYPE,ENTRYID,FIELDPATH`（末尾加 `#` 取数量），以及 `^BaseHP^ * 2 + 10` 这类算术。
5. 先用 Red Hat XML 校验 `.BlizWiz` 文件。向导没有 schema，所以还要对照 Data Wizard Documentation 和 `EditorWizards/` 下已知可用的文件检查结构，然后修正非法元素、属性和 token 并重新校验。被要求修向导 XML 报错时，把这条闭环跑完，不要只描述。
6. 每个向导都要在数据编辑器中实跑测试，并用 `catalogsData.xsd` 校验生成的 XML——能无错加载的向导照样可能产出非法条目。
7. `<condition>` 控制界面与数据流（input、value、match、operator、logic、嵌套条件）；`<validate type="error|confirm|warning">` 强制规则，并通过 `<text>` 向用户显示信息。
8. 重复出现的 ID 和计算用 `<macro>`，重复的字段数组用 `<count>` 配 `^VALUEINDEX^`，不要复制粘贴字面值。
9. 子对话框用内嵌向导输入 `type="Wizard:OtherWizardId"`；要编辑已有条目则用 `loadvalue` 配 `^LOADID^`。
10. 向导只是编辑器期的生成器——它们从不在游戏里运行，所以向导里的任何东西都替代不了 `CValidator`、触发器或 Galaxy 脚本。

## References

- [references/wizard-file-structure.md](references/wizard-file-structure.md) — .BlizWiz 根元素、wizard 包装层，以及文件从哪些位置加载。
- [references/wizard-elements.md](references/wizard-elements.md) — wizard、input、entry、condition、validate、macro 六类元素的属性与示例。
- [references/string-evaluation.md](references/string-evaluation.md) — token 语法、目录引用、算术与索引。
- [references/wizard-patterns-and-testing.md](references/wizard-patterns-and-testing.md) — 内嵌向导、数组、加载已有条目、宏复用，以及如何验证输出。
- [references/external-references.md](references/external-references.md) — 向导文档与示例来源。
