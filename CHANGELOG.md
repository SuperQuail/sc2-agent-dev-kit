# 更新日志

本文件记录每个版本的变更。版本号唯一真源是 `tools/sc2_version.py`。

格式参照 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号用语义化版本。

---

## [0.1.0a1] — 2026-10-07

首个开发版。面向中国用户的《星际争霸II》自定义战役开发套件，**全部文档为简体中文**。

### 新增 — 入口

- `tools/sc2.py` 统一 CLI：6 个内建动作（`start` / `rules` / `check` / `where` / `record` / `find-id`）+ 14 个转发动作
- `tools/sc2_checks.py` 静态规则注册表——**规则以数据形式存在，文档不再复述**
- `tools/sc2_records.py` 本地开发记录，存放在**仓库之外**，不进版本库、不随分发包走
- 29 个技能（10 根 + 13 Galaxy 子技能 + 6 SC2 Data 子技能），255 个 `references/` 深度资料

### 新增 — 静态规则（9 条）

| id | 规则 |
|---|---|
| `xml.ascii` | XML 注释与属性值只用 ASCII |
| `xml.banned` | 禁用 `AbilAutoCmd` / `CBehaviorBuff` 的 `InitEffect` |
| `xml.catalog-not-empty` | 每个 catalog 必须有实际条目 |
| `trigger.gui-first` | 触发器优先 GUI action |
| `galaxy.include` | include 用无扩展名相对路径 |
| `galaxy.utf8-no-bom` | Galaxy 源文件不带 BOM |
| `galaxy.line-length` | 单行不超过 2048 字符 |
| `galaxy.trigger-name` | `TriggerCreate("X")` 必须指向 `bool X(bool,bool)` |
| `guard.protected` | 不手改 `Lib*.galaxy` / `MapScript.galaxy` |

### 新增 — 发布与安装

- `tools/release.py` 分包构建：core（1.29 MB）+ data（18.71 MB）+ 清单
- `installer/` 安装器：Electron 图形界面（单文件 70.9 MB）与无界面 CLI
- 支持 4 个 harness：DeepSeek Harness、Claude Code、Codex、opencode；技能拍平为 `<skills-dir>/<name>/`
- `tools/update.py` 更新器：只增改不删除，改前备份，失败回滚
- `tools/build-updater-exe.py` 新手更新器 exe

### 新增 — 工程

- `pyproject.toml` + pytest（98 用例）+ `.github/workflows/ci.yml`
- `tools/sc2_version.py` 版本唯一真源 + `LAYOUT_REVISION`

### 变更

- `AGENT.md` 并入 `AGENTS.md`（11.4 KB → 3.4 KB）
- `wiki/` 整树迁入各技能的 `references/`，路由入口 6 个收敛为 1 个
- `SKILL.md` 从 409 KB / 4489 行瘦身至 138 KB / 1325 行，深度下沉
- 全部文档中文化（约 190 个文件）

### 移除

- `DataEditorXML/` 移出仓库，改由 release 的 `data.zip` 分发（那是暴雪的游戏数据，不是本项目代码）

### 验证

- 断链 0 ｜ skill frontmatter 29/29 ｜ 静态规则 9/9 ｜ pytest 98 用例
- 4 个 harness × 29 技能真机安装通过（未打开任何 harness）
- 数据包哈希跳过实测：二次安装 6.8 秒，166.3 MB 未重写

### 已知限制

- SC2 编辑器侧的自动化取证（`Editor accepted`）**尚未接入**，状态下机最后一环仍需人工
- 便携版 exe 的 `--smoke` 无效（NSIS 自解压壳另起子进程，argv/stdio 不透传）
