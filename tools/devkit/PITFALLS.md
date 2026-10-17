# 开发坑记录（PITFALLS）

> 目的：**这些不是给用户看的文档，是给套件开发看的**。每一条都要能落成一项工具改进。
> 记法：现象 → 根因 → 怎么发现的 → 套件该怎么改。

## A. 构建与网络

### A1. `cargo test` 不会更新二进制

- 现象：改了代码、`cargo test` 通过，跑 `target/release/x.exe` 却还是旧行为。
- 根因：`cargo test` 编的是测试二进制；`cargo build --release` 才更新 exe。
- 怎么发现的：修了自闭合元素 bug 后统计数一动不动，差点去查一个不存在的问题。
- 改进：验收一律 `cargo build --release` 之后再跑 exe，不拿 `cargo test` 通过当验收。

### A2. crate 下载会被静默截断

- 现象：`cargo build` 报 `failed to unpack ... unexpected end of file`。
- 根因：传输中途断，落盘的 `.crate` 是半个文件。**gzip 校验可能照样通过**，但解包必失败。
- 怎么发现的：`webview2-com-sys`、`window-vibrancy` 各踩一次。
- 改进：批量补包必须按索引里的 cksum 逐个比 sha256，不能只看文件大小或 gzip 能不能读。

### A3. cargo 与 pnpm 对代理的偏好相反

- cargo 要挂代理（不挂 rsproxy 会 server closed abruptly）；pnpm 挂代理反而慢 12 倍。
- 改进：文档里分开写，别给一条通用建议。

## B. XML / catalog

### B1. 自闭合元素走 Empty 事件，不是 Start

- 现象：扫 catalog 只拿到 3,646 行，真实 45,706 行，**漏 92%**。
- 根因：`<CUnit id="X"/>` 在 quick-xml 里是 Empty 事件，只处理 Start 就全漏。
- 怎么发现的：单元测试对不上，再用 Python ElementTree 独立计数。
- 改进：已修。**任何 XML 扫描都必须有独立解析器对账**。

### B2. 类型到文件的映射只对「写新行」成立

- 现象：以为 CUnit 一定在 UnitData.xml。
- 根因：第三方 mod 把 CUnit 按指挥官拆进 `commanders/commanderzeratul_zealot.xml` 等 61 个随意命名的文件。
- 怎么发现的：读 `泽拉图之遗` 的实际目录。
- 改进：**读用全树扫描建索引**（按根元素 Catalog 判断），写才用族表选标准文件。已在 DESIGN 第 10 节修正。

### B3. Value 不一定是单位引用

- 只有 `ValueGameType Type="Unit"` 的才算。`<Value>Zealot</Value>` 在地图里出现过，但不在 Unit 类型的 Param 里，所以工具报 0 是**对的**。
- 改进：工具报 0 时先怀疑题目，别先怀疑工具。

## C. MPQ 打包

### C1. 散列类型索引错位

- 现象：哈希表 50 条全在，但一条都查不到。
- 根因：MPQ 约定 slot=类型 0、HASH_A=类型 1、HASH_B=类型 2；我写成了 A=0/B=1/slot=2。
- 改进：已用 mpyq 的独立实现把标准值钉成测试。

### C2. 哈希表与块表强制加密

- 现象：表写对了，读取器读出来是随机数。
- 根因：两张表用 `hash("(hash table)", TABLE)` / `hash("(block table)", TABLE)` 当密钥加密存储。
- 改进：已实现 encrypt_table。

### C3. 加解密不是对合（这条最坑）

- 现象：加密后解不回来；表头第一个字段对、后面全乱。
- 根因：种子更新必须用**明文**。解密时明文是输出，加密时明文是**输入**，所以不能共用一次运算。
- 怎么发现的：我先用 Python 复刻了一份解密和 Rust 比，**两边一致**就以为对了，但那两份都是我按同一个错误理解写的。真正定位靠「不是对合」这个反证（第一个 u32 能还原、后面不能）。
- 改进：**验证不能用自己写的第二个实现**，必须用第三方（这里是 mpyq）。

### C4. 交付形态是单文件 MPQ

- 解开目录是编辑器「存成组件」的开发形态（49 个碎文件）；分发必须是单个 .SC2Map。
- 真实地图两种格式都有：EditorTest 是 v1（32 字节头），Enslavers Redux 是 v3（208 字节头），引擎都认，所以写 v1 就够。

## D. 地图与触发器语义

### D1. 生成点有三种形态，扫描要全覆盖

| 形态 | 在哪 | 当时工具 |
|---|---|---|
| UnitCreate / libNtve_gf_CreateUnitsAtPoint2 | MapScript.galaxy | 有 |
| UnitCargoCreate（装载，卸载时才算进场） | MapScript.galaxy | **没有** |
| 地图里预置的单位 | Objects（二进制） | 没有 |

「从运输机上下来的 5 个」是**第二种**。第一次改的是 CreateUnitsAtPoint2 的群演，跟它无关。
- 改进：`sc2cat spawns` 要把 UnitCargoCreate 也算进去，并标注「装载」而非「生成」。

### D2. 过场与落地是两段，只改一处只改一半

- Intro Cinematic 装货 5 个 → Intro Cargo Unload 卸货转玩家 → Intro Cleanup - Escape **再造 5 个**给玩家操作。
- 改进：改单位时按名字前缀扫出全部引用点并按触发器分组，提示「这个单位出现在 3 个触发器，是否一起改」。

### D3. 地图文件名必须与战役任务名一致

- 现象：交付 `traynor01_zeratul.SC2Map`，进游戏和原版没区别。
- 根因：战役任务表认的是 `traynor01`，文件名不同等于不覆盖。
- 改进：包约定里加硬检查；pack/shrink 应保留原文件名。

### D4. DocumentHeader 是二进制

- 格式：u32 条数 + N 个 NUL 结尾的 UTF-8 串（`file:Mods\X.SC2Mod`）。
- 改进：已实现 add-dep；还应加 rm-dep 与「列出全部依赖」。

## E. 启动器与交付

### E1. mods 只标记，不挂载

- 现象：声明了依赖，启用后 Mods/ 里却没有它，地图报「模块或地图关联内容已不可用」。
- 根因：mods 只用来在 UI 上标 required；真正挂载的是 variant.payloads（包里带的）加当前启用版本。
- 怎么发现的：读 library/mod.rs 与 package.rs 的 payload 收集。
- 改进：**交付物必须自带全部依赖 mod**，或者明确要求用户先装。仅声明不够。

### E2. 启用/停用会换掉整个 Maps/ 与 Mods/

- 现象：测试时 Maps/Campaign 变空、Mods/ 只剩我带的那个，游戏回落 CASC 原版。
- 根因：启动器「同一时刻只能启用一个版本」，启用我的战役会顶掉用户原来的战役包，连带撤走它的 mod。
- 改进：**调试交付物时必须先确认游戏目录实际状态**，别把「没效果」当成代码问题。工具应提供 `sc2cat doctor` 核对每个 file: 依赖能否解析。

### E3. 包元数据字段类型

- `miyin.mods` 是 Vec<String>，写成字符串会让整个 metadata.json 解析失败，连带 campaign 丢失、回落到按地图名猜归属。
- `miyin.format` 文档自相矛盾（一张表写 1、另一份写 2、版本表又说 2 = kind/priority/requires）。当前取 2。
- 改进：套件侧加 verify-metadata，按 StandardMetadata / MiyinExtensions 的结构逐字段校验类型。

### E4. HSCL 依赖模组校验有 bug（已提 issue #36）

- mod_identity().key 保留 .SC2Mod 后缀，normalize_mod_key() 去掉，两者判等永远不等。
- 改进：等上游修。

## F. 工具与流程（自己的坑）

### F1. String::replace 匹配不上是静默无操作

- 用 replace 改代码，old 打错一个字就什么也没改，且没有任何提示。
- 改进：改完必须 grep 回来确认，或者用会报错的编辑工具。

### F2. 子 agent 的「看起来完成」不等于完成

- 一个子 agent 交上来的文件行数够了、TODO 也没了，但结构性错误（在已出栈的节点上找父节点）导致函数对任何输入都返回空，它自己的 6 个测试全红。
- 改进：子 agent 交付后**自己跑一遍它的验收命令**。

### F3. 中文标识符容易混进空格

- `fn 空_edits 返回原文()` 中间一个空格就编译不过。子 agent 产出过两处。

### F4. PowerShell 引号地狱

- 多层嵌套引号、here-string、$args 自动变量、反斜杠被当正则……本会话至少 8 次语法错。
- 改进：超过两三行的命令一律写脚本文件，不塞进 -Command。

### F5. Python 脚本名撞标准库

- 脚本命名成 bisect.py，tempfile 间接 import bisect 时被顶掉，报 partially initialized module。
- 改进：_recon 里的脚本起名避让标准库。

### F6. 文档写作时占位符冲突

- 用特殊字符当反引号占位符，结果和章节号冲突（「见 §7」被替换成「见 `7」）。
- 改进：占位符换成不会出现在正文里的字符。


## G. CASC 原件与地图基底（2026-10-17 追记）

> 这一段是「做自由日替换地图」全过程踩出来的。**顺序即教训**。

### G1. 不要拿参考项目当基底

我一开始从参考目录里的战役包取 `traynor01.SC2Map`，结果是别人的改版：依赖 `3疯批帝国之翼` / `kit_liberty_story` / `Alenger`（合计 236 MB），为单位换名（`Marine` → `WarPigLand` → `3diguochongfengdui`）。
为了让地图能跑，我又把这些 mod 全打进包——**整条链都是错的**。

**真实开发不会提供这些参考。** 基底只能从游戏本体取。

### G2. SC2 的地图在 CASC 里，是解开形态

`casc` CLI（`casc-cli`，包 Zezula 的 CascLib，有 Windows 预编译 + sha256）能直接读 SC2 的 CASC：

```text
CASC 总条目 779,211   其中 .sc2map 6,479
campaigns\liberty.sc2campaign\base.sc2maps\maps\campaign\traynor01.sc2map\
```

**这一层是编辑器组件形态**，不是打包的 MPQ。

### G3. 抽取必须用递归通配

```bash
casc extract -o out <SC2Data> "…/traynor01.sc2map/*"     # ✗ 只有一层
casc extract -o out <SC2Data> "…/traynor01.sc2map/**"    # ✓ 递归
```

用 `*` 时漏掉了 **嵌套子目录**——正好是最关键的两个：

```text
base.sc2data\gamedata\terraindata.xml      地图本地 GameData
base.sc2data\gamedata\unitdata.xml
base.sc2data\gamedata\waterdata.xml
zhcn.sc2data\localizeddata\gamestrings.txt  中文字符串
```

**症状：进游戏文本全是英文、部分键缺失。** 32 个 vs 48 个的差别全在这里。

### G4. ComponentList 必须与实际语系一致

`componentlist.sc2components` 声明了 12 个语系，但游戏只下载了 `zhcn`（其余在 CASC 里抽不出来）。
**声明了却没有文件，本身就是「缺键」的来源。** 交付前要把 `text` 条目收敛到实际存在的语系。

### G5. `base.sc2data` 不是语系

清理空语系目录时按「没有 gamestrings 就删」处理，结果把 `base.sc2data`（里面是地图本地 GameData）**一起删了**。
判据要加一条：`loc == 'base'` 时跳过。

### G6. `triggers` 与 `mapscript.galaxy` 是两回事（本条最贵）

| 文件 | 是什么 | 谁在用 |
|---|---|---|
| `triggers` | 编辑器的**源**（触发器图） | **只有编辑器** |
| `mapscript.galaxy` | 从触发器编译出的 Galaxy 代码 | **游戏运行时执行的是它** |

我改了 6 处 `triggers`，进游戏**毫无变化**——游戏跑的是没改过的 `mapscript.galaxy`。

**套件规则「不手改 MapScript.galaxy」的前提是「在编辑器里开发」**（编辑器保存会重新生成）。
但**产出交付物**时没有编辑器参与，必须同时改它，否则改动不生效。

> 工具侧结论：`sc2cat set-unit` 必须**同时改两份**——`triggers` 里的 `Param` 与 `mapscript.galaxy` 里的字面量调用，按同一组调用点对齐。

验证信号：改完 `mapscript.galaxy` 后，**原本的陆战队员消失了** → 说明改动生效了。剩下的问题就不在触发层。

### G7. 单位 id 在不同改版里不一样

同一个「运输机上下来的陆战队员」，在三张图里是三个 id：

| 图 | id |
|---|---|
| 暴雪原版 | `Marine` |
| 枪兵之翼 | `WarPigLand` |
| 测试1（帝国之翼系） | `3diguochongfengdui` |

**在原版里搜 `WarPigLand` 一处都搜不到**——所以选错基底时，「目标单位」这个词本身就变了。

原版真正的结构（`gt_IntroCinematic_Func`）：

```galaxy
libNtve_gf_CreateUnitsAtPoint2(1, "SpecialOpsDropship", 0, gv_p1_USER, PointFromId(379));
gv_introDropship = UnitLastCreated();
UnitCargoCreate(gv_introDropship, "Raynor01", 1);   // 雷诺
UnitCargoCreate(gv_introDropship, "Marine", 5);     // ← 这五个
```

配合 `gt_IntroCargoUnload_Func`（卸货时 `UnitSetOwner(EventUnitCargo(), gv_p1_USER)`）
与 `gt_IntroCleanupEscape_Func`（过场结束**再造 5 个**给玩家操作）——**两段都要改**。

### G8. 精简 mod：catalog 闭包看不到 `.m3` 内部引用

按 catalog 引用做闭包（45,816 条 → 1,005 条）后，`Assets` 只剩 20 个文件。
但 `Zealot_Ihanrii_Elite.m3` **内部**还引用了 27 个贴图路径——这些**不在任何 catalog XML 里**，闭包看不见。

> 本例里那 27 个其实是暴雪自带资源（原 mod 里也没有），所以不是本次故障原因；
> 但**机制上的坑是真的**：模型文件的内部引用必须单独扫。

### G8b. 「看不见单位」不等于「模型缺失」（这条纠正了 G8/G9 的推理方向）

**星际争霸的模型丢失会显示错误模型（占位方块），不是让单位消失。**

所以：

| 现象 | 含义 |
|---|---|
| 画面上是错误模型 / 粉色方块 | 模型或贴图缺失 → 资源问题 |
| **单位完全看不到** | **单位根本没被创建** → 数据/加载问题，与渲染无关 |

出现「看不到」时，**先怀疑 `CUnit` 是否存在、mod 的 GameData 有没有加载**，
不要去查 `.m3`、贴图、actor——那条路是错的。

判断办法：按 **F2**（选中全部部队）看单位数。
有 5 个但画面空 → 渲染；单位数是 0 → 没创建。

### G9. 组件格式 mod 的 `.version` 与 `Preload.xml`

精简时把 `DocumentInfo.version` / `GameData.version` / `GameText.version` / `Preload.xml` 一起丢了。
原 mod 四个都有。组件格式靠 `.version` 识别组件，缺了有理由怀疑整个 `gada` 组件被跳过。

**但按 G8b 的判据，补回这四个文件后如果单位仍然完全看不到，就不该再往资源方向查**——
说明 `ZeratulZealot` 这个 `CUnit` 在运行时确实不存在，问题在 mod 加载本身。
最后改用**完整未精简的 mod** 做对照实验，就是为了切断「精简」这个变量。

### G12. 移植协同作战单位：卡技能的是**建筑**，不是升级

把 `ZeratulZealot` 放到战役图上后技能栏是空的。查下来门控是：

```text
HaveZeratulArtifactTier1AndCyberCore
  = AndCountUnitZeratulCyberneticsCoreCompleteOnly
    + CountUpgradeZeratulArtifactTier1CompleteOnly
```

**要求一个「泽拉图控制核心」建筑**——协同作战专有，战役图上不存在。
所以「发给玩家升级」这条修路走不通：**建筑那一半永远满足不了**。

### G13. 「面板什么都不显示」的正确修法是让**锁定态可见**，不是解开门控（★）

我第一反应是把 5 处 `Requirements` 全删掉。**这是错的**——用户要的是「像正常情况那样显示尚未获得的科技」。

正确机制：SC2 用 **`parent="ResearchLocked"` 的锁形占位按钮**表示「未获得」。

原 mod 的卡片布局是**一个槽位挂两个按钮、靠 Requirements 互斥**：

| 槽位 | 正常时显示 | 未获得时显示 |
|---|---|---|
| 1,0 | `ZeratulZealotWhirlwindPassive`（Tier1） | **（作者漏了占位）** |
| 1,1 | `ZeratulZealotStunPassive`（Tier2） | `ZeratulLockedLevel3` |
| 1,2 | `ZeratulZealotRetreatPassive`（Tier3） | `ZeratulLockedLevel11` |

**坑在占位按钮的门控是指挥官等级**（`ZeratulLevel03` / `ZeratulLevel11`）——
战役图里没有指挥官等级，所以**占位也不显示**，整个第一行为空。

**修法**：加反向 requirement，把占位的门控从「等级到了」改成「对应科技还没拿到」：

```xml
<CRequirementNot id="NotHaveZeratulArtifactTier1AndCyberCore">
    <NodeArray index="Use" Link="HaveZeratulArtifactTier1AndCyberCore"/>
</CRequirementNot>
```

然后 `Requirements="ZeratulLevel03"` → `Requirements="NotHaveZeratulArtifactTier2AndCyberCore"`，
并给 1,0 补一个原本缺失的占位。

**原有的科技门控一个都不要删**——删了就等于把「未获得」这个状态本身删掉了。

### G14. 单位显示名在 `Actor/Name`，不在 `Unit/Name`

这个 mod 里**没有** `Unit/Name/ZeratulZealot`，显示名是 `Actor/Name/ZeratulZealot=Xel'Naga Warrior`。
汉化时两个键都写一份最稳。

### G15. 任务简报也是触发器里的 Marine

「开场运输机卸下的 5 个」和「任务简报页面的陆战队员」是**两组不同**的调用点：

| 触发器 | 处数 | 归属玩家 |
|---|---|---|
| `CC2D6739` Intro Cinematic | 1 | `gv_p1_USER`（`UnitCargoCreate(..., 5)`） |
| `EEEC2125` Intro Cleanup - Escape | 5 | `gv_p1_USER` |
| `5E09D203` Briefing Scene 00 | 4 | `gv_p2_DOMINION_RED` |
| `F12B8538` Briefing Scene 01 | 4 | `gv_p4_ACTORS_BLUE` |

合计 14 处，`triggers` 与 `mapscript.galaxy` **两处都要改**，字节变化应为 `14 × 7 = 98`（`"Marine"` 8 字符 → `"ZeratulZealot"` 15 字符）。

### G10. DocumentHeader 的依赖表有两种串格式

```text
file:Mods\X.SC2Mod                                 裸的形式
bnet:Liberty (Mod)/0.0/999,file:Mods/Liberty.SC2Mod  带 bnet 前缀
```

两种都会出现。依赖**条数固定在偏移 44**（u32），串从 48 起、NUL 结尾。
靠「第一个 `file:` 往前 4 字节」定位会读到 741947705 这种荒谬值。

### G11. 交付物形态

- 地图必须是**单文件 MPQ**（解开目录是编辑器开发形态）
- 包布局 `Maps/Campaign/traynor01.SC2Map` + `Mods/<mod>/`，**文件名必须与战役任务名一致**（否则不覆盖）
- `mods=` 只是标记必需，**不负责挂载**；依赖要么打进包，要么确认用户已装

## H. 本地化与组件结构（2026-10-17 追记，收尾阶段）

> 这一组全是「看起来改对了、进游戏全是裸键」的坑。

### H1. CASC 抽出来的是**源形态**，不是分发形态

| | CASC 源形态 | 能跑的成品图 |
|---|---|---|
| 语系目录名 | `zhcn.sc2data` 全小写 | `zhCN.SC2Data` **驼峰** |
| 语系数量 | 只带当前语系 | **12 个全带** |
| `ComponentList` | 声明 12 个（文件可能缺） | 声明 12 个、文件也齐 |

按源形态打出来的图，**文本组件不生效，画面上所有文字变成裸键**（`Param/Value/803351AA` 这种）。
修法：`sc2cat normalize` —— 驼峰化 + 补齐 12 语系 + 重写 `ComponentList`。

**症状特征**：游戏自身的 UI 字符串（`等级: 信徒`、`轻甲 - 生物单位`）是中文，只有地图/mod 的文本是裸键。

### H2. 本地化文件要带 UTF-8 BOM

Blizzard 的文件带 `\xef\xbb\xbf`，手写的不带就**读不到**。
`sc2cat localize` 现在会保留原 BOM。

### H3. 一个语系目录里要放**整套**文件，不是只有要改的那个

原版 mod 的 `enUS.SC2Data/LocalizedData/` 有 5 个文件
（`ConversationStrings` / `GameHotkeys` / `GameStrings` / `ObjectStrings` / `TriggerStrings`），
功能正常的第三方 mod 每份都有 4 个。
只放一个 `ObjectStrings.txt` 的语系**不生效**。

### H4. 设置字符串是**替换**，不是追加

往文件末尾追加 `Unit/Name/X=中文`，而文件前面已经有 `Unit/Name/X=Xel'Naga Warrior` 时，
**先出现的赢** —— 追加等于没改。必须就地替换。

### H5. `.version` 清单与实际文件集必须一致

`GameText.version` 这类文件记录「这个组件有哪些文件」。
改了文件集（比如从 1 个语系变 12 个）而清单没跟着变，**组件会被判无效**。
生成不出正确清单时，删掉它比留一份错的强（游戏能容忍缺清单）。

## I. 盲测暴露的（2026-10-17）

> 派了个子 agent 只给 `--help` 去做同一件事，12 分钟。它找到两个我没预料到的问题。

### I1. `set-unit` 的两个作用域参数各管一半，只给一个会产出半成品（★ 危险）

`--in-trigger` 管 `Triggers`，`--galaxy-fn` 管 `MapScript.galaxy`，**彼此独立**：

| 给了什么 | Triggers | galaxy | 结果 |
|---|---|---|---|
| 只 `--in-trigger` | 改 | **不改** | 进游戏毫无变化 |
| 只 `--galaxy-fn` | **全图改 25 处** | 改 3 处 | 两边不一致，更糟 |
| 两个都给 | 按触发器 | 按触发器 | ✅ |

已修：`--galaxy-fn` 现在**必须与 `--in-trigger` 同时给**（clap 的 `requires`），且帮助文本改成实话 —— 原文写「没有 TriggerStrings 时才需要」，读起来像可选补丁，实际是 galaxy 侧的**总开关**。

### I2. 打包产物读不回来

`pack` 出来的单文件 MPQ，`layout` / `find-unit` / `spawns` 一律报「不是目录」——
交付形态只能靠「打包前验目录 + 体积对账」这种间接证据。

已修：新增 `sc2cat unpack` + `mpq::read_entries`。
顺带发现写入器里那句「加解密是同一套运算（对称）」的注释**本身是错的**（G 段 C3 早就证过），一并订正。

### I3. `find --type` 必须写 `CUnit` 不写 `Unit`

写错了**静默 0 命中**，子 agent 差点判定「模组里没这个单位」。
已修：两边都认，`C` 前缀可有可无。

### I4. `find-unit` 有 7 处「(无归属)」

那 7 处里 5 处实际在某个触发器内，而且**正好是最像目标的诱饵**（`UnitGroup("Marine", ...)` 统计查询）。
**未修** —— 要改触发器的边界解析。记在 TODO T2。

### I5. 「5 个陆战队员」不是引用数最多的那个

子 agent 的弯路：`find-unit Marine` 里正好有 5 处引用的触发器（EEEC2125）是**空投舱**那段；
真目标在**只有 1 处**引用的 `CC2D6739`，靠同块的 `SpecialOpsDropship` + 参数 `5` 才认出来。
**这不是工具的锅**，是「按表象找目标」的典型陷阱 —— 记在这里提醒。

## J. 盲测复盘：哪些是任务书的锅，哪些是工具的（2026-10-17）

第一次盲测**成功执行了我写的任务**，但它做出来的东西**不是我要的**。逐条归因：

### J1. 任务书的三个缺口（**不是执行者的问题**）

| 缺口 | 我写的 | 它做出的 | 真实需求 |
|---|---|---|---|
| **只描述入口现象** | 「把从运输机上下来的 5 个换成 X」 | 改了那 1 处调用 —— **逐字照做** | 过场后玩家手上仍是 5 个狂热者（单位被造了**两次**） |
| **交付形态写错** | 「一个目录，形如 Maps/… Mods/…」 | 真的只给了目录 | 要 `metadata.json`，否则启动器导入不了 |
| **已知前提没交代** | —— | 无从知道 | 需要 `ZeratulCommander` 上下文，否则技能面板空白 |

**教训**：任务书要写**验收标准**（进游戏看到什么，且要可数）、**交付形态**（能直接被用的那个）、**已知前提**。
模板见 `tools/devkit/BLIND-TEST-BRIEF.md`。

**「可数」这条特别重要**：我后来写的 `doctor` 查不出「该改 14 处只改了 1 处」——
因为产物**两边确实一致**，工具无从知道意图。意图只能在任务书里写死。

### J2. 工具的三个缺口（已补）

| 缺口 | 现状 |
|---|---|
| 没有「往 Galaxy 插代码」的命令 | ✅ 新增 `sc2cat inject`（插在动作段的 `runActions` 守卫之后），2 个测试 |
| 没有「装成可导入的包」的命令 | ✅ 新增 `sc2cat package`（补 `metadata.json` + 用平台 `tar` 归档，不自己写 zip 格式） |
| 没有「整体自检」 | ✅ 新增 `sc2cat doctor` —— 查触发同步 / 语系结构 / 依赖解析 / metadata 形状 |

`doctor` 实测：盲测产物 **FAIL 元数据**，我们的 v8.6 **全部通过**。

### J3. 第一次盲测暴露、已修的三条

见 I1（`--galaxy-fn` 必须配对）、I2（新增 `unpack`）、I3（`find --type` 宽容匹配）。

### J4. 顺带发现：`repro.sh` 自己也是半成品

它没把依赖模组复制进产物 —— `localize` 会凭空建一个只有本地化文件的空壳，
产出 1.45 MB 的「包」而 `doctor` 还说通过（因为游戏的 `Mods/` 里装了同名模组，依赖查得到）。
**已修**：加上 `MOD_SRC` 与存在性检查，现在产出 49 MB 完整包。

> 这条本身就是「自检也有盲区」的例子：`doctor` 查的是**能不能解析**，不是**包是不是完整**。

## K. 盲测第二轮的收获（2026-10-17）

按 `BLIND-TEST-BRIEF.md` 写任务书重跑，**6 条验收标准全过**，12 分钟。
说明「验收标准可数 + 交付形态说清 + 已知前提列全」确实管用。

### K1. `doctor` 不认 zip（已修）

交付物是 zip，自检却只吃目录 —— 最尴尬的一种不自洽。
现在 `doctor` 可以直接吃 zip（内部用平台 `tar` 解到临时目录）。

### K2. bsdtar 会把 `D:` 当成远程主机（已修）

`tar -f D:/x.zip` 报 `Cannot connect to D: resolve failed`。
修法：`current_dir` 到父目录 + 只传裸文件名；另外 `-C` 的目标**必须是绝对路径**，
因为 `current_dir` 换过之后相对路径会指错地方。

### K3. `inject` 只写 `MapScript.galaxy`，不写 `Triggers`（已改说明）

它和 `set-unit`（两边都写）行为不一致，而 help 里那句「改了源不改产物等于没改」**反而误导**。
真实约束：用 Galaxy 代码往 `Triggers` 里补 GUI 动作、得构造带 id 的 Element，成本远高于收益 ——
所以 `inject` 只写产物，**代价是编辑器打开并保存这张图时会被覆盖**。help 已改成说实话。

### K4. `package` 的 help 没说它也能吃 `.SC2Mod`（已改）

### K5. 我自己的一处**真错误**：多余的 `CampaignContext.xml`（已删）

为了让技能面板亮起来，我在 mod 里新建了 `CampaignContext.xml`，定义了
`CUpgrade ZeratulCommander` / `CommanderLevel` 两个极简升级。

盲测者反查发现 `ZeratulCommander` 在 `UpgradeData.xml` 里没有对应条目 —— 它搜窄了。
但顺着它查下去发现了**真问题**：

| id | mod 里本来就有 | 在哪 |
|---|---|---|
| `ZeratulCommander` | ✅ 而且是**十几条 `EffectArray`** 的丰富定义 | `commanders/commanderzeratul.xml` |
| `CommanderLevel` | ✅ | `UpgradeData.xml` |

**我那份是重复定义，只会覆盖掉丰富的那个** —— 会让星灵枢纽/探针的改名等 EffectArray 失效。

真正让面板亮起来的是地图里那条 `TechTreeUpgradeAddLevel`，不是这个文件。**已删除。**

> 教训：**先查依赖里有没有，再决定要不要自己造。**
> 一个「看起来解决了问题」的重复定义，代价可能是悄悄毁掉别人的数据。

## L. 依赖 mod 会把战役原文「抄」进来（2026-10-17，收尾）

### L1. 现象：包能加载，但**部分战役原文变成英文**

截图样本：目标栏 `Hero Units` / `Marines`、字幕 `Keep your intervals wide…`。
而同一屏的 `教程`、CHEAT 提示是中文 —— 说明中文数据在，只是**被盖住了**。

### L2. 根因：第三方 mod 是「在战役基础上做的」，导出时把战役文本整包抄进了自己

那个泽拉图 mod 的 `enUS/GameStrings.txt`（25,885 行、**0 个中文字**）里：

| 前缀 | 条数 | 是什么 |
|---|---|---|
| `Conversation/` | 7,924 | 战役对话：`TRaynor01` `TZeratul01` `TValerian02A` … |
| `ConversationState/` | 2,139 | `StoryTips` / `MissionObjective` / `CinematicSubtitles` … |
| `Param/` | 1,045 | **战役地图的字符串键** |

`TRaynor01` 就是自由日。这些键覆盖了战役自己的中文 → 玩家看到英文。

**而 `ObjectStrings.txt` 反而不覆盖原版**（实测 `Unit/Name/Marine` 等都不在里面）——
所以**不能整个删掉 locale 文件**，只能**按前缀裁**。

### L3. 我在这上面犯过两次错

1. 把 mod 的 `enUS` 目录**原样复制**成 `zhCN` —— 等于把那份英文抄进了中文语系
2. 更早：为了让单位名显示，补了一堆语系文件，从没检查过里面装的是什么

**教训：补语系文件前，先看清楚那份内容属于谁。**
第三方 mod 的本地化文件不一定只描述它自己。

### L4. 修法：按前缀裁（已做成命令）

`sc2cat prune-locale <mod> --drop-prefix Conversation/ --drop-prefix ConversationState/ --drop-prefix Param/`

裁掉 36,216 个键（6 个文件），保留 mod 自己的 `Button/` / `Effect/` / `Unit/` / `UserData/`。

效果：两段英文样本消失 ✓，`ObjectStrings` 仍有 29,826 行（单位名都留着）✓，`doctor` 全绿 ✓。

> **边界**：哪些前缀算「战役专属」要判断，不能自动推。
> 这三个是这次的答案，换一个 mod 要重看。

## M. **拒绝全局替换**（硬规则，2026-10-17）

> 这一条是用户明确立下的，不是我的建议。

### M1. 规则

**改动的影响面，必须等于需求的影响面。**

| 拒绝 | 为什么 |
|---|---|
| 覆盖**游戏自带的全局 id**（如 `WarPig`、`Marine`、`Zealot`） | 一个 `Unit/Name/WarPig=…` 会让**整场战役**所有陆战队员改名；覆盖它的 actor 会让全战役换外观 |
| 全量字符串替换 | 同上，影响面失控 |
| 整类覆盖（"把一类单位全换掉"） | 需求通常只针对**某几个点**，不是某一类 |

**允许**：改**具体创建点**、造**新 id**、只动**本包内的文件**。

### M2. 为什么这条比「能跑通」更重要

我为了定位「简报面板的单位怎么绑的」，做了一个**只改两行字符串**的探针
（`Unit/Name/WarPig` / `Button/Tooltip/WarPig`）—— 它确实能一次定死方向，
代价是**整场战役的陆战队员全叫「萨尔那加狂热者」**。

用户否决。**这是对的**：一个「能跑」但污染全局的测试，比没有测试更糟 ——
测试完用户得卸包恢复，中间还可能出现存档、Bank、成就状态被写入。

**推论**：**当一个实验必须靠全局替换才能做，这个实验就不该做。**
应该换静态分析路线，或者承认结论到此为止。

### M3. 写文档时怎么落实

任何「验证方法」写进文档前，先过一遍：**它会不会改到全局 id？**
会 → 不写，或者写成「不推荐」并说明为什么不行。

## N. 测试前先核对部署状态（2026-10-17，代价很大）

### N1. 现象

用户报「敌方陆战队员没替换」「没汉化」「模型变成普通狂热者」——三个都像代码 bug。

### N2. 真相：**测的是 6 小时前的残留**

查游戏目录：

```text
Maps/Campaign/   空的                                    ← 地图根本没装
Mods/            zeratul-zealot-kit.SC2Mod  8 MB  16:46   ← 我早期「精简版」实验的残留
                                         只有 enUS.SC2Data  ← 所以没汉化
                                         Assets 只有 20 个文件 ← 贴图缺，所以模型是普通狂热者
```

三个症状**全部**由「装的是旧残留」解释 ✓。真正的新包（298 MB mod + 2 张地图）**一个文件都没进去**。

### N3. 为什么会被骗这么久

- 用户说「能正常加载」→ 我以为是新包能跑 ✓ 实际上**原版也能加载** ✗
- 中文/模型的外观差异，看起来太像「数据写错」✗ 实际是「根本没换」✗

### N4. 规则

**任何游戏内测试结果，先核对三件事再分析：**

| 核对什么 | 怎么查 |
|---|---|
| 游戏目录里的地图是不是**这次的** | 对比文件时间戳与包生成时间 |
| 游戏目录里的 mod 是不是**这次的** | 对比目录大小（新包 298 MB vs 旧残留 8 MB） |
| 是不是有多余的**旧残留**抢占 | `ls Mods/` `ls Maps/Campaign/` |

**同一个 mod 目录只留一份**：历史上我用过 `zeratul-zealot-kit.SC2Mod` / `kit_lotv_zeratul.SC2Mod` 两个名字，
残留的那份会一直生效且不会被发现。

> 这条和 G 段的 E2（启动器会换掉整个 Maps/Mods）是一类问题：
> **测试环境的状态是测试结论的一部分**，不能默认它是对的。

### H5b. `.version` 清单：地图删了，**mod 忘了删**（2026-10-17 实测）

H5 说过「清单与实际文件集不一致，组件会被判无效」。我给**地图**删了 `gametext.version` ✓，
但**没有给 mod 删** `GameText.version`。

`normalize` 把 mod 从 2 个语系扩成 12 个之后，那份 44 字节的旧清单（只声明 2 个）就再也对不上了
→ **文本组件被判无效** → 游戏回落到 `enUS`
→ 实测症状：单位名显示 `Xel'Naga Warrior` 而不是我们写的「萨尔那加狂热者」。

**规则**：`normalize` 只要改了语系数量，就必须同时删 `GameText.version`。
（`doctor` 该查这一条，现在没查。）

## O. 「模型看起来不对」先比贴图表，别猜（2026-10-17）

### O1. 现象

换上的单位名字、提示、数值都对（护盾 150 证明 GameData 加载了），
但**模型看起来像普通狂热者**，用户报「模型不对」。

### O2. 查了什么（全部通过，所以不是 bug）

| 检查 | 结果 |
|---|---|
| 模型链路 | `CUnit` → `CActorUnit`（`<Model value="ZealotZeratul"/>`）→ `CModel` → `.m3` 全在 |
| `.m3` 文件 | 在包里，468,816 字节，与原始 zip 逐字节相同 |
| 它引用的 27 张贴图 | CASC 里 **27/27 全部存在** |
| PreloadAssetDB | `id=ZealotZeratul` + `asset=…Zealot_Ihanrii_Elite.m3` —— **暴雪官方就是这个映射** |
| 我们的改动 | 324 文件里只有 2 个不同，模型链路一个字节没动 |

### O3. 真正原因：那个模型的**主材质就是普通狂热者的贴图**

把三个 `.m3` 的字符串表拉出来比前三条贴图引用：

| 模型 | 前三张 |
|---|---|
| `zealot.m3`（普通狂热者） | `Zealot_Diffuse` / `Zealot_Specular` / `Zealot_Normal` |
| `zealot_ihanrii.m3`（纯 Ihan-rii） | `Zealot_Ihanrii_Diffuse` / `_Emissive` / `_Normal` |
| mod 的 `Zealot_Ihanrii_Elite.m3` | **和普通狂热者完全一样** |

**所以它不是「渲染错了」，是「长这样」。** mod 作者拿普通狂热者贴图当基础涂装，
Ihan-rii 和精英涂装只用在次级材质上。

### O4. 规则

**「模型看起来不对」先做两件事，再怀疑代码：**

1. **比贴图表**：把目标 `.m3` 和「看起来正常的那个模型」的贴图字符串表拉出来比。
   前几条不同 → 外观本来就不同；前几条相同 → 外观本来就该一样。
2. **查 `PreloadAssetDB.txt`**：它会写明「哪个 `CModel` id 对应哪个 `.m3`」。
   如果映射和暴雪官方一致，就说明**这条链是原厂设计**，不是我们接错的。

> 反例教训：我先后猜过「`<Optimized/>` 屏蔽组件」「贴图缺失」「被 tstory01 那次改动影响」——
> **三个全错**。前两个查文件就能否掉，第三个用「数据逐字节相同」否掉。
> **时间相关 ≠ 因果。**

### O5. 想要不同外观时的正确改法（非全局）

`CModel ZealotZeratul` 在整个模组里**只有一份定义、只被 `CActorUnit ZeratulZealot` 引用**，
所以改它的 `<Model value="…"/>` 指向别的 `.m3` **不会影响任何其它单位** —— 符合规则 M。
改完要重新 pack / 重装，并确认新模型的动画（如 `Attack,00/03/04`）齐全。

## P. **迁移必须留痕**：依赖漂移报告（2026-10-17，用户点名的设计问题）

### P1. 用户的原话

> 「从已有依赖项做迁移不应该再制造出这样的缺漏，这是**重大设计问题**。」

**这是对的，而且它是这一整轮反复出错的根因。**

我们做的是「拿别人做好的 mod，改一点，连同包一起发」。
但**从头到尾没有一个东西能回答「我们到底改了什么」** ——
于是我每一轮都是**改完、发出去、等用户测出来**才发现漏了或改坏了。

### P2. 补的命令

```bash
sc2cat diff-dep <原始依赖目录> <我们交付的拷贝>
```

输出三行：**丢了几个 / 加了几个 / 改了几个**，逐个列出。

**判据**：

| 结果 | 含义 |
|---|---|
| **丢了 ≥ 1** | **迁移事故，直接拒收**（工具会 FAIL） |
| 加了 / 改了 | 每一条都**必须能解释清楚**才能交付 |

### P3. 第一次跑的真实结果

```text
丢了 0 个 / 加了 5 个 / 改了 3 个
  +  zhCN.SC2Data/LocalizedData/ 的 5 个文件
  ~  ComponentList.SC2Components            409 -> 480 字节
  ~  enUS.SC2Data/LocalizedData/ConversationStrings.txt   688977 -> 680134
  ~  enUS.SC2Data/LocalizedData/GameStrings.txt          2418683 -> 1425525
```

**324 个文件里 302 个逐字节相同** —— 包括全部 GameData 表与 Assets。
所以「模型是不是我们改坏的」这个问题，**现在有工具能一眼回答**，不用再靠猜。

### P4. 规矩

**交付前必跑 `diff-dep`。** 它把「迁移有没有掉东西」从**事后发现**变成**事前拦截**。
没有它，任何「我只改了一点点」的说法都是不可验证的。

## Q. 「文件没变但外观变了」= 依赖顺序（2026-10-17，疑难）

### Q1. 症状

依赖 mod 提供的单位，**特制模型变成普通模型**。但：

- mod 与原始包**逐字节对比：324 个文件里 302 个相同**，模型链路一个字节没动
- 那个 `.m3` 引用的 **27 张贴图，CASC 里 27/27 全在**
- `CModel` 直连模型无 parent；`CActorUnit` 的 parent 是标准模板
- 名字、提示、数值**全对**（护盾 150 证明 GameData 加载了）

**五个猜测全部被证伪**：Optimized 标记屏蔽组件 / 贴图缺失 / 模型数据被改 /
两个 mod 来源不同 / 合作指挥官 mod 撞名覆盖。

### Q2. 定位方法：**依赖二分**

用户手里正好有一条阶梯——四个包，每个只比前一个多一个变量：

| 包 | 相对上一个新增 |
|---|---|
| ① 基线 | —— |
| ② | ＋合作指挥官依赖 |
| ③ | ＋第二张地图 |
| ④ | ＋改预置单位 |

**从基线逐个装，第一个变错的包就是元凶。** 一次定位到「加合作指挥官依赖」。

> **这是这一轮唯一有效的方法。前面五轮静态分析全部落空。**

### Q3. 机制

地图依赖表是**有序**的，**后面的盖前面的**（`DocumentHeader` 偏移 44 起是条数，
随后是 N 个 NUL 结尾串，顺序即优先级）。

初始顺序：

```text
1  Liberty.SC2Mod
2  Liberty.SC2Campaign
3  LibertyStory.SC2Campaign
4  kit_lotv_zeratul.SC2Mod                     ← 自制 mod
5  StarCoop/Commanders/ArcturusMengsk.SC2Mod   ← 后加的，排最后
```

合作指挥官 mod 只依赖 `StarCoop.SC2Mod` 一条，但那是**合作模式基础 mod** ——
它把整套合作数据加进来，**优先级高于前面的自制 mod**，盖掉了自制 mod 的 actor 解析。

### Q4. 修法

**把自制 mod 排到最后**：

```bash
sc2cat set-deps <地图> --dep '...' --dep 'file:Mods\\kit_lotv_zeratul.SC2Mod'
```

**非全局**：只改这一张地图的依赖顺序，不动任何 mod 的内容。

### Q5. 三条教训

1. **静态分析全绿也可能漏掉「加载顺序」** —— 它不是文件内容，是**列表顺序**。
2. **二分法比猜快得多**：五个猜测不如四个包。
3. **保留中间产物**：那条阶梯之所以存在，只是因为我当时没删旧包。
   **每个交付版本留一份**，排查时就是现成的二分工具。

## 汇总：给套件的改进项

| # | 改进 | 状态 |
|---|---|---|
| 1 | catalog 按内容扫描 | 已做 |
| 2 | find-unit 带触发器归属 | 已做 |
| 3 | set-unit --in-trigger 精确限定 | 已做 |
| 4 | add-dep 改二进制依赖表 | 已做 |
| 5 | pack 打 MPQ v1 | 已做 |
| 6 | spawns 覆盖 UnitCargoCreate | 待做 |
| 7 | doctor 依赖解析自检 | 待做 |
| 8 | verify-metadata 类型校验 | 待做 |
| 9 | rm-dep / list-dep | 待做 |
| 10 | 文件名覆盖检查 | 待做 |