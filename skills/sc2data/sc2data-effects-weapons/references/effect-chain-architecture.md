# 效果链架构

武器或技能的根效果如何经由集合、搜索与弹体逐级展开，以及为什么第一个效果决定目标选取类型。


```
CWeapon
  └─ Effect ──► CEffectSet
                  ├─► CEffectDamage             (subtract HP)
                  ├─► CEffectApplyBehavior      (add buff/debuff)
                  ├─► CEffectCreateUnit         (spawn unit)
                  ├─► CEffectSearch / EnumArea  (AoE spread)
                  │     └─► CEffectDamage
                  └─► CEffectLaunchMissile ──► CEffectDamage
```

技能或武器只触发**一个根效果**。这个根几乎总是 `CEffectSet`，由它串起多个阶段。

### 首效果决定目标选取

**技能效果链中的第一个效果**决定该技能的目标选取类型。要得到可指定目标的技能（点单位或点地面），根效果必须是接受并使用目标的类型。把 `CEffectIssueOrder` 放在**第一位**会让技能无法指定目标——请把 `CEffectSet` 放在最前，或确保排序已考虑这一点。
