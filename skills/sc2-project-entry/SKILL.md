---
name: sc2-project-entry
description: StarCraft II mod and campaign implementation entry point. Use for SC2Mod/SC2Map, catalog XML, Galaxy, triggers, actors, localization, or Editor handoff; route to the relevant specialist skill.
---
# SC2 项目入口（统一路由）

## 何时使用

做 SC2 模组或战役实现时加载本技能。涉及项目路径时读 `agent-config.json`，仓库规则读 `AGENTS.md`。纯文档或纯工具维护，直接用对应文件即可。

用户给出主 Components `.SC2Mod` 文件夹路径并要求初始化或选择项目时，直接跑 `python tools/init-project.py "<path>"`——绝不要让用户去编辑 `agent-config.json` 或手工列举依赖。

## 加载边界

- **本技能拥有：** 任务路由表、项目身份与命名规则、实现流程（有效值 → 查询 → 项目唯一 ID → 组件源 → 静态/运行时 → 决策），以及必须用编辑器 GUI 的操作清单。
- **父技能拥有：** `AGENTS.md` 拥有当前模组身份、编辑范围、Galaxy/XML 约束、关键路径与三条红线；`agent-config.json` 拥有机器相关路径。
- **兄弟技能拥有：** 每个专业技能的实现规则由它自己拥有——不要在这里维护第二份。`sc2-tools-validation` 拥有工具 CLI，`sc2-editor-handoff` 拥有问题生命周期。

## 核心规则

1. **先路由再阅读：** 从 [references/task-routing.md](references/task-routing.md) 挑专业技能；绝不预加载领域知识。
2. **绝不猜身份**——Bank 名、Library ID、脚本块和代码前缀都来自证据（`AGENTS.md`、library XML、GUI Action Definition），绝不来自上一个项目的文档。
3. **新模组用自己的前缀；** 不要整片替换别的项目的 `libEpi_` / Bank 名字符串。
4. **编辑前先证明有效值**——记录对象 ID、父级、精确字段/数组索引、当前依赖的值，以及打算做的覆盖。本地缺字段不等于零。
5. **落笔前查询 catalog 图，** 之后只看相关的原始 XML，并把这份证据复用到本任务剩余部分。
6. **不要把 catalog 快照当成活动依赖**——通过依赖链确认其启用状态。
7. **变体要做成项目唯一 ID；** 只有在用户明确要求全局改动时才覆盖同名原版 ID。
8. **只改组件源**——绝不改 `publish/`、生成的 `MapScript.galaxy` 或编译产物 `Lib*.galaxy`。
9. **先静态后运行时：** 静态检查通过只是 `static validation passed`，永远不等于编辑器接受或打包后运行时通过。
10. **记录长期决策**到 `DesignDocument.md` 和对应技能的 `references/` 页面；长期系统事实不要留在冗长的历史日志里。

## 参考文件

- [references/task-routing.md](references/task-routing.md) — 任务→技能表与专业路由；先读它来挑对技能。
- [references/project-rules-and-identity.md](references/project-rules-and-identity.md) — 身份规则、同级依赖发现与前缀注意事项。
- [references/implementation-workflow.md](references/implementation-workflow.md) — 六步流程，以及你绝不能通读的大型查询源。
- [references/editor-limits-and-initialization.md](references/editor-limits-and-initialization.md) — 项目初始化，以及只有用户能在编辑器里执行的操作。
- [references/project-initialization.md](references/project-initialization.md) — 完整初始化参考；[references/multi-pc-setup.md](references/multi-pc-setup.md) — 多机与 IDE 配置。
- [references/agent-context-efficiency.md](references/agent-context-efficiency.md) — 保持上下文与工具输出精简；[references/external-paths.md](references/external-paths.md) 与 [references/external-sc2-resources.md](references/external-sc2-resources.md) — 路径与外部资源。
- [references/agents/domain.md](references/agents/domain.md)、[references/agents/issue-tracker.md](references/agents/issue-tracker.md)、[references/agents/research.md](references/agents/research.md) — 各智能体角色说明。
