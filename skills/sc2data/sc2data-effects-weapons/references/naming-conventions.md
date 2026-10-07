# 命名约定 — 效果、武器、升级

武器、根效果、伤害效果、搜索效果、施加行为效果与升级的 ID 命名约定。

## 命名约定

| 对象 | 约定 |
|---|---|
| 武器 id | `UnitNameWeapon` 或描述性名称（如 `MarineWeapon`） |
| 根效果 id | 与武器或技能 id 相同（如 `MarineWeaponEffect`） |
| 伤害效果 id | 根 + `Damage`（如 `MarineWeaponDamage`） |
| 搜索效果 id | 根 + `Search`（如 `SiegeSplashSearch`） |
| 施加行为效果 | `ApplyBehaviorName`（如 `ApplyLockedDown`） |
| 升级 id | `PascalCase`（如 `InfantryWeaponLevel1`） |
