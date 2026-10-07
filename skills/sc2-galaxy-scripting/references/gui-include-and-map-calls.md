# GUI include 注入模式

`Lib67AA1763.galaxy` 是自动生成的，保存时会覆盖手写的 `include` 语句。要持久地 include `Epi_Main.galaxy`：

1. 在 SC2 编辑器中打开 `AeonOfIhanrii.SC2Mod` → Triggers（F6）。
2. 选中 `LegacyoftheIhanrii` 库。
3. 新建一个 Custom Script（Ctrl+Alt+T），命名为 `Epi_Include`。
4. 加入单行：`include "Epi_Main"`。
5. 保存模组（Ctrl+S）——编辑器会把 `include "Epi_Include"` 注入到 `Lib67AA1763.galaxy` 中 `include "Lib67AA1763_h"` 之后。

不要 把 `Epi_Main.galaxy` 的内容直接粘进 GUI Custom Script 块——它含非 ASCII 注释，而 GUI 块强制 ASCII 且要求 4 空格行前缀。（库名与文件名要按当前项目身份调整。）

## 地图调用模组必须走 GUI action

除非地图调用的是模组里定义的 **GUI action**，否则 SC2 链接器会静默丢掉模组库。地图里的 Custom Script 块不算数。在模组库里定义 GUI action，再从地图触发器调用它们。
