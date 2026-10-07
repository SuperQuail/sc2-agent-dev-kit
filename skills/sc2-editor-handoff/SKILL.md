---
name: sc2-editor-handoff
description: StarCraft II Editor handoff workflow and issue lifecycle for the AeonOfIhanrii campaign. Use after code/XML changes to verify Editor acceptance, save as Components, run playtests, and track defects through the 6-stage lifecycle. Covers what the agent can and cannot do in the SC2 Editor.
---
# SC2 编辑器交接与问题生命周期

## 何时使用

在做出源码改动（XML、Galaxy、触发器）之后加载本技能，用来核验 SC2 编辑器接受、协调用户的编辑器保存，并按要求的生命周期跟踪问题。

需要登记或推进一个缺陷、为用户准备交接检查单，或判断一次静态通过能否报成「接受」时也加载。

## 加载边界

- **本技能拥有：** 六阶段问题生命周期及其证据闸门、按改动类型的编辑器交接检查单、问题账本、问题受理规则，以及智能体做不到的编辑器操作。
- **父技能拥有：** `sc2-project-entry` 拥有项目身份与路径；`AGENTS.md` 拥有全仓库工作流，以及「禁止在项目模组作为外部 override 激活时打开原版地图」这条禁令。
- **兄弟技能拥有：** `sc2-tools-validation` 拥有检查单引用的工具；`sc2-localization` 拥有锚点恢复；`sc2-bank-system` 拥有一次 Bank playtest 必须证明什么。

## 核心规则

1. **每个缺陷都按顺序走六个阶段**——`reported → root cause confirmed → source fixed → static validation passed → Editor accepted → packaged runtime passed`。
2. **没有对应证据绝不推进阶段**——`test-suite.py` 跑干净只是 `static validation passed`，仅此而已。
3. **只有用户能打开并保存暴雪地图、编辑地形、制作过场动画、另存为 Components**——智能体改为准备并交付检查单。
4. **绝不在项目模组作为外部 override 激活时打开暴雪地图**——那样加载的是改过的版本，而不是原版。
5. **`Lib*.galaxy` 与 `MapScript.galaxy` 只有编辑器保存才会重新生成**，所以用户另存为 Components 之前，脚本改动都不算完成。
6. **部署是有条件的：** `workspace_copy` 模式下先用 `--dry-run` 预览；`in_place` 模式下跳过部署，直接打开权威源。
7. **每次 catalog XML 保存之后都跑 `python tools/audit-gamestrings-anchors.py --fill`** 来恢复被剪掉的锚点。
8. **先记录受理证据：** 地图/任务、玩家/阵营配置、观察到的与预期的差异，以及确切的编辑器告警或 `bugreport.txt` 错误。
9. **账本保持精简**——长期系统事实移到主题正页；历史讨论不要留在 `issues.md` 里。

## 参考文件

- [references/issue-lifecycle.md](references/issue-lifecycle.md) — 强制的六个阶段、每个阶段需要什么，以及账本维护。
- [references/handoff-checklists.md](references/handoff-checklists.md) — Galaxy、XML/触发器、地图组件与 playtest 交接给用户的确切检查单。
- [references/intake-and-evidence.md](references/intake-and-evidence.md) — 数值/UI/运行时受理规则与每道闸门背后的证据纪律。
- [references/editor-limits-and-safety.md](references/editor-limits-and-safety.md) — 智能体做不到什么，以及编辑器安全规则。
- [references/editor-handoff.md](references/editor-handoff.md) — 原始交接检查单；[references/testing-feedback-workflow.md](references/testing-feedback-workflow.md) — 分诊与反馈流程；[references/editor-guide.md](references/editor-guide.md) — 编辑器工作流笔记。
- `sc2agent-records/<project>/issues.md` — 当前问题账本。
- `skills/sc2-bank-system/references/simulated-bank-playtests.md` — 模拟 Bank playtest 设置。
