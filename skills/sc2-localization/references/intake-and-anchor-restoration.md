# 问题受理与自动恢复

## 问题受理规则

改 UI 文本之前，先确定确切的 UI 面及其生效的本地化锚点：

- **世界悬停** — 在游戏世界里悬停对象时显示的文本
- **选择面板** — 单位/建筑选择面板里的文本
- **命令卡按钮** — `Button/Name/ID` 与 `Button/Tooltip/ID`
- **提示框** — `Unit/Tooltip/ID`、`Upgrade/Name/ID` 等
- **编辑器文本** — `ObjectStrings.txt` 里的 `Type/Name/ID` 与 `Type/EditorDescription/ID`

改 `GameStrings.txt` 或 `ObjectStrings.txt` 之前，先记录生效的锚点。

## 自动字符串恢复

SC2 编辑器保存时会规范化条目，并把面向编辑器的文本迁移进 `ObjectStrings.txt`。为确保所有被引用的玩家可见字符串仍锚定在 `GameStrings.txt` 里，跑：

```powershell
python tools/audit-gamestrings-anchors.py --fill
```

它会扫描所有 `GameData/*.xml`，对照 `GameStrings.txt` 检查，并从 `ObjectStrings.txt` 自动补齐缺失的键。每次触及 catalog XML 的编辑器保存之后都要跑它。

## 硬规则

- **数据编辑器显示空白/错误文本时，改本地化文件，不要改原始数据 XML。** 面向玩家的文本属于 `GameStrings.txt`；编辑器显示文本属于 `ObjectStrings.txt`。
- **每个新 catalog `id` 都必须在同一次改动里拿到 `ObjectStrings.txt` 条目**——`Type/Name/ID` 与 `Type/EditorPrefix/ID` 都要。否则该对象在数据编辑器里显示为空白。
- **面向玩家的文本需要显式 XML 锚点**（`<Description value="Unit/Tooltip/..."/>`、`<Name value="Button/Name/..."/>` 等）。缺锚点会让编辑器在保存时剪掉本地化文本。
- **每次触及 catalog XML 的编辑器保存之后都跑 `audit-gamestrings-anchors.py --fill`**，恢复被剪掉的锚点。
