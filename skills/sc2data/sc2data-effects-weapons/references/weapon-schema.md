# 武器 schema（WeaponData.xml）

带注释的 CWeapon 条目与武器字段表（Effect、DisplayEffect、Range、Period、DamagePoint、BackswingPoint、Arc、TargetFilters）。

## 武器（`WeaponData.xml`）

文件：`Base.SC2Data/GameData/WeaponData.xml`

```xml
<CWeapon id="MyWeapon">
    <EditorCategories value="Race:Terran"/>
    <DisplayEffect value="MyWeaponDamage"/>     <!-- effect shown in tooltip for damage display -->
    <Effect value="MyWeaponEffect"/>            <!-- root effect to fire on attack -->
    <Range value="5"/>                          <!-- attack range -->
    <Period value="0.8608"/>                    <!-- attack period (seconds between attacks) -->
    <DamagePoint value="0.0"/>                  <!-- % of Period before damage applies (0–1) -->
    <BackswingPoint value="0.5"/>              <!-- % of Period before unit can move/turn -->
    <Arc value="360"/>                          <!-- attack arc in degrees -->
    <Flags index="AttackTargetMover" value="1"/>
    <TargetFilters value="Visible;Self,Ally,Neutral,Dead,Invulnerable"/>
</CWeapon>
```

### 武器关键字段

| 字段 | 用途 |
|---|---|
| `Effect` | 攻击时触发的根效果 |
| `DisplayEffect` | 用于提示条伤害显示的效果（可以与前者不同） |
| `Range` | 攻击距离 |
| `Period` | 攻击速度——每次攻击的秒数 |
| `DamagePoint` | 在 Period 动画的哪个位置结算命中（0.0–1.0） |
| `BackswingPoint` | 攻击后单位何时可以再次移动 |
| `Arc` | 方向性武器的锥形角度 |
| `TargetFilters` | 哪些目标可被攻击 |
