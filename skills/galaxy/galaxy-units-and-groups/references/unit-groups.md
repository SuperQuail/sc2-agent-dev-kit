# 单位组

单位组的创建、`UnitFilter` 掩码、修改、查询函数，以及编辑器的从末尾遍历模式。

## 单位组

### 创建单位组

```galaxy
// Empty group
unitgroup lv_group = UnitGroupEmpty();

// Filtered group from the game world
unitgroup lv_enemies = UnitGroup(
    null,                       // null = any unit type (or "Marine" for specific)
    1,                          // player
    RegionEntire(),             // region to search in
    UnitFilter(0, 0, (1 << c_targetFilterDead), 0),   // exclude dead
    0                           // max units, 0 = unlimited
);
```

### UnitFilter

```galaxy
// UnitFilter(requiresMask, excludeMask, requiresType, excludeType)
UnitFilter(0, 0, 0, 0)                              // no filter
UnitFilter(0, 0, (1 << c_targetFilterDead), 0)      // exclude dead
UnitFilter((1 << c_targetFilterEnemy), 0, 0, 0)     // require enemy

// Common filter constants
c_targetFilterDead
c_targetFilterAlly
c_targetFilterEnemy
c_targetFilterSelf
c_targetFilterGround
c_targetFilterAir
c_targetFilterStructure
c_targetFilterWorker
c_targetFilterHero
```

### 修改单位组

```galaxy
UnitGroupAdd(lv_group, lv_unit);
UnitGroupRemove(lv_group, lv_unit);
UnitGroupAddUnitGroup(lv_group, lv_otherGroup);
UnitGroupClear(lv_group);
```

### 查询单位组

```galaxy
int  lv_count  = UnitGroupCount(lv_group, c_unitCountAll);
bool lv_has    = UnitGroupHasUnit(lv_group, lv_unit);
unit lv_first  = UnitGroupUnit(lv_group, 1);        // 1-based index
unit lv_random = UnitGroupRandomUnit(lv_group, c_unitCountAll);
unit lv_close  = UnitGroupClosestToPoint(lv_group, lv_point);
bool lv_dead   = libNtve_gf_UnitGroupIsDead(lv_group);
point lv_center = UnitGroupCenterOfGroup(lv_group);

// Filter by player
unitgroup lv_filtered = UnitGroupFilterPlayer(lv_group, 2, 0);

// Filter by unit type from an existing group (version = 0)
unitgroup lv_marines = UnitGroupFilter("Marine", lv_player, lv_sourceGroup, UnitFilter(0,0,0,0), 0);

// Get all units created by the last batch UnitCreate call (e.g. after creating 5 at once)
unitgroup lv_batch = UnitLastCreatedGroup();

// Convert single unit to group
unitgroup lv_single = libNtve_gf_ConvertUnitToUnitGroup(lv_unit);

// Unit count constants
c_unitCountAll
c_unitCountAlive
c_unitCountDead
```

### 遍历单位组（从末尾开始模式）

编辑器生成的是从末尾开始的遍历，以便在循环中安全移除单位：

```galaxy
int lv_u = UnitGroupCount(lv_group, c_unitCountAll);
unit lv_cur;
for (; lv_u > 0 ; lv_u -= 1) {
    lv_cur = UnitGroupUnitFromEnd(lv_group, lv_u);
    if (lv_cur == null) { continue; }
    // act on lv_cur ...
}
```
