# 项目规则与身份

`AGENTS.md` 拥有当前模组身份、编辑范围、Galaxy/XML 约束与关键路径。机器相关路径读 `agent-config.json`。详细的实现规则用对应的专业技能；不要在这里维护第二份。

## 新模组身份规则

- 新模组必须采用自己的身份与命名前缀。不要在用户模组里整片替换 `libEpi_` / `libEpi_g_` / `AeonOfIhanriiBank` / `LegacyoftheIhanrii` 字符串——那些是 AeonOfIhanrii 专属的。引用 `CCM`、`TF` 或固定部署路径的旧文档只是历史示例；当前位置从 `agent-config.json` 解析。
- 写代码前，一律从项目的 `AGENTS.md` / library XML / GUI Action Definition 确认实际的模组身份、部署路径与命名前缀。

## 同级依赖模组

不要靠枚举 `Mods/` 下的同级文件夹来推断活动依赖。从主模组出发，用 `ComponentList.SC2Components` 定位它的 info 组件，读 `<Dependencies>`，并对每个本地解析到的组件模组递归重复。光在磁盘上存在并不证明依赖是活动的。

## 来源注意事项

- **旧项目前缀冲突：** 源文档混用了 `CCM`、`TF` 与 `libEpi_` 前缀。按当前项目身份逐个判断；不要在所有用户模组上统一替换。
- **导出 override 冲突（交叉引用）：** 非活动 XML 文件夹里可能含合作指挥官的引用，而完整的非活动 XML 文件夹可能根本不存在。操作规则见 `sc2-catalog-xml`。

## 工具命令

当前命令与参数以 `tools/README.md` 和各工具的 `--help` 为准。项目路径来自 `agent-config.json`；样例数据留在本工作区的 `DataEditorXML/SC2GameDataComponents/` 下。
