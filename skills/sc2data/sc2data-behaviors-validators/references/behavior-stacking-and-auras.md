# 行为叠加与光环

叠加数量与叠加模式，以及用「行为 + 搜索 + 施加」实现光环的模式。

## 行为叠加

```xml
<CBehaviorBuff id="PoisonStack">
    <MaxCount value="10"/>          <!-- allow up to 10 stacks -->
    <Stack value="Duration"/>       <!-- stacking resets duration each application -->
</CBehaviorBuff>
```

叠加模式：`Duration`（重置计时器）、`None`（只加计数）、`Any`（时长与计数各取最大）。

## 光环模式（行为 + 搜索效果 + 施加）

光环由行为上的周期效果实现：

```xml
<!-- 1. The aura behavior on the source unit -->
<CBehaviorBuff id="HealingAuraBehavior">
    <Duration value="0"/>
    <PeriodicEffectArray index="0" value="HealingAuraSearch" Period="1"/>
</CBehaviorBuff>

<!-- 2. A search effect finds nearby allies -->
<CEffectSearch id="HealingAuraSearch">
    <AreaArray index="0" Radius="3" Effect="HealingAuraApply"
               TargetFilters="Alive;Self,Enemy,Neutral,Dead"/>
</CEffectSearch>

<!-- 3. Apply a heal behavior to each found unit -->
<CEffectApplyBehavior id="HealingAuraApply">
    <Behavior value="HealingAuraHeal"/>
</CEffectApplyBehavior>
```
