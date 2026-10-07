# 命名约定、编辑器视图设置、最佳实践

ID 命名约定、推荐的数据编辑器 View 菜单设置，以及编写时的最佳实践。

## 命名约定

| 对象 | 约定 |
|---|---|
| 单位 id | `PascalCase`，与单位类型名一致（如 `MyHeroZealot`） |
| 技能 id | `PascalCase`，通常为 `UnitNameAbilityName`（如 `AlarakPsiOrb`） |
| 按钮 id | 与技能 id 相同（如 `AlarakPsiOrb`） |
| 效果 id | 与技能 id 相同 + 效果角色后缀（如 `AlarakPsiOrbDamage`） |
| 名称字符串 | `"Unit/Name/UnitId"` 或 `"Abil/Name/AbilId"` — 由 `GameStrings.txt` 解析 |
| EditorCategories | `"Race:Terran,AbilityorEffectType:Units"` |

## 数据编辑器推荐视图设置

来自数据编辑器 **View** 菜单——这些设置能改善导航并减少常见困惑：

| 设置 | 推荐值 | 说明 |
|---|---|---|
| Toggle Easy Mode | Off | 简易模式会隐藏重要字段 |
| Display Object List As Tree | Off | 扁平列表更好搜索 |
| Show Object Explorer | On | 有用的交叉引用面板 |
| Table View | On | 带字段名搜索栏 |
| Detail View | Off | 显著拖慢编辑器 |
| Sort Field By Source | On | 把基础字段与覆写字段分组 |
| Combine Structure View | On | 关掉后部分数组字段会失效 |
| Show Field Differences | On | 高亮你的模组相对父条目覆写了什么 |
| XML Syntax Highlighting | On | 做 XML 视图工作时必需 |

## 最佳实践

- 始终把 `parent` 设为最接近的基础单位/技能——能继承就继承。
- 用 `EditorCategories` 让模组数据有组织、可筛选。
- 只覆写与父条目不同的字段。不要重复父条目的值。
- 注释掉的条目（`<!-- ... -->`）可以安全保留——它们不影响游戏性。
- 要禁用从父条目继承来的数组元素时，用 `index` 配空值或零值覆写。
- 在编辑器中测试改动时打开 **View Raw Data**，看清真实的字段 ID。
