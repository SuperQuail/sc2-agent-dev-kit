# 本地化文件总览

任务触及 `<ModName>.SC2Mod/enUS.SC2Data/LocalizedData/` 下三个本地化文件中的任何一个时加载本技能：

- `GameStrings.txt` — 游戏内可见文本
- `ObjectStrings.txt` — 数据编辑器显示文本
- `TriggerStrings.txt` — 触发器编辑器显示名

数据编辑器显示空白名、错误前缀或描述缺失时也加载，以及修已上报的 UI 文本问题时。

| 文件 | 用途 | 键格式 |
|---|---|---|
| `GameStrings.txt` | 游戏内可见文本：按钮名、提示框、展示给玩家的升级名 | `Button/Name/ID=Text`<br>`Button/Tooltip/ID=Text`<br>`Unit/Tooltip/ID=Text`<br>`Upgrade/Name/ID=Text` |
| `ObjectStrings.txt` | 数据编辑器显示文本：所有数据对象的编辑器名、前缀、描述 | `Type/Name/ID=Name`<br>`Type/EditorPrefix/ID=Prefix`<br>`Type/EditorDescription/ID=Desc` |
| `TriggerStrings.txt` | Galaxy 函数与触发器的触发器编辑器显示名 | `FunctionDef/Name/ID=Name`<br>`Trigger/Name/ID=Name` |

## ObjectStrings.txt 覆盖的类型

`Abil`、`Actor`、`Behavior`、`Button`、`Effect`、`Model`、`Mover`、`Requirement`、`RequirementNode`、`Sound`、`Turret`、`Unit`、`Upgrade`、`Validator`、`Weapon`。

## 新增 XML 条目的规则

每当你新建或覆盖一个应当在 SC2 数据编辑器里可读的 XML catalog 对象，都要在同一次改动里更新 `ObjectStrings.txt`。为 `AbilData.xml`、`UnitData.xml`、`ButtonData.xml`、`UpgradeData.xml`、`RequirementData.xml`、`RequirementNodeData.xml` 等 catalog 里每个新 `id` 同时加上 `Type/EditorPrefix/ID=...` 与 `Type/Name/ID=...`。

`GameData.xml` 里面向玩家的文本，一律加上显式 XML 锚点：

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

## ObjectStrings 覆盖范围

新增一个可读的 catalog 对象时，要为每个相关类型同步 `ObjectStrings.txt` 的名字与项目前缀——Actor、Model、Sound、Turret、Validator 以及配套 Weapon——而不只是显而易见的 Unit/Ability/Button/Effect。

## 锚点消费注意事项

`Name`、`Tooltip`、`Description` 字段的消费方式因对象而异——先对照字段与当前依赖确认某个 catalog 字段读的是哪个 `GameStrings` 键。
