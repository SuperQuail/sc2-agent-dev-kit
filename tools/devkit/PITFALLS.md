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

### G9. 组件格式 mod 的 `.version` 与 `Preload.xml`

精简时把 `DocumentInfo.version` / `GameData.version` / `GameText.version` / `Preload.xml` 一起丢了。
原 mod 四个都有。组件格式靠 `.version` 识别组件，缺了有理由怀疑整个 `gada` 组件被跳过。

**最终仍未确认**是哪一项导致 `ZeratulZealot` 造不出来——所以最后改用**完整未精简的 mod**做对照。

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