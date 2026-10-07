# 测试套件作用域注意事项

- `test-suite.py` 接受 `--mod-dir`，否则解析配置的 `project.primary_mod`；`SC2_MODS_PATH` 与同级布局发现只提供候选根。没有配置项目名时它会直接失败，而不是猜一个 AeonOfIhanrii 目标。它把解析出的绝对目标传给每个懂模组的子工具。
- 依赖内容校验遵循 `agent-config.json`；当前项目扫描完整的本地组件依赖链，不做排除。本地化与命令卡审计仍然只在主 Mod 作用域内，因为朴素的逐依赖本地化检查无法建模继承字符串。
- 目标模组不存在时套件失败，且 `validate-mod.py` 会报告扫描的文件数。仍要确认打印出的目标就是预期的组件目录。
- 可能被依赖需求门控的命令卡槽位冲突，默认是非致命候选；聚焦命令卡交接之前，用 `audit-actor-and-card-integrity.py --strict` 检查它们。
- 不要把零退出运行说成编辑器接受或打包后运行时接受——这套件始终只是一组静态规则。
- 旧版工具在没有扫到任何模组时会静默通过。当前工具共用 `sc2_paths.py`，接受并传递 `--mod-dir`，发现不到目标时直接失败。
- 本地化工作使用既有的 `audit-gamestrings-anchors.py`；文档里的工具引用由 `check-doc-links.py` 自动检查，好让被移除的遗留命令不会悄悄回流。
- `audit-gamestrings-anchors.py --fill --mod-dir '<ActualPath>'` 会重写本地化文件。编辑器保存之后，用它恢复锚点，然后审阅 diff。
