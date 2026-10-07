# StarCraftIIAgent — 星际Ⅱ模组开发智能体

> 星际争霸2自定义战役开发智能体。统一知识库 + 29 个技能（10 根 + 13 galaxy + 6 sc2data），开发规则、工具与资料均在本目录维护。
>
> **作者：扯蛋虾米**。参见 [AUTHORS.md](AUTHORS.md)。

---

## 目录

- [简介](#简介)
- [作者声明](#作者声明)
- [文件结构](#文件结构)
- [环境要求](#环境要求)
- [快速开始](#快速开始)
- [任务路由](#任务路由)
- [工具速查](#工具速查)
- [常见场景示例](#常见场景示例)
- [硬性规则](#硬性规则)
- [智能体限制](#智能体限制)
- [故障排除](#故障排除)

---

## 作者声明

本智能体及其配套知识库、技能、工具、配置与文档由 **扯蛋虾米** 创作和维护。统一声明见 [AUTHORS.md](AUTHORS.md)。

---

## 简介

本智能体专为星际争霸2自定义战役开发而设计，整合两大模块（全部位于本目录下）：

1. **统一知识库** — wiki 设计/实现/参考文档、DataEditorXML 导出、W3C XSD schema、Python 工具链、catalog 图查询
2. **统一技能库**（`skills/`） — 29 个技能按需加载（10 根技能 + 13 个 Galaxy 子技能 + 6 个 SC2 Data 子技能）

当前尚未选择项目。提供主 Components `.SC2Mod` 路径后运行 `python tools/init-project.py "<路径>"`。

---

## 文件结构

```
./StarCraftIIAgent/
├── README.md                  ← 本文件（面向人类）
├── AGENTS.md                  ← 工作区入口：红线 + 路由 + 指向工具
├── AUTHORS.md                 ← 作者声明
├── DesignDocument.md          ← 设计来源与摘要
├── setup.md                   ← 安装与首次配置
├── agents/trae.yaml           ← Trae 智能体配置
├── skills/                    ← 知识与路由的唯一入口（29 个技能）
│   ├── sc2-*/                 ← 10 个根技能
│   │   ├── SKILL.md           ← 薄：何时用 + 流程 + 该跑哪些工具
│   │   ├── references/        ← 该技能的深度资料，需要时才读
│   │   └── scripts/           ← 可执行辅助（按需）
│   ├── galaxy/                ← 13 个 Galaxy 子技能
│   └── sc2data/               ← 6 个 SC2 Data 子技能
├── tools/                     ← Python 工具链
│   ├── sc2.py                 ← 统一入口：先跑这个
│   ├── sc2_checks.py          ← 静态规则注册表（RULES 是唯一真源）
│   ├── sc2_records.py         ← 本地开发记录（工作区外）
│   ├── package.py             ← 打包分发
│   ├── validate-mod.py        ← XML/Galaxy 静态校验
│   ├── sc2-catalog-query.py   ← catalog 图查询
│   └── schemas/sc2-xsd/       ← W3C XSD schema
└── DataEditorXML/             ← 游戏数据导出（写 XML 前查询）
```

> **开发记录不在仓库里。** 它们存放在工作区的 `sc2agent-records/<project>/`，
> 用 `python tools/sc2.py start` 读、`python tools/sc2.py record` 写——不进版本库，也不随分发包走。

### 外部路径（由 `agent-config.json` 解析）

| 用途 | 路径 |
|---|---|
| 主模组文件夹 | `<paths.mods_dir>/AeonOfIhanrii.SC2Mod` |
| 自定义脚本 | `<paths.mods_dir>/AeonOfIhanrii.SC2Mod/Base.SC2Data/Epi_Main.galaxy` |
| 战役地图文件夹 | `<paths.campaign_maps_dir>/` |

---

## 环境要求

- **操作系统：** Windows
- **Python：** 3.x（用于工具链）
- **StarCraft II：** 已安装，编辑器可用
- **TraeCode：** 已安装

---

## 快速开始

### 1. 让 AI 助手加载智能体

在 TraeCode 中对话时，告诉 AI：

```
加载 sc2-project-entry 技能，帮我 [具体任务]
```

AI 先读 `AGENTS.md`（红线与路由），需要路径时读 `agent-config.json`，再按需加载对应技能；工具入口是 `python tools/sc2.py`。

### 2. 常见启动指令示例

```
加载 sc2-project-entry 技能，帮我给单位 Stalker 添加一个闪现技能变体
```

```
加载 sc2-project-entry 技能，帮我检查当前模组的 XML 是否通过预飞行测试
```

```
加载 sc2-project-entry 技能，帮我把 paiur01 地图的攻击波数量按难度缩放
```

配置或切换项目主 Mod 时，可以直接提供 Components 文件夹路径：

```text
我的主 Mod Components 文件夹是：
E:\Program Files (x86)\StarCraft II\Mods\MyCampaign.SC2Mod

请运行 tools/init-project.py 完成项目初始化，递归检查本地依赖，
更新 agent-config.json；完成后核对 AGENTS.md 中的模组身份并运行配置验证。
不要让我手工编辑 JSON，也不要根据 Mods 目录内容猜测依赖。
```

更多可直接复制的对话模板见[安装与配置指南](setup.md#-与智能体对话示例)和[项目初始化配置](skills/sc2-project-entry/references/project-initialization.md#可直接复制的对话示例)。

### 3. 手动加载顺序（供参考）

1. 读取 `AGENTS.md` — 工作区硬性规则、模组身份与统一入口顺序
2. 读取 `agent-config.json` 与 `AGENTS.md` —— 解析实际项目路径并路由任务
3. 加载 `skills/sc2-project-entry/SKILL.md` — 统一入口（路由表+工具速查+实现流程）
4. 按任务加载对应技能，深度资料在其 `references/` 下按需加载

---

## 任务路由

收到任务后，AI 按以下路由加载对应根技能，再按子主题加载 `skills/galaxy/` 或 `skills/sc2data/` 下的子技能：

| 任务类型 | 加载根技能 | 子技能 |
|---|---|---|
| 项目入口、模组身份、硬性规则、任务路由 | `sc2-project-entry`（统一入口） | — |
| 编写/编辑 Galaxy 脚本 | `sc2-galaxy-scripting` | 见根技能内 Galaxy Sub-Skill Router 表（13 个 galaxy-*） |
| 编写/编辑模组 XML（单位/技能/行为/效果/武器/升级/验证器/足迹） | `sc2-catalog-xml` | 见根技能内 Catalog Sub-Skill Router 表（6 个 sc2data-*） |
| 深入 Actor 工作（CActorUnit/Action/Model/Beam/Sound，VFX/音频，贴图切换） | `sc2-actor-system` | — |
| 编辑地图触发器、GUI 动作接线、胜利/失败钩子 | `sc2-map-triggers` | — |
| Bank 系统、战役持久化、任务存档、解锁 | `sc2-bank-system` | — |
| 本地化（GameStrings/ObjectStrings/TriggerStrings/GameHotkeys）、字符串锚点 | `sc2-localization` | — |
| 运行工具、校验、部署 | `sc2-tools-validation` | — |
| 编辑器交接、问题生命周期、游戏测试 | `sc2-editor-handoff` | — |
| 攻击波缩放、AI 性格波迁移 | `sc2-attack-wave-scaling` | — |

完整路由表（含 13 个 galaxy-* 与 6 个 sc2data-* 子技能的逐项映射）见 [skills/sc2-project-entry/SKILL.md](skills/sc2-project-entry/SKILL.md)。

---

## 工具速查

所有工具路径均相对于当前智能体工作区根目录；实际 SC2、Mods 与地图路径由 `agent-config.json` 解析。

| 需求 | 命令 |
|---|---|
| 提交前预飞行测试（当前完整依赖链约 20s） | `python tools/test-suite.py` |
| XML schema + Galaxy 语法校验 | `python tools/validate-mod.py` |
| 查询单位/技能/数据链 | `python tools/sc2-catalog-query.py <cmd>` |
| 重建 catalog 图数据库 | `python tools/build-sc2-catalog-graph.py` |
| 部署模组到 SC2 安装目录 | `python tools/deploy-mod.py` |
| 检查/填充 GameStrings 锚点 | `python tools/audit-gamestrings-anchors.py --fill` |
| 审计命令卡完整性（保留旧文件名） | `python tools/audit-actor-and-card-integrity.py` |
| 校验技能 SKILL.md frontmatter | `python tools/audit-skill-frontmatter.py` |
| 提取游戏测试错误与日志 | `python tools/extract-playtest-bugreport.py` |
| 检查文档链接 | `python tools/check-doc-links.py` |

### 攻击波工具

| 需求 | 命令 |
|---|---|
| 检查攻击波数量（不修改） | `python skills/sc2-attack-wave-scaling/scripts/wrap_attack_wave_counts.py "<map_path>/Triggers"` |
| 应用攻击波数量包装 | `python skills/sc2-attack-wave-scaling/scripts/wrap_attack_wave_counts.py "<map_path>/Triggers" --apply` |
| 校验所有数量已包装 | `python skills/sc2-attack-wave-scaling/scripts/wrap_attack_wave_counts.py "<map_path>/Triggers" --check` |

---

## 常见场景示例

### 场景 1：给单位添加新技能变体

```
加载 sc2-project-entry 技能，帮我给 Stalker 添加一个闪现技能变体，
cd 设为 10 秒，需要本地化中文名称"暗影闪现"
```

AI 会：
1. 加载 `skills/sc2-project-entry/SKILL.md` 获取任务路由与工具速查
2. 路由到 `skills/sc2-catalog-xml/SKILL.md`
3. 用 `tools/sc2-catalog-query.py` 查询 Stalker 现有技能链
4. grep `DataEditorXML/` 确认字段名和 ID
5. 用项目独立 ID 创建变体（前缀 `libEpi_`）
6. 同步检查生产入口、命令卡、Actor、本地化
7. 运行 `tools/test-suite.py` 静态校验
8. 提示你在 SC2 编辑器中打开/保存地图

### 场景 2：编写 Galaxy 脚本

```
加载 sc2-project-entry 技能，帮我在 Epi_Main.galaxy 中添加一个函数，
根据难度返回攻击波数量倍率
```

AI 会：
1. 加载 `skills/sc2-project-entry/SKILL.md` 获取任务路由
2. 路由到 `skills/sc2-galaxy-scripting/SKILL.md`
3. 确认语法约束（变量提升、无 ++/--、用 while 不用 for）
4. 使用 `libEpi_` 函数前缀
5. 确保 include 在 `Lib67AA1763_h` 之后
6. 运行 `tools/validate-mod.py` 校验

### 场景 3：地图攻击波缩放

```
加载 sc2-project-entry 技能，帮我把 paiur01 地图的攻击波数量按难度缩放
```

AI 会：
1. 加载 `skills/sc2-project-entry/SKILL.md` 获取任务路由
2. 路由到 `skills/sc2-attack-wave-scaling/SKILL.md`
3. 确认地图未在 SC2 编辑器中打开
4. 运行 `wrap_attack_wave_counts.py`（不带 --apply）检查目标
5. 确认配置匹配活动依赖
6. 用 --apply 原子化应用
7. 用 --check 校验
8. 提示你在编辑器中打开/保存地图以重新生成 MapScript.galaxy

### 场景 4：Bank 存档系统

```
加载 sc2-project-entry 技能，帮我设计 Bank 存档，
记录玩家完成的任务、解锁单位和难度
```

AI 会：
1. 加载 `skills/sc2-project-entry/SKILL.md` 获取任务路由
2. 路由到 `skills/sc2-bank-system/SKILL.md`
3. 参考 `skills/sc2-bank-system/references/bank-system.md` 和 `skills/sc2-bank-system/references/galaxy-bank.md`
4. 设计 section 和 key 结构
5. 编写 Galaxy 读写函数（`libEpi_` 前缀）
6. 配置预加载

---

## 硬性规则

以下规则不可协商，违反会导致编译/链接/游戏运行失败：

1. **编辑范围：** 按当前配置 `source_mode` 选择实际编辑源。绝不编辑 `publish/` 文件夹。
2. **Galaxy 源真值：** 从当前 Triggers 核对手写源码和 include 顺序，只修改手写源码或可编辑触发器。具体Library、Bank及前缀来自项目证据，绝不手改生成Galaxy。
3. **地图调用必须用 GUI 动作：** 地图调用必须使用 GUI 动作，不能用 Custom Script。SC2 链接器会在地图未调用 GUI 动作时静默丢弃模组库。
4. **目录/XML：** 写 XML 前先运行 `python tools/sc2-catalog-query.py` 或 grep `DataEditorXML/`。字段名和 ID 区分大小写。提交前运行 `python tools/test-suite.py`。
5. **XML 仅 ASCII：** XML 注释和属性值只用 ASCII 字符。
6. **禁用 XML 模式：** `AbilAutoCmd` 按钮类型无效；`CBehaviorBuff` 不支持 `InitEffect`；纯注释 catalog 会被解析器拒绝。
7. **Galaxy 语法：** 局部变量必须提升到函数体最顶部。不支持 `++`/`--` 和 `for(...)` 循环——用 `i += 1;` 和 `while(...)`。include 指令用无扩展名的相对路径。
8. **编辑器交接：** 地图必须由用户保存为 Components。编辑器保存后运行 `python tools/audit-gamestrings-anchors.py --fill`。

---

## 智能体限制

智能体无法完成以下操作，必须由用户在 SC2 编辑器中手动完成：

- 打开和保存 Blizzard 地图（需要 SC2 账号登录）
- 地形、区域、装饰物、寻路
- 过场动画和电影
- 将模组/地图保存为 Components（用户操作）

---

## 故障排除

### 工具报告 `os error 206` 或字符串替换错误

参考 `skills/sc2-project-entry/references/agent-context-efficiency.md` 中的恢复指南。使用精确文本替换、保留编码/换行、按受影响范围运行 `python tools/test-suite.py --scope tools|docs|mod` 或对应回归。

### 编辑器打开地图加载了模组而非原版

**绝对不要**在项目模组作为外部 override 激活时打开地图——会加载模组版本而非原版。

### 添加依赖

地图 → Modules → Dependencies → `<ModName>.SC2Mod`

### 智能体无法读取本地知识库

确认：
1. TraeCode 已安装且工作目录设置为本智能体工作区根目录
2. `AGENTS.md` 存在
3. `skills/` 目录下有 10 个根技能子目录 + `skills/galaxy/`（13 个）+ `skills/sc2data/`（6 个）
4. `AGENTS.md` 存在
5. `tools/test-suite.py` 可执行（Python 3.x 已安装）

## 独立使用

在 AI 工具中打开本目录，读取 [AGENTS.md](AGENTS.md) 与 [项目入口技能](skills/sc2-project-entry/SKILL.md)。无需安装其他智能体工作区。

```text
选择 ../StarCraft II/Mods/MyCampaign.SC2Mod 作为开发项目。
调查 Unit:MyMarine 的武器、生产条件及升级影响，本次只读。
```

```text
将当前项目 Unit:MyMarine 的未升级基础生命调整为120。
核对本地覆盖、父对象和活动依赖，只修改实际开发源并验证相关组件。
暂不部署，给出编辑器与游戏验证步骤。
```

按需执行与验收见 [工作区规则](AGENTS.md#独立工作区与按需执行)。本工作区提供开发所需本地化键维护和字符串锚点检查；完整汉化、翻译知识库、自动 UI 文本提取及百科产品不属于本工作区能力。
