# 贴图声明与继承的贴图切换

模型上的 `TextureDeclares` 把贴图前缀和文件名适配映射到贴图槽位；它们本身不会发起切换。`TextureSelectById`、`TextureSelectByMatch`、`TextureSelectBySlot` 这类 Actor 消息才执行选择。因此，当一个复制来的模型或皮肤保留了这些声明，而继承的 Actor 事件又通过 `DarkProtoss` 或 `PurifierProtoss` 这类战役升级去命中它们时，模型看起来就会坏掉。

删掉声明可以让那个不想要的选中不再解析，但同时也会禁掉任何需要该槽位的正常动态贴图切换。变体仍然需要动态贴图选择时，优先删除或覆盖继承的 Actor 事件，或者阻止其启用升级。只有模型确实自成一体、且没有任何保留的 Actor 事件会切换它的贴图时，才省略这些声明。
