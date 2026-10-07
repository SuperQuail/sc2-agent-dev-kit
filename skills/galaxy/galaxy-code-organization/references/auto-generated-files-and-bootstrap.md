# 自动生成的文件与引导链

编辑器拥有哪些 `.galaxy` 文件、它们里面有什么，以及地图引导如何从 `MapScript.galaxy` 走到 `main()`。

## ⚠️ 自动生成的文件 —— 绝不编辑

SC2 编辑器会自动生成某些 `.galaxy` 文件。**不要往这些文件里写代码** —— 编辑器会覆盖任何手工改动。

| 文件 | 为什么是自动生成的 |
|---|---|
| `MapScript.galaxy` | 编辑器每次保存都会重建它。它接线触发器库并调用 `InitMap()`。 |
| `LibHASH.galaxy`（例如 `Lib5A1C9904.galaxy`） | `.SC2Mod` 的库包装文件。编辑器从模组的触发器数据生成它，并通过 `include` 包装对 `scripts/main.galaxy` 的调用。 |
| `LibHASH_h.galaxy` | 库包装文件自动生成的头文件。 |

### 这些文件里有什么（仅供参考）

`MapScript.galaxy` —— 引导引擎：
```galaxy
include "TriggerLibs/NativeLib"
include "LibHASH"             // pulls in the mod library if present
void InitLibs() { libNtve_InitVariables(); libHASH_InitLib(); }
include "scripts/main"        // custom scripts (if standalone map)
void InitCustomScript() { main(); }
void InitMap() { InitLibs(); InitCustomScript(); }
```

`LibHASH.galaxy` —— 一个编辑器生成的薄包装，它：
- 用 `libNtve_InitVariables()` 作为自己的库初始化
- 只 `include` 一次 `scripts/main`
- 把公开函数暴露成 `libHASH_gf_*` 包装
- 注册一个 `LoadTriggers` 触发器，调用 `ProximaInitTriggers()`（或等价物）
- 定义 `libHASH_InitLib()` / `libHASH_InitTriggers()`

**所有真正的逻辑都在 `scripts/` 里** —— 那才是你写代码的地方。

---

## 引导链

**MapScript.galaxy** 是地图的入口 —— 它由编辑器生成，只应包含极少的代码：

```galaxy
include "TriggerLibs/NativeLib"
include "TriggerLibs/LibertyLib"

void InitLibs() { libNtve_InitLib(); libLbty_InitLib(); }

include "scripts/main"          // <-- pulls in all custom scripts

void InitCustomScript() { main(); }
void InitMap() { InitLibs(); InitCustomScript(); }
```

**scripts/main.galaxy** 是协调器 —— 它按依赖顺序列出每个 include，并定义 `main()`：

```galaxy
include "scripts/Lib/Starcode"
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

`main()` 只注册一个地图初始化触发器。所有真正的初始化都发生在 `MapInit.galaxy` 里那个触发器函数内部。
