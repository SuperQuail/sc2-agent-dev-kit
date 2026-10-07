# 校验器在各处的实际效果

校验器返回假时，在效果、行为的 Disable 槽、行为的 Remove 槽、技能、Actor 项中分别会发生什么。


### 在效果中（决定是否施加）：

```xml
<CEffectDamage id="FinisherDamage">
    <Amount value="200"/>
    <ValidatorArray index="0" value="TargetBelowHalfHP"/>  <!-- only deal if below 50% HP -->
</CEffectDamage>
```

### 在行为中 — Disable（暂时休眠）：

校验器返回假期间，行为被抑制但仍留在单位身上：

```xml
<CBehaviorBuff id="AuraOnlyWhileMoving">
    <ValidatorArray type="Disable" index="0" value="TargetIsMoving"/>
</CBehaviorBuff>
```

### 在行为中 — Remove（永久剥离）：

校验器返回假时，行为被整个移除：

```xml
<CBehaviorBuff id="ExpiresWhenFullHP">
    <ValidatorArray type="Remove" index="0" value="TargetBelowHalfHP"/>
</CBehaviorBuff>
```

### 在技能中（决定是否可用）：

校验器返回假时技能置灰 / 无法施放：

```xml
<CAbilEffectTarget id="MyAbility">
    <ValidatorArray index="0" value="NotBurrowed"/>
</CAbilEffectTarget>
```

### 在 Actor Terms 中：

```xml
<On Terms="ActorCreation; ValidateUnit MyUpgrade" Send="AnimGroupApply Superior"/>
```
