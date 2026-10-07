# 工具清单

在项目根目录运行。含空格的 Windows 路径要加引号。每个工具的 `--help` 对其当前参数是权威。

| 如果你需要…… | 跑这条命令 |
|---|---|
| 从一个 Mod 路径初始化/选择项目 | `python tools/init-project.py "<primary.SC2Mod>"` |
| 核验目标模组的改动 | `python tools/test-suite.py [--mod-dir <path>] [--primary-only]` |
| 只检查工具、文档或模组 | `python tools/test-suite.py --scope tools|docs|mod` |
| 校验 GameData XML 与 Galaxy | `python tools/validate-mod.py` |
| 查看递归模组依赖 | `python tools/inspect-mod-dependencies.py` |
| 查看单位/技能/数据链 | `python tools/sc2-catalog-query.py <cmd>` |
| 查找官方/合作组件示例或字符串 | `python tools/sc2-reference-query.py find <term> --family <Family> --component <name> --limit 20` |
| 刷新过期的 catalog 查询数据库 | `python tools/build-sc2-catalog-graph.py --sqlite-only` |
| 预览/部署模组到 SC2 安装目录 | `python tools/deploy-mod.py --dry-run` |
| 检查 GameStrings / 本地化锚点 | `python tools/audit-gamestrings-anchors.py --fill` |
| 审计命令卡 | `python tools/audit-actor-and-card-integrity.py` |
| 提取 playtest 错误与日志 | `python tools/extract-playtest-bugreport.py` |
| 检查文档链接 | `python tools/check-doc-links.py` |
| 审计问题生命周期的标签/证据 | `python tools/audit-issue-lifecycle.py` |
| 列出全部工作区动作/规则 | `python tools/sc2.py` / `python tools/sc2.py rules` |

## 工作区入口

- `python tools/sc2.py` — 列出所有动作。
- `python tools/sc2.py rules` — 静态规则的唯一真源。不要在别处复述它们。
- `python tools/sc2.py check` — 对当前项目强制这些规则。
- `python tools/sc2.py where` — 按当前 `source_mode` 打印该改哪个文件。
- `python tools/sc2.py start` — 开工前读取本项目之前的任务记录。
- `python tools/sc2.py issues` — 校验问题账本。

## 定向静态检查（PowerShell）

```powershell
python tools/test-suite.py --mod-dir "$env:SC2_MODS_PATH\AeonOfIhanrii.SC2Mod"
python tools/validate-mod.py --mod-dir "$env:SC2_MODS_PATH\AeonOfIhanrii.SC2Mod"
python tools/audit-actor-and-card-integrity.py --mod-dir "$env:SC2_MODS_PATH\AeonOfIhanrii.SC2Mod"
python tools/audit-gamestrings-anchors.py --mod-dir "$env:SC2_MODS_PATH\AeonOfIhanrii.SC2Mod"
python tools/check-doc-links.py
```

把示例路径换成实际目标。完整工具指南在 `tools/README.md`。
