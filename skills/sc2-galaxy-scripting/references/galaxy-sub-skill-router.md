# Galaxy 子技能路由

做深层 Galaxy 脚本 API 工作时，加载 `skills/galaxy/` 下对应的 `galaxy-*` 子技能。父技能 `sc2-galaxy-scripting` 保留所有子技能都遵守的项目级 **语法约束** 与 **真源** 规则。

| 任务 | 子技能 | 路径 |
|---|---|---|
| 核心语法、类型、结构体、数组、控制流 | `galaxy-language-fundamentals` | `skills/galaxy/galaxy-language-fundamentals/SKILL.md` |
| 文件结构、include 顺序、模块化布局 | `galaxy-code-organization` | `skills/galaxy/galaxy-code-organization/SKILL.md` |
| 数学、字符串、类型转换、颜色、位运算 | `galaxy-math-strings-conversion` | `skills/galaxy/galaxy-math-strings-conversion/SKILL.md` |
| 单位：创建、属性、经验/升级、编组、命令 | `galaxy-units-and-groups` | `skills/galaxy/galaxy-units-and-groups/SKILL.md` |
| 点、区域、几何、寻路 | `galaxy-points-regions-geometry` | `skills/galaxy/galaxy-points-regions-geometry/SKILL.md` |
| 玩家、同盟、种族、资源、镜头、难度 | `galaxy-players-and-alliances` | `skills/galaxy/galaxy-players-and-alliances/SKILL.md` |
| Actor 视觉：ActorSend、AttachModelToUnit、PlayAnimation | `galaxy-actor-and-visuals` | `skills/galaxy/galaxy-actor-and-visuals/SKILL.md` |
| 音效、音乐、镜头、过场、天气、光照 | `galaxy-sound-camera-environment` | `skills/galaxy/galaxy-sound-camera-environment/SKILL.md` |
| UI 对话框、XML 框体、英雄/升级面板、HUD、SSF | `galaxy-ui-and-dialogs` | `skills/galaxy/galaxy-ui-and-dialogs/SKILL.md` |
| 触发器：TriggerExecute、事件注册、异步 | `galaxy-triggers-and-functions` | `skills/galaxy/galaxy-triggers-and-functions/SKILL.md` |
| 游戏系统：Bank 存读档、刷兵器、波次、复活、科技 | `galaxy-game-systems` | `skills/galaxy/galaxy-game-systems/SKILL.md` |
| AI 行为、科技树、波次难度缩放 | `galaxy-ai-and-techtree` | `skills/galaxy/galaxy-ai-and-techtree/SKILL.md` |
| 调试、Data Table、Catalog 运行时、UserData | `galaxy-debug-data-catalog` | `skills/galaxy/galaxy-debug-data-catalog/SKILL.md` |

子技能与既有的顶层技能互补：`sc2-bank-system`（Bank schema/持久化）、`sc2-map-triggers`（触发器 XML 接线）、`sc2-attack-wave-scaling`（GUI 波次数量包裹）。子技能主题与顶层技能重叠时，顶层技能拥有项目约定，子技能拥有 API 参考。

## native 查询纪律

- 不要猜 native 名、参数或常量。按名字查 `skills/sc2-map-triggers/references/triggers-native/`；要最终 Galaxy 签名，交叉核对实际的 `natives.galaxy` 或编辑器生成的输出。
- 对单位事件，查 `EventUnit` 与真正的目标事件 API——避免「最近的单位」这类推断。普通单位创建、进入地图、训练完成与折跃完成是不同的语义；挑与需求匹配的那个，并核验去重。
