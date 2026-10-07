# 任务路由

读完本技能后，按任务路由到合适的深层技能。

| 任务类型 | 接着加载的技能 | 要读的参考 |
|---|---|---|
| 编写/编辑 Galaxy 脚本 | `sc2-galaxy-scripting` | `skills/sc2-galaxy-scripting/references/galaxy-gotchas.md`、`skills/galaxy/galaxy-language-fundamentals/references/galaxy-language.md` |
| 编写/编辑模组 XML（单位、技能、行为、效果、武器、升级、验证器、足迹） | `sc2-catalog-xml` | `skills/sc2-catalog-xml/references/xml-patterns.md`、`skills/sc2-catalog-xml/references/catalog-rules.md` |
| 深层 Actor 工作（CActorUnit/Action/Model/Beam/Sound、VFX/音频、贴图切换、变形过渡、非活动 XML 提取） | `sc2-actor-system` | `skills/sc2-actor-system/references/actors.md`、`skills/sc2-actor-system/references/editor-roundtrip-and-actors.md` |
| 编辑地图触发器、GUI action 接线、胜利/失败钩子 | `sc2-map-triggers` | `skills/sc2-map-triggers/references/triggers-overview.md`、`skills/sc2-map-triggers/references/per-map-setup.md` |
| Bank 系统、战役持久化、任务存档、解锁 | `sc2-bank-system` | `skills/sc2-bank-system/references/bank-system.md`、`skills/sc2-bank-system/references/galaxy-bank.md` |
| 本地化（GameStrings/ObjectStrings/TriggerStrings）、字符串锚点、编辑器名空白、GameHotkeys | `sc2-localization` | `skills/sc2-localization/references/localization.md` |
| 跑工具、校验、部署 | `sc2-tools-validation` | `tools/README.md` |
| 编辑器交接、问题生命周期、playtest | `sc2-editor-handoff` | `skills/sc2-editor-handoff/references/editor-handoff.md`、`skills/sc2-editor-handoff/references/testing-feedback-workflow.md` |
| 攻击波缩放、AI 性格波次迁移 | `sc2-attack-wave-scaling` | `skills/sc2-attack-wave-scaling/references/ai-personality-to-gui-triggers.md` |

## 专业路由

`skills/galaxy/` 与 `skills/sc2data/` 下的子技能，只在加载对应顶层路由技能之后才加载。加载 `sc2-galaxy-scripting` 或 `sc2-catalog-xml` 之后，用该技能自己的子技能表找具体的 Galaxy API 或 catalog 族。

新触发器请求一律路由到 `sc2-map-triggers`，并优先用可编辑的 GUI 事件、条件与动作。只有当 GUI 无法合理表达一小部分时才内嵌 Custom Script；只有用户明确要求、或任务是在维护既有 Galaxy 源时，才选独立 Galaxy。
