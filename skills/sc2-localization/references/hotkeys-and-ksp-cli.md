# GameHotkeys 与 KSP 本地化 CLI

这些是 `audit-gamestrings-anchors.py` 不覆盖的本地化事项。

## GameHotkeys.txt

第四个本地化文件，与 `GameStrings.txt` / `ObjectStrings.txt` / `TriggerStrings.txt` 并列。存放本地化的快捷键绑定与标签。同样是 `Key=Value` 行格式，UTF-8。保持快捷键意图，避免做出破坏预期快捷方式的语言区域改动。

## Localization Editor SC2 KSP CLI

用于人工本地化与诊断的外部工具。GitHub：<https://github.com/VoVanRusLvSC2/Localization-Editor-SC2-KSP>。CLI 安装路径见 [PR #1](https://github.com/VoVanRusLvSC2/Localization-Editor-SC2-KSP/pull/1/files)。

本技能覆盖开发所需的本地化键、锚点与文件格式诊断。整项目翻译不在本工作区范围内。KSP 是一个可选的既有桌面诊断工具。

### CLI 诊断

```powershell
sc2loc check-missing "C:\Maps\MyMap\enUS.SC2Data\LocalizedData\GameStrings.txt"
sc2loc check-missing "C:\Maps\MyMap\enUS.SC2Data\LocalizedData\GameStrings.txt" --json
```

报告以下问题类别：

- 缺失的语言区域文件（同级的 `xxXX.SC2Data` 文件夹）
- 跨语言比对下的缺失键
- 空白或类 `null` 的值
- 没有 `=` 的畸形行
- 同一文件里的重复键
- 读取错误

退出码：`0` 干净，`1` 警告/错误，`2` 用法/输入错误，`3` 意外运行时失败。

### 本地化错误检查/修复循环

编辑本地化文件时，反复跑这个循环直到干净：

1. 跑 `sc2loc check-missing <file>`（CLI 可用时）。
2. 按类别复核——缺文件、缺键、空值、畸形行、重复键。
3. 修文本文件里的根因，不要修症状。
4. 重跑 CLI，如此往复直到解决。
5. CLI 不可用时，手工检查同级语言区域文件，保持各语言的键对齐。

如果用户要求修本地化错误，要端到端执行这条工作流，而不是只描述它。

### 常见文本修复

- **补缺失的键：** `Unit/Name/MyUnit=My Unit`
- **修空值：** `DocInfo/PatchNote003=Fixed an issue where the unit icon was missing.`
- **修畸形行：** 在第一个 `=` 处切分；值里的 `=` 要保留——`Unit/Name/MyUnit My Unit` → `Unit/Name/MyUnit=My Unit`
- **删重复键：** 每个键只留一条规范条目，其余删掉。
