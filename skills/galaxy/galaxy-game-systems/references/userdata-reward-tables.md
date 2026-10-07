# UserData（catalog 驱动的奖励表）

UserData 表在 Galaxy Data 编辑器中定义，运行时读取。用它把设计数值（奖励、配置）从硬编码脚本里拿出来：

```galaxy
// Read from a UserData table by type identifier and field name
fixed baseExp     = UserDataGetFixed("KillRewards", unitType, "Exp", 1);
fixed baseBiomass = UserDataGetFixed("KillRewards", unitType, "Biomass", 1);
int enemyType     = UserDataGetInt("KillRewards", unitType, "EnemyType", 1);
int teamSizeMax   = UserDataGetInt("AcvReqSpeedruns", "P0D1", "TeamSizeMaxForSmall", 1);
```

第 4 个参数是实例索引（从 1 开始）。UserData 表在 Galaxy Data 编辑器中定义。

> 运行时 catalog / UserData 查询（含 5 参数的 `UserDataGet*` 形式与 catalog 字段访问）在 `galaxy-debug-data-catalog` 中。
