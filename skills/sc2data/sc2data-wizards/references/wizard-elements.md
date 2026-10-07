# 向导核心元素

向导、输入、条目（entry）、条件（condition）、校验（validate）、宏（macro）六类元素，含属性与示例。

## 核心元素

### 向导元素

定义向导本身：

- `<name>`：菜单中的显示名。
- `<description>`：详细说明。
- `<category>`：菜单路径（如 "Data/Custom"）。
- `<objecttypes>`：创建/加载/查看支持的类型（如 `create="Unit;Actor"`）。
- `<instructions>`：可选的对话框说明（按页）。
- `<hidden>`：从菜单隐藏（用于内嵌向导）。

### 输入元素

对话框的用户输入项：

- `id`：唯一标识符。
- `type`：控件类型（如 `WizardText`、`WizardMenu`、`WizardRadio`，或任意目录类型如 `int32`、`CStringLink`）。
- `<name>`：标签。
- `<tooltip>`：帮助文本。
- `<default value="...">`：初始值。
- `<item>`：用于菜单/单选（含 `value` 与 `text`）。
- `<condition>`：可见性条件。
- 布局：`page`、`column` 等。

示例：

```xml
<input id="UnitName" type="string">
    <name>Unit Name</name>
    <default value="MyUnit"/>
</input>
```

### 条目元素

要创建或修改的目录条目：

- `catalog`：类型（如 "Unit"）。
- `type`：条目类型（如 "CUnit"）。
- `<id>`：条目标识符（参与求值）。
- `<parentid>`：父条目。
- `<field>`：要设置的字段（含 `<value>`、`<index>` 等）。
- `<token>`：要设置的 token。
- `<condition>`：何时处理。

示例：

```xml
<entry catalog="Unit" type="CUnit">
    <id>^UnitName^</id>
    <field id="LifeMax">
        <value>^HealthValue^</value>
    </field>
</entry>
```

### 条件元素

用于界面与数据流的布尔逻辑：

- `input`：要检查的输入。
- `value`：要求的值。
- `match`：比较方式（equal、begin、end、contain）。
- `operator`：数值运算（greater、less 等）。
- `logic`：and/or/not。
- 可嵌套条件。

示例：

```xml
<condition input="HasShield" value="1"/>
```

### 校验元素

强制规则：

- `type`：error/confirm/warning。
- `<condition>`：何时校验。
- `<text>`：提示信息。

示例：

```xml
<validate type="error">
    <condition input="UnitName" empty="0"/>
    <text>Unit name cannot be empty.</text>
</validate>
```

### 宏元素

可复用表达式：

- `id`：标识符。
- `replaceSrc/replaceDst`：文本替换。
- `truncate`：最大长度。

示例：

```xml
<macro id="UnitId">MyMod_^UnitName^</macro>
```
