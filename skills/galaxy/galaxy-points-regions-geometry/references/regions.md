# 区域

预定义区域与代码构造的区域、它们的测量、包含测试、随机点，以及区域进入/离开事件。

## 区域

`region` 是区域形状（矩形、圆、多边形），可以放在编辑器里，也可以用代码创建。

### 常用预定义区域

```galaxy
region lv_all  = RegionEntireMap();     // the whole map
region lv_play = RegionPlayableMap();   // playable area only
region lv_emp  = RegionEmpty();         // zero-size region
region lv_named = RegionFromName("MySpawnRegion");
```

### 用代码创建区域

```galaxy
// Rectangle (from two corners)
region lv_rect = RegionRect(lv_minPt, lv_maxPt);

// Circle
region lv_circle = RegionCircle(lv_center, 8.0);

// Add/subtract regions
RegionAdd(lv_dest, lv_addRegion);
RegionRemove(lv_dest, lv_removeRegion);
```

### 区域测量

```galaxy
point lv_center = RegionGetCenter(lv_region);
point lv_min    = RegionGetBoundsMin(lv_region);
point lv_max    = RegionGetBoundsMax(lv_region);
fixed lv_width  = PointGetX(lv_max) - PointGetX(lv_min);
fixed lv_height = libNtve_gf_HeightOfRegion(lv_region);
```

### 点在区域内的判定

```galaxy
bool lv_in = RegionContainsPoint(lv_region, lv_point);
```

### 区域内的随机点

```galaxy
point lv_rand = RegionRandomPoint(lv_region);
```

### 区域事件

```galaxy
// Fire when a unit enters or leaves
TriggerAddEventUnitRegion(myTrigger, null, lv_region, true);   // enter
TriggerAddEventUnitRegion(myTrigger, null, lv_region, false);  // leave

// Inside handler
unit   lv_ent = EventUnit();
region lv_reg = EventUnitRegion();  // which region triggered
```
