# AGENTS.md — 工作区入口

**作者：扯蛋虾米。** 统一作者声明见 [AUTHORS.md](AUTHORS.md)。

本工作区用于 **StarCraft II 自定义战役开发**：模组、地图、数据调查、编辑器交接。

---

## 先跑工具，再读文档

本工作区的规则**已经做成工具**。不要凭记忆写 XML 或 Galaxy——跑检查器。

````bash
python tools/sc2.py          # 列出全部动作
python tools/sc2.py rules    # 列出全部静态规则（唯一真源）
python tools/sc2.py check    # 对当前项目强制这些规则
python tools/sc2.py where    # 打印按 source_mode 该改哪个文件
python tools/sc2.py --version              # 版本（唯一真源 tools/sc2_version.py）
python -m pytest tools/tests -q            # 工具单测（pytest）
python tools/update.py --check <发行包>     # 与发行清单比对，看有哪些更新
````

`sc2.py rules` 是规则的**唯一真源**，本文档不复述它们。
**没列在 `rules` 里的约束＝还没做成工具**——那就去对应技能的 `references/` 查。

## 三条硬红线

工具会拦，但先说清：

1. **不编辑 `publish/`** —— 发布产物
2. **不手改 `Lib*.galaxy` / `MapScript.galaxy`** —— 编译产物，编辑器保存时会覆盖
3. **不自造字段名** —— 写 XML 前查 catalog（`sc2.py query`）

## 任务路由

领域知识**按需加载**，不要预先读。需要时打开对应技能的 `SKILL.md`：

| 任务 | 技能 |
|---|---|
| 入口 / 模组身份 / 通用流程 | `skills/sc2-project-entry/` |
| GameData XML（单位/技能/行为/效果/武器） | `skills/sc2-catalog-xml/` |
| Galaxy 脚本 | `skills/sc2-galaxy-scripting/` |
| Actor 系统 | `skills/sc2-actor-system/` |
| 地图触发器 | `skills/sc2-map-triggers/` |
| Bank 存档 / 战役持久化 | `skills/sc2-bank-system/` |
| 本地化字符串 | `skills/sc2-localization/` |
| 工具与静态校验 | `skills/sc2-tools-validation/` |
| 编辑器交接 / 问题生命周期 | `skills/sc2-editor-handoff/` |
| 攻击波缩放 | `skills/sc2-attack-wave-scaling/` |
| Galaxy 语言细节（13 个子技能） | `skills/galaxy/` |
| SC2 Data 细节（6 个子技能） | `skills/sc2data/` |

每个技能的深度资料在其 `references/` 下，**只在需要时读**。

## 项目状态

当前尚未选择项目（工作区没有活动的 `agent-config.json`）。用户给出主 `.SC2Mod` Components 路径后：

````bash
python tools/sc2.py init "<主Mod文件夹路径>"
````

不要要求用户手工编辑 JSON，也不要根据 Mods 目录猜测依赖。初始化后核对模组身份与所选项目是否相符。

## 问题工作流

````text
reported -> root cause confirmed -> source fixed
         -> static validation passed -> Editor accepted -> packaged runtime passed
````

**禁止越级声明。** 静态工具全绿**只能**报 `static validation passed`；
没有编辑器重载/保存证据不得写 `Editor accepted`，没有游戏内复现不得写 `packaged runtime passed`。

账本校验：`python tools/sc2.py issues`

## 内容放哪

| 内容 | 位置 |
|---|---|
| 怎么做（可复用知识） | 对应技能的 `references/` |
| 可复用规则 | `tools/sc2_checks.py` 的 `RULES` |
| 设计决策 | `DesignDocument.md` |
| 工作区状态（当前工作、状态、问题账本、决策日志） | `workspace/` |
| 历史 | git |

## 版本与更新

版本号只有一处真源：`tools/sc2_version.py` 的 `VERSION`。打包器把它写进归档名、归档内的 `VERSION`
与发行清单；更新器用它和本机已装版本比对。**不要在别处再写一份版本号。**

````bash
python tools/package.py --write            # 打包；默认预演，写出后自动校验并解压试运行
python tools/update.py --check <包>         # 只报告差异（默认行为）
python tools/update.py --apply <包>         # 写入更新：先校验全部哈希，改前备份，失败回滚
python tools/build-updater-exe.py          # 给新手构建单文件更新器 exe
````

更新器**只增改、从不删除**，也不碰 `agent-config.json` 等同机专属文件；备份留在
`.sc2-update-backup/`，用 `update.py --list-backups` 查看。

## 需要人工的编辑器操作

以下智能体做不了，需要用户操作：打开/保存 Blizzard 地图（需账号登录）、地形与区域、doodads 与寻路、过场动画、把模组/地图另存为 Components。

> 注意：**绝不**在项目模组作为外部 override 激活时打开地图——会加载模组版本而非原版。
