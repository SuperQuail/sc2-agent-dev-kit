# SC2 触发器 XML 参考

> 由 `Core.SC2Mod/base.sc2data/triggerlibs/nativelib.triggerlib`（3,196 个 native 函数，库 `Ntve`）生成，另加在外部生成触发器 XML 时经验证的工作流笔记。

生成或编辑 `.SC2Map` / `.SC2Mod` 触发器 XML 时先看本页。快速查函数 ID 与参数 ID 用 [triggers-cheatsheet.md](triggers-cheatsheet.md)。完整 native 查询表用 [triggers-native-functions.md](triggers-native-functions.md)。
外部触发器编辑器入门见 [../../sc2-project-entry/references/external-sc2-resources.md](../../sc2-project-entry/references/external-sc2-resources.md)。

## 快速定位

SC2 触发器以 XML 存放在 `<Map>/Triggers`（地图）或包在 `<Library Id="XXXXXXXX">` 里的
`<Mod>/Triggers`（模组），显示名另放一个纯文本的 `<Map>/enUS.SC2Data/LocalizedData/TriggerStrings.txt`。
每个元素都有 8 字符十六进制 Id（例如 `A1B2C3D4`）。
引用用 `Type+Library+Id`；地图本地引用省略 `Library`，
跨库引用要带上（例如原生自带函数用 `Library="Ntve"`）。

推荐流程：在外部生成可用的触发器 XML，再让 SC2 编辑器像人手工搭的那样打开它。
**非平凡逻辑避免用 `customscriptaction`** —— `<ScriptCode>` 块里的裸 Galaxy
在编辑器里难读，而且绕过类型检查器。改为搭原生 FunctionCall 树。

对 AI 性格攻击波迁移，纯 GUI 是强制的：用
`skills/sc2-attack-wave-scaling/scripts/convert_custom_ai_to_gui.py`，只交付
`FunctionCall`/`Param` 树。兼容用的脚本阶段辅助绝不能直接用在用户的地图上。
编辑器交接前先核验参数归属的库：LotV 难度参数定义用 `Library="Lotv"`；默认配置档的
`AttackWaveModifier` 输入 `4D4D221F` 用 `Library="67AA1763"`。
迁移出来的性格触发器是无事件的 worker。用普通的、不等待的 GUI `Run Trigger` 动作把它们
接进地图既有的 AI 编排触发器（通常叫 `Start AI`）。不要给每个迁移性格单独挂地图初始化
事件。不要为触发器编辑器未暴露的 AI native 造空的 `FlagNative` 包装；编辑器可能保留
视觉定义，却在生成的 Galaxy 里静默省略它的调用。

参考文件（路径相对于解包出来的 Core 模组 / SC2 安装目录）：
- native catalog XML：`core.sc2mod/base.sc2data/triggerlibs/nativelib.triggerlib` —— 下面记录的 3,196 个函数的来源。
- Galaxy native 签名：`core.sc2mod/base.sc2data/triggerlibs/natives.galaxy` —— Galaxy 语法下的类型签名（生成 `<ScriptCode>` 时有用）。
- 示例对话框、贴图与布局用的自带 SC2 模组：`{core,liberty,swarm,void}.sc2mod/` —— 用 MPQ 工具（例如 MPQEditor、CASCExplorer）从 SC2 安装目录解包。
- 依赖解析方面，SC2 编辑器可能需要 SC2 安装目录 `Mods/` 下有一份模组副本。把本工作区的组件文件夹保持为真源，并为项目记录任何外部复制/同步步骤。
- 自带单位头像贴图：`Assets\Textures\btn-{unit|building|ability|upgrade}-{terran|zerg|protoss}-{name}.dds`（例如 `btn-unit-terran-marine.dds`）。每张 SC2 地图都可用。

## GUI 编辑器定位

SC2Mapster 的触发器总览对人工编辑器操作有用；本页专注 XML 形态。在 GUI 与 XML 之间来回时
要记住的关键概念：

- 地图/模组运行时，触发器文件会被转换成 Galaxy。外部 XML 改动之后，如果行为异常，检查生成的 `MapScript.galaxy`。
- 来自依赖的库触发器在触发器编辑器库面板里可见，其事件/条件匹配时可以运行。需要压制某个导入/库触发器时，用触发器的开/关逻辑，不要以为右键禁用能跨库生效。
- 局部变量按触发器执行/线程独立。全局变量在触发器和线程之间共享。
- 自定义定义可以把重复的 GUI 动作树藏在具名动作/函数后面。当地图不应直接调用它们时，把模组内部辅助定义标为 `Internal`。
- preset 是带显示文本的枚举式整数。它们适合静态选项，但生成的 XML 仍然需要底层的 preset/库 ID。

---

## 元素结构规则

- `<FunctionDef>` 的主体 = 一串 `<FunctionCall>` 子元素
  （有序语句）。**不是** 直接在 FunctionDef 上写 `<ScriptCode>` ——
  那种形态只对带 `<FlagSubFunctions/>` 的 *子函数模板* 有效，
  例如 `ForEachMission`。
- `<FlagCall/>` = 该函数返回值（需要 `<ReturnType>`）。
  `<FlagAction/>` = 动作，返回 void。`<FlagEvent/>` = 事件注册。
- 局部变量：在 FunctionDef 内声明一个 `<Variable Type="Variable" Id="..."/>`
  引用，另加一个独立的顶层 `<Element Type="Variable">`，带
  类型与初始值。
- `<Param>` 元素里的参数传递 —— 以下之一：
  - 内联字面量：`<Value>X</Value> + <ValueType Type="int"/>`
    （或 `string`、`fixed`、`bool`）
  - 游戏链接（catalog 引用）：`<Value>Marine</Value> +
    <ValueType Type="gamelink"/> + <ValueGameType Type="Unit"/>`
  - 子调用：`<FunctionCall Type="FunctionCall" Id="..."/>`
  - 变量：`<Variable Type="Variable" Id="..."/>`（跨库加 `Library=`）
  - preset 枚举：`<Preset Type="PresetValue" Library="Ntve" Id="..."/>`
  - 父函数参数：`<Parameter Type="ParamDef" Id="..."/>`

### SubFunctionType 模式（最容易踩坑的那个）

有些 native 接受 *子动作*（then 分支、循环体、条件）。
子 FunctionCall 通过 `<SubFunctionType Library="Ntve" Id="..."/>` 声明自己的角色。
子动作住在父级的子元素列表里，**不是** `<Parameter>` 槽位里。

已验证示例：

- `IfThenElse`（Ntve `00000137`）—— 子类型：
  `00000003` = if（条件）、`00000004` = then、`00000005` = else。
- `PickEachUnitInGroup`（Ntve `C4DC760C`）—— body 子类型 `9441B8B5`；
  在 body 内用 `UnitGroupLoopCurrent`（Ntve `19CE733E`）取被选中的单位。
- **`Or`（Ntve `00000133`）与 `And`（Ntve `00000132`）** 用 **不同的**
  条件子类型 ID：`And` 条件 = `00000002`；`Or` 条件 = `00000001`。
  混用会在保存时静默剥离 Comparison。一律先在 nativelib 里读
  父级的 `<SubFunctionType>` 声明。

---

## 事件声明（最常见的静默失败）

事件 **不是** 声明成 `<Element Type="Event">` 兄弟元素。它们声明成
`<Element Type="FunctionCall" Id="X">`（与动作同形），
Trigger 通过 `<Event Type="FunctionCall" Id="X"/>` 引用它们。
编辑器保存时会静默剥离任何 `<Event Type="Event"/>` 引用以及任何
`<Element Type="Event">` 声明 —— schema 里根本没有这个类型。

**布局要求：** 事件 `<Element Type="FunctionCall">` 必须
紧跟在它所属 Trigger 的收尾 `</Element>` 之后（与兄弟元素相邻），不能放在文件后面。
如果被引用的 FunctionCall 离得很远（例如文件末尾），编辑器的 `_Init` 代码生成会静默
**丢掉** `TriggerAddEvent*(...)` 注册调用 —— 触发器看起来注册了，却永不触发。
读 `MapScript.galaxy` 来核验。

```xml
<Element Type="Trigger" Id="D23DC42E">
    <Event Type="FunctionCall" Id="859460FA"/>
    <Action Type="FunctionCall" Id="47FB5D16"/>
</Element>
<Element Type="FunctionCall" Id="859460FA">      <!-- event, immediately after -->
    <FunctionDef Type="FunctionDef" Library="Ntve" Id="6D565EB4"/>
    <Parameter Type="Param" Id="DurParam"/>
    <Parameter Type="Param" Id="TimeTypeParam"/>
</Element>
```

条件遵循同样的模式：`<Condition Type="FunctionCall" Id="X"/>`。

---

## 触发器参数 schema 与 Galaxy 类型映射

在星际争霸 II 的触发器编译器中，GUI 触发器参数类型直接映射到底层 Galaxy 基础类型。
生成或检查触发器 XML 与 Galaxy 绑定时：

| 触发器参数类型（`Type="..."`） | Galaxy 基础类型 | 引擎用法示例 |
|---|---|---|
| `gamelink`、`anygamelink` | `string` | 单位类型（`"Marine"`）、技能、武器、升级 |
| `catalogentry`、`catalogfieldpath`、`reference` | `string` | 动态 catalog 反射（`CatalogFieldValueSet`） |
| `filepath`、`modelanim`、`actormsg` | `string` | 资源路径、动画名、Actor 消息 |
| `charge`、`cooldown` | `string` | 共享技能充能/冷却链接 ID |
| `convcharacter`、`convline`、`conversationtag` | `string` | 战役剧情与传输引擎字符串 |
| `layoutframe`、`layoutframerel` | `string` | UI 布局框体路径字符串 |
| `userfield`、`userinstance` | `string` | 用户数据表与自定义实例 |
| `preset` | `int` | preset 枚举值（例如 `c_unitPropLife`） |
| `int`、`fixed`、`bool` | `int`、`fixed`、`bool` | 标准数值与布尔字面量 |
| `point`、`region`、`unit`、`playergroup`、`unitgroup` | 引擎对象句柄 | 空间与游戏性实体 |

### 元素标志位

触发器 XML 元素支持位域标志位，控制编译器与编辑器可见性：
- `Native`（`0x02`）：引擎 native 函数绑定。
- `FuncAction`（`0x04`）：动作定义（void 返回）。
- `FuncCall`（`0x08`）：有返回值的函数调用。
- `Event`（`0x10`）：事件注册函数。
- `Template`（`0x20`）：子函数动作模板。
- `CustomScript`（`0x100`）：原始自定义 Galaxy 脚本块。
- `SubFunctions`（`0x800`）：嵌套子动作的容器。
- `Hidden` / `Internal` / `Restricted`：编辑器可见性与作用域限制。
