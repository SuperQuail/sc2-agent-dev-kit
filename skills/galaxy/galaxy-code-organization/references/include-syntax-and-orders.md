# include 语法与真实项目的 include 顺序

`include` 行必须遵守的规则，外加从已发布项目抄来的 include 顺序，以及 `main.galaxy` 的推荐顺序。

## include 语法与规则

```galaxy
include "scripts/Utilities"          // relative to map root, no .galaxy extension
include "scripts/UI/UI-Main"         // subdirectories are supported
include "scripts/Lib/Starcode"       // third-party / library code in Lib/
```

**关键规则：**
- 路径相对于**地图根目录**（MapScript.galaxy 所在处），不是相对于发起 include 的文件。
- **不要**加 `.galaxy` 扩展名 —— 引擎会自己加。
- 顺序有影响：文件只能调用/使用在它**之前**被 include 的文件里声明的函数/类型。
- 禁止循环 include。每个文件恰好被 include 一次。
- 没有头文件守卫机制 —— 避免重复 include 同一个文件。

---

## 真实项目的 include 顺序

### SSF 完整 include 顺序（`scripts/main.galaxy`）—— 次要参考
```galaxy
include "scripts/Lib/Starcode"       // third-party lib first
include "scripts/Enums"              // pure constants
include "scripts/GlobalVariables"    // structs, global vars, typedefs
include "scripts/secrets"
include "scripts/Header"             // forward declarations
include "scripts/Utilities"          // shared helpers
include "scripts/AchievementsTemplates"
include "scripts/Achievements"
include "scripts/UI/UI-Achievements" // UI panels first (alphabetical)
include "scripts/UI/UI-CustomDefeatFrame"
include "scripts/UI/UI-HeroPanel"
include "scripts/UI/UI-Objectives"
include "scripts/UI/UI-Options"
include "scripts/UI/UI-PlayerBoard"
include "scripts/UI/UI-Popup"
include "scripts/UI/UI-Speedruns"
include "scripts/UI/UI-Stats"
include "scripts/UI/UI-Votekick"
include "scripts/UI/UI-HivePanel"
include "scripts/UI/UI-Revive"
include "scripts/UI/UI-Main"         // UI coordinator last
include "scripts/Bank"
include "scripts/Enemy"
include "scripts/PartTerran"         // content parts
include "scripts/PartProtoss"
include "scripts/PartZerg"
include "scripts/Parts"              // part coordinator last
include "scripts/Hive"
include "scripts/HeroAbilities"
include "scripts/Player"
include "scripts/Collectibles"
include "scripts/HeroSelection"
include "scripts/Tutorial"
include "scripts/Debug"
include "scripts/MapInit"            // map init always last

void main(){
    TriggerAddEventMapInit(TriggerCreate("MapInit_Main"));
}
```

### Alcyone Frontlines 的 include 顺序（`scripts/main.galaxy`）
```galaxy
include "scripts/GlobalVariables"
include "scripts/MapInit"
include "scripts/Bank"
include "scripts/Player"
include "scripts/Spawner"
include "scripts/Jungle"
include "scripts/UI"
include "scripts/Buildings"
include "scripts/AI"
include "scripts/HeroSelection"
include "scripts/HeroLevelUp"
```

---

## main.galaxy 中推荐的文件 include 顺序

```galaxy
// 1. Third-party libraries (no dependencies)
include "scripts/Lib/Starcode"

// 2. Pure constants (no dependencies)
include "scripts/Enums"

// 3. Global state (depends on Enums for array sizes)
include "scripts/GlobalVariables"

// 4. Secret/config (if any)
include "scripts/secrets"

// 5. Forward declarations (depends on GlobalVariables for types)
include "scripts/Header"

// 6. Shared utilities (may depend on GlobalVariables)
include "scripts/Utilities"

// 7. Templates/data-only files
include "scripts/AchievementsTemplates"

// 8. Feature files alphabetical (depend on headers + globals)
include "scripts/Achievements"
include "scripts/Bank"
include "scripts/Enemy"

// 9. UI files — panels first, coordinator last
include "scripts/UI/UI-HeroPanel"
include "scripts/UI/UI-PlayerBoard"
include "scripts/UI/UI-Main"

// 10. Part files — individual parts first, coordinator last
include "scripts/PartTerran"
include "scripts/PartProtoss"
include "scripts/PartZerg"
include "scripts/Parts"

// 11. High-level systems that depend on parts
include "scripts/Hive"
include "scripts/HeroAbilities"
include "scripts/Player"
include "scripts/Collectibles"
include "scripts/HeroSelection"
include "scripts/Tutorial"

// 12. Debug and init last
include "scripts/Debug"
include "scripts/MapInit"
```
