# 本地化文件

## 本地化文件总览

| 文件 | 用途 | 键格式 |
|---|---|---|
| `GameStrings.txt` | 游戏内可见文本：按钮名、提示框、展示给玩家的升级名 | `Button/Name/ID=Text`<br>`Button/Tooltip/ID=Text`<br>`Unit/Tooltip/ID=Text`<br>`Upgrade/Name/ID=Text` |
| `ObjectStrings.txt` | 数据编辑器显示文本：所有数据对象的编辑器名、前缀、描述 | `Type/Name/ID=Name`<br>`Type/EditorPrefix/ID=Prefix`<br>`Type/EditorDescription/ID=Desc` |
| `TriggerStrings.txt` | Galaxy 函数与触发器的触发器编辑器显示名 | `FunctionDef/Name/ID=Name`<br>`Trigger/Name/ID=Name` |

---

## ObjectStrings.txt 覆盖的类型

`Abil`、`Actor`、`Behavior`、`Button`、`Effect`、`Model`、`Mover`、`Requirement`、`RequirementNode`、`Sound`、`Turret`、`Unit`、`Upgrade`、`Validator`、`Weapon`

---

## 新增 XML 条目的规则

每当你新建或覆盖一个应当在 SC2 数据编辑器里可读的 XML catalog 对象，都要在同一次改动里更新 `ObjectStrings.txt`。为 `AbilData.xml`、`UnitData.xml`、`ButtonData.xml`、`UpgradeData.xml`、`RequirementData.xml`、`RequirementNodeData.xml` 等 catalog 里每个新 `id` 同时加上 `Type/EditorPrefix/ID=...` 与 `Type/Name/ID=...`。

`GameData.xml` 里面向玩家的文本，一律加上显式 XML 文本锚点：

```xml
<!-- UnitData.xml -->
<CUnit id="MyUnit">
    <Description value="Unit/Tooltip/MyUnit"/>
</CUnit>
```

```text
# GameStrings.txt
Unit/Tooltip/MyUnit=Player-facing unit description here.
```

没有显式 XML 锚点，SC2 编辑器在保存过程中可能剪掉或无法保留面向玩家的本地化文本。

---

## 自动字符串恢复

`audit-gamestrings-anchors.py` 接受 `--locale`（默认 `enUS`）。Niadra 有 `zhCN` 本地化；用 `python tools/audit-gamestrings-anchors.py --mod-dir "../StarCraft II/Mods/Niadra.SC2Mod" --locale zhCN`，编辑器保存后再加上 `--fill`。不要仅仅为了满足默认审计就建一个英文目录。Niadra 当前的迁移状态与既有的锚点遗漏记录在命名契约里。

SC2 编辑器保存时会规范化条目，并把面向编辑器的文本迁移进 `ObjectStrings.txt`。为确保所有被引用的玩家可见字符串仍锚定在 `GameStrings.txt` 里，跑：

```powershell
python tools/audit-gamestrings-anchors.py --fill
```

它会扫描所有 `GameData/*.xml`，对照 `GameStrings.txt` 检查，并从 `ObjectStrings.txt` 自动补齐缺失的键。
