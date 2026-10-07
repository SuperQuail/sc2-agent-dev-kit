# Actor 别名与宏

引用别名（::Creator、::Target、_UnitMedium）与可复用的暴雪事件宏。

## 别名

别名让其他 Actor 用简写的类型名找到本 Actor：

```xml
<Aliases value="_UnitMedium"/>
```

`Terms` 中常用的系统别名：
- `::Creator` — 创建本 Actor 的那个 Actor
- `::Host` — 本 Actor 依附的单位
- `::Target` — 动作的目标
- `::Main` — 宿主单位的主 Actor
- `_Unit` — 任意单位 Actor（泛型）
- `_Structure` — 任意建筑 Actor
- `_UnitSmall` / `_UnitMedium` / `_UnitLarge` — 按体型分类

## 宏

宏在运行时展开成一组 `<On>` 条目。定义在 `CActorMacro` 条目中，按名引用：

```xml
<Macros value="ZergBurrowStandardAnimMacro"/>
```

暴雪为常见模式（钻地/出地、死亡、隐形等）提供了大量可复用宏。手写 `<On>` 事件之前，先到同种族/同类型的既有单位里找可用的宏。
