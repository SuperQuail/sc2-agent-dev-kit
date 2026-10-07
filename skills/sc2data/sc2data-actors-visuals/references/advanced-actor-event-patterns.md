# 高级 Actor 事件模式

计时器、跨 Actor 信号、状态标志、事件次数上限、效果树隔离，以及升级/充能/冷却事件的前置条件。

## 高级事件模式

### 计时器模式 — 每个 Actor 自己的延时与循环

计时器完全活在 Actor 上。创建时设置，到期时响应：

```xml
<!-- Start a 3-second timer (with up to 1 second random variance) on creation -->
<On Terms="ActorCreation" Send="TimerSet MyTimer 3 1"/>

<!-- Respond when that timer fires, identified by the TimerName term -->
<On Terms="TimerExpired; TimerName MyTimer" Send="AnimPlay Attack"/>
```

`TimerSet <Name> <BaseDuration> <RandomRange>` —— 在基础时长上再加 0–RandomRange 秒的随机量。到期前取消用 `TimerStop MyTimer`。

### 信号模式 — 跨 Actor 事件

信号通过引用别名，从一个 Actor 向另一个 Actor 发送人造事件：

```xml
<!-- From a buff actor: tell the host unit actor to play a flash -->
<On Terms="ActorCreation" Send="Signal AbilityFlash ::Host"/>

<!-- In the main unit CActorUnit: listen for that signal -->
<On Terms="Signal AbilityFlash" Send="AnimPlay Flash"/>
```

信号绕过常规游戏事件系统，直接由发送者送到接收者。

### Status Set — Actor 状态机

用 `StatusSet` + `IsStatus` 追踪 Actor 内部状态，无需外部数据：

```xml
<!-- Track burrow state -->
<On Terms="Abil.BurrowDown.SourceFinish" Send="StatusSet Burrowed 1"/>
<On Terms="Abil.BurrowUp.SourceFinish"   Send="StatusSet Burrowed 0"/>

<!-- Conditionally act only when burrowed -->
<On Terms="WeaponStart.MyWeapon.AttackStart; IsStatus Burrowed 1" Send="AnimPlay BurrowedAttack"/>
```

### Cap — 限制事件在单个 Actor 生命周期内的触发次数

`Cap N` term 让事件在 Actor 一生中最多触发 N 次：

```xml
<!-- Play birth anim only once even if creation somehow fires again -->
<On Terms="ActorCreation; Cap 1" Send="AnimPlay Birth"/>
```

### From Effect Tree Descendant — 隔离每个效果实例的 Actor

同一个效果的多个实例同时运行时（例如多个持续效果），用 `FromEffectTreeDescendant` 把每个 Actor 的响应限制在它**自己**的效果树内：

```xml
<On Terms="Effect.TentacleHit; FromEffectTreeDescendant TentacleEffect" Send="AnimPlay Hit"/>
```

没有这个 term，该类型 Actor 的每个活动实例都会响应每一条匹配事件。

### 升级事件 — Affected Unit Array 前置条件

`Upgrade.UpgradeId.Add` Actor 事件**只有在单位类型位于该升级的 `Affected Unit Array`** 中时才会触发。如果实测中事件从不触发，就把单位加进该数组：

```xml
<CUpgrade id="MyUpgrade">
    <AffectedUnitArray value="MyUnit"/>   <!-- required for Upgrade actor event to fire on MyUnit -->
</CUpgrade>
```

### 技能充能/冷却 Actor 事件

充能与冷却 Actor 事件只有在技能上设置了对应标志时才会触发：

| 技能标志（`Shared Flags`） | 启用的 Actor 事件 |
|---|---|
| `Register Charge Event` | `Abil.MyAbility.ChargeStart`、`.ChargeUse`、`.ChargeExpire` |
| `Register Cooldown Event` | `Abil.MyAbility.CooldownStart`、`.CooldownExpire` |
