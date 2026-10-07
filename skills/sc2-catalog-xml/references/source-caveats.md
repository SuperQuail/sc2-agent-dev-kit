# 来源注意事项与冲突

源文档之间已记录的矛盾。每一条都是「去实测数据核验」的指令，而不是让你选边站。

- **生产资源冲突：** [references/production.md](production.md) 先说 `InfoArray.Resource` 必须与单位价格一致，随后又说它是在费用之上的增量。不要盲目照抄费用——核验父级链、批量数量、原版显示与实际扣费。
- **升级结构冲突：** [references/core.md](core.md) 里有一个 `CUpgrade`/`Level`/`Effect` 的效果执行示例，别处都没用过。把它当未验证样例；采用前先查实际的 `CUpgrade` schema 与官方导出。
- **导出 override 冲突：** 文件夹里可能含合作指挥官的引用，而完整的非活动 XML 文件夹可能不存在。导出是否包含、依赖是否启用，要分开确认。
- **本地化锚点冲突：** 编辑器把面向编辑器的文本搬进 `ObjectStrings.txt`，并不能满足仍指向 `GameStrings.txt` 的运行时 `Name` 字段。每次保存后都审计锚点。
