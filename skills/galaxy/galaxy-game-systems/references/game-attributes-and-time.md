# 游戏属性与游戏时间

## 游戏属性（模式判定）

```galaxy
// Read lobby options (set at game creation)
string lv_attr = GameAttributeGameValue("attrId");
// Example:
bool isHardMode = (GameAttributeGameValue("Difficulty") == "Hard");
```

## 游戏时间

```galaxy
fixed lv_elapsed  = GameGetMissionTime();   // seconds since map start
fixed lv_timeGame = TimerGetElapsed(lv_timer);

// Async delay in a trigger function
Wait(5.0, c_timeGame);     // 5 game seconds
Wait(1.0, c_timeReal);     // 1 real second
```
