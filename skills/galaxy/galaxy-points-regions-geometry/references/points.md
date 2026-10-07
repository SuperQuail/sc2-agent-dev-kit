# 点

创建、移动、读取与测量 `point` 值，以及能在某个点上做的地形与寻路查询。

## 点

`point` 是 2D（带高度则为 3D）地图坐标。

### 创建点

```galaxy
point lv_p = Point(16.5, 32.0);                          // x, y
point lv_p3 = libNtve_gf_PointFromXYZ(16.5, 32.0, 4.0); // x, y, z

// Named point from map (placed in editor)
point lv_named = PointFromName("StartLocation1");
```

### 修改点

```galaxy
// Move a point in place
PointSet(lv_p, 20.0, 40.0);

// Set facing angle on a point
PointSetFacing(lv_p, 90.0);

// Set height
PointSetHeight(lv_p, 2.0);
```

### 读取点的分量

```galaxy
fixed lv_x = PointGetX(lv_p);
fixed lv_y = PointGetY(lv_p);
fixed lv_facing = PointGetFacing(lv_p);
fixed lv_height = PointGetHeight(lv_p);
```

### 点之间的距离与角度

```galaxy
fixed lv_dist  = DistanceBetweenPoints(lv_a, lv_b);
fixed lv_angle = AngleBetweenPoints(lv_a, lv_b);  // degrees, from lv_a toward lv_b
```

### 偏移与移动

```galaxy
// Create new point offset by x/y
point lv_offset = PointWithOffset(lv_origin, lv_dx, lv_dy);

// Move in a direction (polar)
point lv_ahead  = PointWithOffsetPolar(lv_origin, lv_distance, lv_angleDegrees);

// Step from A toward B by a distance
point lv_step   = libNtve_gf_PointOffsetTowardsPoint(lv_a, lv_b, lv_dist);

// Add Z offset (above terrain)
point lv_above  = libNtve_gf_PointWithZOffset(lv_p, 2.5);

// Point at the facing angle of another point
point lv_front  = libNtve_gf_PointFacingAngle(lv_p, lv_range);
```

### 在点上的地形查询

```galaxy
fixed lv_h      = WorldHeight(lv_p);            // terrain height
string lv_tex   = TerrainTexture(lv_p);         // terrain texture id
int    lv_cliff = CliffLevel(lv_p);             // cliff height level (int)
fixed  lv_cliff2 = PointPathingCliffLevel(lv_p); // cliff level (fixed)
bool   lv_pass  = PointPathingPassable(lv_p);   // is ground passable?
bool   lv_conn  = PointPathingIsConnected(lv_a, lv_b); // are pts connected?
int    lv_cost  = PointPathingCost(lv_a, lv_b); // pathing cost
```
