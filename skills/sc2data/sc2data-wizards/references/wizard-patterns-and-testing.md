# 向导模式与测试

内嵌向导、数组、加载已有条目、宏复用，以及如何验证向导的输出。

## 常用模式

- **内嵌向导**：子对话框用 `type="Wizard:OtherWizardId"`。
- **数组**：多字段用 `<count>` 与 `^VALUEINDEX^`。
- **加载已有条目**：编辑用 `loadvalue` 与 `^LOADID^`。
- **宏复用**：把公共 ID 或计算抽成宏。

测试向导的唯一可靠方法：在数据编辑器中运行它，并用 catalogsData.xsd 校验生成的 XML。
