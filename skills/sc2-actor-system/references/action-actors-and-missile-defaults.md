# 动作 Actor 与导弹默认值

对基于 `GenericAttack` 的 `CActorAction`，名字恰好是 `<ActionActorId>Missile` 的导弹 Actor 就是默认导弹绑定。在 XML 里它可能表现为显式的 `<Missile value="..."/>`，但 SC2 编辑器保存时可能删掉该字段，因为解析出的预载 token 已经是 `##id##Missile`。

## 满足以下全部条件时，把这次删除当正常现象

- 动作 Actor 用 `parent="GenericAttack"`，或另一个导弹 token 已解析为 `##id##Missile` 的父级。
- 预期导弹 Actor ID 恰好是动作 Actor ID 加上 `Missile`。
- `PreloadAssetDB.txt` 仍显示该动作 Actor 通过 `##id##Missile` 解析导弹槽位。

**不要** 把它推广到那些从父级继承了一个具名导弹的动作 Actor。例如基于 `MarauderAttackBase` 的动作 Actor 可能继承 `MarauderAttackMissile`；如果自定义弹体应该是 `<CustomActionId>Missile`，就保留显式的 `<Missile value="..."/>` 覆盖。
