---
name: sc2-tools-validation
description: StarCraft II mod development tooling for the active project. Use when running pre-flight tests, XML schema validation, Galaxy syntax checks, catalog queries, mod deployment, or playtest bug extraction. Covers all tools/ CLI commands and when to use each.
---
# SC2 工具与校验

## 何时使用

在运行 `tools/` 目录下任何工具时加载本技能：预检测试、XML schema 校验、Galaxy 语法检查、catalog 查询、模组部署、playtest 日志提取，或问题账本审计。

命令表见 [references/tool-inventory.md](references/tool-inventory.md)；权威参数以 `tools/README.md` 与各工具的 `--help` 为准。

## 加载边界

- **本技能拥有：** 每个 `tools/` CLI 的用途、何时运行、作用域与安全注意事项，以及退出码的含义。
- **父技能拥有：** `sc2-project-entry` 拥有项目身份与路径解析；`AGENTS.md` 拥有全仓库规则；`python tools/sc2.py rules` 是静态规则的唯一真源，本文不复述。
- **兄弟技能拥有：** `sc2-catalog-xml`、`sc2-galaxy-scripting`、`sc2-map-triggers`、`sc2-actor-system`、`sc2-localization`、`sc2-bank-system` 拥有这些工具所检查的编写规则；`sc2-editor-handoff` 拥有问题生命周期与编辑器打包步骤。

## 核心规则

1. **写 XML 或 Galaxy 之前先跑工具**——`python tools/sc2.py rules` 是唯一真源；没列在那里的约束在对应主题技能的 `references/` 里。
2. **核对打印出的目标**——`test-suite.py` 与 `validate-mod.py` 解析配置的主模组，找不到时直接失败；`SC2_MODS_PATH` 与同级目录发现只提供候选根。
3. **零退出的静态运行只能意味着 `static validation passed`**——永远不等于编辑器接受，也不等于打包后运行时验证。
4. **查询 catalog 图，不要读原始转储**——用 `python tools/sc2-catalog-query.py`；绝不通读 `DataEditorXML/`。
5. **只在图缺失或过时时刷新**（`python tools/build-sc2-catalog-graph.py --sqlite-only`），刷新前先确认其输入配置覆盖了目标模组。
6. **不要用 `--source-class local_mod` 过滤基础对象查询**——它会把基础依赖条目藏掉。
7. **部署不是校验：** 不带 `--dry-run` 的 `deploy-mod.py` 会复制文件；在 `in_place` 模式下它是空操作，配置的 Mod 本身就是权威源。
8. **依赖缺失或损坏会中止当前数据索引；** `--allow-incomplete-dependencies` 与 `--allow-stale` 只允许带标记的只读查看，绝不授权写入或有效值结论。
9. **编辑器保存后跑 `python tools/audit-gamestrings-anchors.py --fill`** 并审查 diff——它会重写本地化文件。
10. **没有证据绝不推进问题阶段**——套件全绿跨不过编辑器闸门；用 `python tools/sc2.py issues` 校验账本。

## 参考文件

- [references/tool-inventory.md](references/tool-inventory.md) — 每个工具、选型决策表，以及工作区 `sc2.py` 入口。
- [references/validation-pipeline.md](references/validation-pipeline.md) — `init-project.py`、`test-suite.py`、`validate-mod.py`、各审计器与 XSD schema 实际检查什么。
- [references/catalog-graph-and-querying.md](references/catalog-graph-and-querying.md) — catalog 查询命令、图重建、数据库过期与有界参考查询。
- [references/test-suite-scope-caveats.md](references/test-suite-scope-caveats.md) — 依赖扫描、仅主模组审计与非致命冲突为何如此表现。
- [references/deployment-and-dependencies.md](references/deployment-and-dependencies.md) — `deploy-mod.py` 模式、依赖根解析与部分作用域权限。
- [references/playtest-logs-and-recovery.md](references/playtest-logs-and-recovery.md) — `bugreport.txt` 提取、`os error 206` 恢复与问题反馈分层。
- `skills/sc2-project-entry/references/agent-context-efficiency.md` — 保持工具输出与上下文精简。
- `skills/sc2-project-entry/references/multi-pc-setup.md` — 多机设置与 IDE 配置。
