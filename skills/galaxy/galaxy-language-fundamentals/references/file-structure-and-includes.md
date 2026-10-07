# 文件结构与 include

代码如何拆到多个 `.galaxy` 文件，以及地图、库模组、战役任务各自适用哪种 include 模式。

## 文件结构与 include

代码用 `include` 拆到多个 `.galaxy` 文件。入口是 `MapScript.galaxy`，它把控制权交给 `scripts/main.galaxy` 协调器：

> **⚠️ 自动生成的文件 —— 绝不往里写代码：**
> - `MapScript.galaxy` —— 每次保存都由编辑器重建。
> - `LibHASH.galaxy`（例如 `Lib5A1C9904.galaxy`）—— 编辑器为 `.SC2Mod` 生成的库包装文件；它通过 `include` 包装对 `scripts/` 的调用。
> - `LibHASH_h.galaxy` —— 自动生成的库头文件。
>
> **所有自定义逻辑都属于 `scripts/` 下的文件。** 编辑器会覆盖自动生成的文件。

```galaxy
// MapScript.galaxy (editor-managed — do not add logic here)
include "TriggerLibs/NativeLib"
include "TriggerLibs/LibertyLib"
void InitLibs() { libNtve_InitLib(); libLbty_InitLib(); }
include "scripts/main"
void InitCustomScript() { main(); }
void InitMap() { InitLibs(); InitCustomScript(); }
```

```galaxy
// scripts/main.galaxy — coordinator, lists all includes and defines main()
include "scripts/Enums"
include "scripts/GlobalVariables"
include "scripts/Header"
include "scripts/Utilities"
// ... all other files ...
include "scripts/MapInit"

void main() {
    TriggerAddEventMapInit(TriggerCreate("MapInit_Main"));
}
```

- `include` 中的路径**相对于地图根目录**，不带 `.galaxy` 扩展名。
- 顺序有影响：文件只能使用更早 include 中声明的类型/函数。
- `Header.galaxy` 放前置声明，让文件能调用 include 链中更靠后才定义的函数。
- 完整的文件拆分模式见 `galaxy-code-organization` 技能。

### 战役地图模式（WoL / HotS / LotV 剧情任务）

战役地图 include 三个引擎库 —— NativeLib + LibertyLib + CampaignLib：
```galaxy
include "TriggerLibs/NativeLib"
include "TriggerLibs/LibertyLib"
include "TriggerLibs/CampaignLib"

void InitLibs() {
    libNtve_InitLib();
    libLbty_InitLib();
    libCamp_InitLib();
}
```
这可以使用 `libNtve_*`（native 辅助）、`libLbty_*`（Liberty/WoL 辅助）和 `libCamp_*`（战役专用 API：传输、目标、剧情状态、空投舱等）。

### 库模组模式（备选）

在 `.SC2Mod` 库内工作时，文件使用带哈希前缀的 header/impl 拆分模式：
```galaxy
include "TriggerLibs/NativeLib"
include "LibXXXXXXXX_h"  // header: structs, globals, forward decls (replace XXXXXXXX with your mod's hash)
```

### 测试地图 / 简单地图模式（备选）

有些小地图用不带路径、只写库名的 include：
```galaxy
include "LibHASH"   // no path, no extension — editor resolves from linked dependencies
```
