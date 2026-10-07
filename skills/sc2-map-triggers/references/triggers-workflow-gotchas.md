# SC2 触发器 XML 工作流陷阱

外部编辑触发器 XML 时经验证的工作流与 schema 坑。直接改 `Triggers` 文件之前先读本页。

## 工作流陷阱（关键——多数经过经验验证）

1. **外部编辑 Triggers 文件之前，先在编辑器里关闭地图。** 编辑器内存里的副本
   会在下次保存时覆盖外部改动。先备份 `.bak`。
2. **外部编辑之后，重新打开并切换任意一个触发器**（启用/禁用、改一条注释）再保存。
   这会强制 `MapScript.galaxy` 代码生成重新输出 `InitTriggers()` 并接上外部新增的
   触发器。没有这一下，触发器存在于树里、渲染也正常，但永不注册事件。这是最容易
   想不到的失败点。
3. **调试用 `UIDisplayMessage` 请用 Chat** —— preset `89CC0A21`（Chat）。
   战役式玩法中 `Subtitle`（`875889C8`）是隐藏的。
4. **有事件但零动作的触发器会让编辑器在保存时崩溃**（对模组文件验证过）。一律至少
   放一个动作——哪怕是一个指向 Comment 元素的占位 `<Action Type="Comment" Id="..."/>`。
   地图可能比模组宽松；安全规则是通用的。
5. **模组文件比地图更挑剔。** 生成全新触发器时，先在地图里打样、验证保存+测试，
   再迁进模组。跨库引用（`Library="<libId>"`）让模组级常量留在模组里，而地图级
   触发器可以引用它们。
6. **`<ScriptCode>` 块绕过编辑器的类型检查器。** 即使脚本引用了未定义符号或越界数组
   下标，保存也会成功。随后 Galaxy 运行时会在那一行静默中止正在执行的触发器。
   直接读 `MapScript.galaxy` 来诊断——那才是代码生成后的 Galaxy 源。
7. **ArraySize 的 XML 取值 vs 运行时大小：** `<ArraySize Dim="0" Value="N"/>`
   创建的是大小 **N+1** 的数组（索引 0..N）。写索引 N+1 会静默失败。对 33 项名单
   （索引 0..32），至少用 `Value="32"`。
8. **每个已声明的 Element 都需要一条匹配的 TriggerStrings 名字条目**，
   包括 FunctionDef 内的局部变量与 ParamDef。缺条目会让编辑器为 Galaxy 符号名
   编造一个垃圾哈希。内部元素很容易漏。
9. **`TriggerAddEventUnitCreated` 做「任意单位出生」不可靠** —— 在训练/生成/折跃入场
   之间触发不一致。改用 `TriggerAddEventUnitRegion`（Ntve `00000041`），区域设
   Entire Map、状态用默认 Enter —— 它统一捕获所有单位出现。
10. **不要靠玩家 ID 区间（如 `2..16`）过滤「是不是敌人」。** 地图在该区间里混有同盟
    和敌人。用 `PlayerIsEnemy`（Ntve `CA3AB9A9`，签名 `(source, target, relation)`，
    其中 `relation` 来自 preset `PlayerRelation` = `2D9EC843`）。
11. **`nativelib` 的参数默认值不会自动生效。** 即使某个 ParamDef 在 nativelib 里有
    `<Default Type="Param" .../>`，构造 `<FunctionCall>` 时也必须为每个已声明参数
    显式写出 `<Parameter Type="Param" Id="..."/>` —— 否则编辑器把该参数渲染成红色的
    「(No Value)」，编译失败。
12. **`NoUnit` 这个 PresetValue（Ntve `630EB901`）是坏的** —— 它的 `<Value>` 是
    `true`（bool）而不是 `null`。把 Unit 类型的值与 NoUnit 比较会生成 Galaxy
    `x != true`，编译不过。绕法：改用间接过滤（例如用 `ZoneOwner[i] == EnemyPlayer`
    而不是 `Building[i] != NoUnit`）。
13. **`Or` / `And` 用不同的条件子类型**（`Or` = `00000001`，
    `And` = `00000002`）。见上面的 SubFunctionType 章节。
14. **地图要能加载，`.version` 二进制戳必须存在。** 删掉它们会让加载以
    「Unable to copy file」失败。编辑器保存时会重新生成它们，但加载时需要它们在场。
    确实要重置就从同级地图复制——编辑器下次保存时会用当前哈希覆盖。
15. **Galaxy Custom Script 解析器的怪癖**（在 `<ScriptCode>` 块里）：
    - `for (init; cond; incr) { … }` —— 失败。用 `init; while (cond) { …; incr; }`。
    - 循环里的 `break;` —— 失败。在 while 条件里用哨兵变量。
    - `+=` / `-=` / `++` / `--` —— 失败。用显式的 `x = x + 1;`。
    - **每行前面加 4 个空格。** 触发器编辑器的 Custom Script 显示会剥掉每行前 4 个
      字符。不加前缀，行会渲染错乱（`unitgroup` → `group`，
      `while (…)` → `e (…)`）。Galaxy 忽略多余缩进，所以两边都安全。
    - **`//` 行注释可用**，但多连字符分隔线不行 —— Galaxy 的分词器把 `--` 当成
      不支持的递减运算符，即使在 `//` 注释里也一样。用单个连字符。
    - **长破折号与任何非 ASCII** —— 解析器只认 ASCII。去掉弯引号、长破折号、箭头。
    - **级联失败：** 任何 `<ScriptCode>` 解析错误都会中止 **整个文件** 的加载，并
      对下游所有内容报出「Orphaned trigger parameter」/「Empty trigger element
      reference」告警。真正的解析错误要去 `MapScript.galaxy` 里找。
16. **不存在的 Galaxy 常量：** `c_unitStateStructure` —— 改用
    `UnitTypeTestAttribute(UnitGetType(u), c_unitAttributeStructure)`。
    已验证存在：`c_messageAreaDirective`、`c_messageAreaChat`、`c_playerAny`、
    `c_unitCountAll`、`c_unitCountAlive`、`c_unitCreateIgnorePlacement`、
    `c_orderQueueReplace`、`c_anchorBottom`，以及完整的 `c_triggerControl*` 与
    `c_unitAttribute*` 家族。
17. **`AIStart` 是 `(int player, bool isCampaign, int apm)`** —— 不是旧文档里的
    `(player, bool restart, string script)` 签名。用 `AIStart(p, true, 200);`。
18. **`<ArraySize>` 必须嵌在 `<VariableType>` **里面**，不能放在它后面。**
    放错位置的 ArraySize 会创建标量变量而不是数组 —— 之后所有 `Set X[i]` GUI 动作
    都会静默失效。正确写法：
    ```xml
    <Element Type="Variable" Id="...">
        <VariableType>
            <Type Value="int"/>
            <ArraySize Dim="0" Value="6"/>
        </VariableType>
    </Element>
    ```
    二维数组用两个 `<ArraySize Dim="..."/>` 兄弟元素。
19. **自闭合的 `<Element Type="Comment" Id="…"/>` 会让编辑器在保存时崩溃。**
    Comment 必须有非空的 `<Comment>…</Comment>` 主体 **并且** 在 TriggerStrings 里
    有一条 `Comment/Name/X=…`。崩在保存，不在加载。
20. **把 Custom Script Action 当作自定义 FunctionDef 的主体不可靠** —— 即使去掉长破折号/
    递减运算符、加了 4 空格前缀，仍会出现看不懂的编译失败。绕法：把 Custom Script
    Action 直接放进 Trigger 主体，或者在自定义 FunctionDef 里只用纯 GUI 的 native
    FunctionCall。
21. **FunctionCall 元素同一时间只能有一个父引用。** 让两个 Param 指向同一个 FunctionCall
    Id（想「复用」子树）会绑定第一个，把第二个渲染成红色的「(No Value)」。孤儿子树
    仍会抢绑定 —— 删掉孤儿，或把子树复制成新 ID 而不是共享。
22. **`TriggerStrings.txt` 是纯文本 —— 不要做 HTML 实体转义。**
    写 `Init &amp; Scaffolding` 会在编辑器里原样显示成 `Init &amp;
    Scaffolding`。用原始字符。（在 Triggers XML 自身内部，属性值里的特殊字符
    **确实** 需要实体。）
23. **`ArithmeticInt`（`00000128`）与 `ArithmeticReal`（`00000129`）
    的 ParamDef ID 不同**，尽管三槽形态一模一样：
    - ArithmeticInt：`00000205` val1 int、`00000206` op、`00000207` val2 int。
    - ArithmeticReal：`00000208` val1 fixed、`00000209` op、`00000210` val2 fixed。
    - op preset 取值共享：`00000085` = `+`、`00000086` = `-`、
      `00000087` = `*`、`00000088` = `/`。按上下文的期望类型来选。
24. **`Repeat Action Forever` 会代码生成成 `while (true)`。** 它之后
    的动作只有在循环遇到显式 `break` 时才执行；`return` 会退出整个触发器。
    对于调用方必须继续跑的长驻监视器，把循环放进一个单独的无事件触发器，并不等待地
    运行它。循环里保留一个 `Wait` 以免撞上执行上限。暴雪生成的示例两种模式都有：
    `paiur02` 的 `Group - Burrowed Zerg` 在后续动作之前 break，而 `paiur03` 在继续
    调用方之前用 `TriggerExecute(..., true, false)` 跑 `Deploy Pylon Units Powered`。
25. **显示/隐藏与暂停是彼此独立的单位状态。** 显示/隐藏改的是
    `c_unitStateHidden`；暂停改的是 `c_unitStatePaused`。如果单位既要不出现在表现里
    又要停止活动，两个状态都要设，之后两个都要恢复。暴雪的 `pshakuras02` 开场把隐藏组
    与暂停组分开维护，并各自独立恢复。

---

## ID 生成策略

每一轮生成挑一个可辨识的 4 或 5 字符十六进制前缀（任何不太可能与既有 ID 冲突的十六进制
串，例如 `A1B2C…`、`DEAD0…`）。先在现有 Triggers 文件里 grep 该前缀确认不冲突。
用后缀区间让元素树可读：例如触发器的 `<prefix>100..1FF` 给事件链，
`<prefix>200..2FF` 给分支 1，`<prefix>300..3FF` 给分支 2。

---
