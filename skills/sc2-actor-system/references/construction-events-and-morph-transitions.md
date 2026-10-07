# 单位建造事件与变形过渡

## 单位 Actor 的带索引建造事件

重定向继承自 `GenericUnitMinimal` 的事件数组的 `CActorUnit` 行，必须保留继承索引的含义：

- 索引 `4`：`UnitConstruction.<unit>.Start`（为建造/折跃表现创建单位 Actor）
- 索引 `5`：`UnitConstruction.<unit>.Finish`（结束建造表现）

**不要** 在任一索引上使用裸 `UnitConstruction.<unit>` term，也 **不要** 把 `.Start` 放到索引 5。后者会覆盖继承的完成处理器，可能留下两个常驻模型或一个回退折跃球。光有 `BuildModel` 修不好挪位的事件覆盖。编辑器往返之后，要针对这些确切后缀审计项目里所有 `CActorUnit` 行，因为编辑器可能把继承的带索引事件重新具象化出来。

## 变形过渡表现 Actor

项目自有的纯视觉变形过渡，优先用通过继承挂在 `_Selectable` 上的 `CActorModel parent="ModelAdditionNoAnims"`。在 `AbilMorph.*.Start` 创建它，在 `ActorCreation` 播放想要的变形动画，在变形结束时销毁它，并把 `ActorOrphan -> Destroy` 作为兜底加上。

避免把视觉叠加层作为 `CActorUnit parent="GenericUnitBaseMorphTransition"` 或 `CActorMissile` 导入。即使它的 `unitName` 指向一个哑变形 ID，从自定义单位的变形事件创建它也可能把它放进真实单位作用域，并报出 `More than one CActorUnit persisting in the same unit scope`。编辑器可能原样保留 XML；这个告警是运行时作用域问题，不是序列化问题。

对变体专属的变形视觉，确认被引用的 `CModel` 在当前依赖链里有显式的 `.m3` 资源路径。从非活动依赖复制来的精简行可能只含半径/缩放元数据，因为它的原始依赖把资源放在别处。那会导致变形不可见，或静默回退成通用原版模型。优先用带显式资源路径的项目自有模型 ID，再把过渡 Actor 指向该 ID。

一句话总结变形过渡的纯视觉 Actor：优先 `CActorModel parent="ModelAdditionNoAnims"`，变形开始创建，变形结束销毁，加上 `ActorOrphan → Destroy`。不要把视觉叠加层做成第二个常驻 Unit Actor。

