# 玩家镜头、游戏属性与难度

按玩家生效的镜头调用、大厅属性读取，以及任务难度。

## 镜头控制

```galaxy
// Pan camera to point
CameraPan(lv_player, lv_point, 0.0, -1, 10.0, false);
// (player, point, distance, yaw, pitch, sync)

// Snap camera instantly
CameraSetTarget(lv_player, lv_point, 0.0, -1, 10.0, false);
```

---

## 游戏属性（大厅选项）

用于读取大厅里配置的游戏设置：

```galaxy
// Returns the string value of the attribute for this player/game
string lv_val = GameAttributeGameValue("attrId");

// Example: select difficulty by attribute
bool isHardMode = (GameAttributeGameValue("Difficulty") == "Hard");
```

## 难度

```galaxy
// Get the current mission difficulty as an integer
// 1 = Casual, 2 = Normal, 3 = Hard, 4 = Brutal
int lv_diff = PlayerDifficulty(1);

// Branch on it
if (PlayerDifficulty(1) >= 3) {
    // hard or brutal
}
```
