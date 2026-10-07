# 向导文件结构

.BlizWiz 根元素、向导包装层，以及向导文件从哪些位置加载。

> **文件放置：** 向导使用 `.BlizWiz` 扩展名。放在用户游戏目录的 `EditorWizards/` 文件夹，或放进地图/模组归档。文件变更后自动重载。


文件：`MyWizard.BliZWiz`

根元素是 `<wizardfile>`，内含一个或多个 `<wizard>` 元素：

```xml
<?xml version="1.0" encoding="utf-8"?>
<wizardfile>
    <wizard id="MyWizard">
        <name>My Custom Wizard</name>
        <description>Creates a unit with associated actor and effect.</description>
        <category>Data/Custom</category>
        <objecttypes create="Unit;Actor;Effect"/>
        <!-- inputs, entries, conditions, etc. -->
    </wizard>
</wizardfile>
```
