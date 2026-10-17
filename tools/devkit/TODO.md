# 待办（TODO）

> 只记**还没做**的。做完就删掉这一条，历史在 git 里。

## T1. 任务简报界面的单位（未解决）

**目标**：把「任务选择 / 任务简报」界面右侧面板里显示的陆战队员，换成萨尔那加狂热者。

**已知**：
- 那一屏是**战役侧**的界面，不是地图里的触发器。证据：那一屏的文字（任务名、面板标题、说明）**全部正常中文**，而地图/mod 的文本当时是坏的 —— 说明它读的是战役数据。
- 地图里的 `Briefing Scene 00/01` 触发器作用是 `MovieStartRecording("Briefing_Video_TRaynor01_00")` —— **把简报录制成影片**。玩家看到的是 CASC 里已渲染的 `briefing_video_traynor01_00/01/02.ogv`。
- 那 8 个陆战队员已在地图两处换掉，**对那一屏无影响**。

**下一步**：从参考包（`SC2MR8.0`、`帝国之翼+`、`黄金之遗`）里找它们怎么改那一屏的「可用单位/科技」面板。

## T2. 下一阶段（调研见 BENCHMARK `10）

| # | 做什么 | 为什么现在能做 |
|---|---|---|
| 1 | `remap`：多对多批量映射替换（`set-unit` 的复数版，触发器 + galaxy 两边） | 任务 1a 只差这一件 |
| 2 | `catalog get`：**读父链的有效值** | 任务 1c「血量 ×2」的前提 |
| 3 | `catalog set`：写 `CUnit` 覆盖（含「数组索引不能省」的检查） | 任务 1c 的落点 |

**数据可得性（省得再查）**：
- 蒙斯克单位在 `Mods\StarCoop\Commanders\ArcturusMengsk.SC2Mod`，**本地 CASC 里就有**，64 个单位，不随包分发
- 别被 `alliedcommanders.sc2mod`（纯文本壳，无 GameData）和 `liberty.sc2mod` 里的 `*ACGluescreenDummy`（UI 假单位）骗

## T3. sc2cat 的已知缺口

| # | 缺口 | 现在只能怎么绕 |
|---|---|---|
| 1 | 从 CASC 直接取地图（要外部 `casc.exe`） | 手工两步 |
| 2 | `--in-trigger` 给 Id 时推不出 galaxy 函数名（图没有 TriggerStrings） | 补 `--galaxy-fn` |
| 3 | `doctor` 查不出「意图」：两边一致但不是你要的效果 | 任务书里把验收标准写死、可数 |
| 4 | `find-unit` 有「(无归属)」的引用 | grep 行号 + 手数触发器的元素边界 |
| 5 | 组件的 `.version` 清单不会生成，只能删 | 删掉，靠游戏容忍 |

## T4. 已知但按下不表的

- `normalize` 补齐的 12 个语系是**内容复制**，不是真翻译。要出多语言得逐份替换 —— 代码里已用 `shortcut:` 标注。
- `package` 用平台自带的 `tar` 归档，**没在 Linux/macOS 上测过**。
- `inject` 只按「`runActions` 守卫之后」一个位置插；某函数结构不标准时会插错位置且**不报错**。
