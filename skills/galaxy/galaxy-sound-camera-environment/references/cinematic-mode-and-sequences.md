# 过场模式、序列与触发队列

完整的战役过场管线，以及串行化叙事序列的触发队列。

## 过场模式与序列

战役地图使用的完整过场管线：

```galaxy
// 1. Enter cinematic mode (letterbox + disable UI)
libNtve_gf_CinematicMode(true, PlayerGroupAll(), 0.5);   // (enable, players, transition time)
// Alternate: use GlobalCinematicSetting if letter-boxing all players at once
libNtve_gf_GlobalCinematicSetting(true);

// 2. Set fullscreen mode (removes game HUD completely)
UISetMode(PlayerGroupAll(), c_uiModeFullscreen, 0.0);

// 3. Register a skip key (allows player to jump to end)
TriggerSkippableBegin(PlayerGroupAll(), 0, null, true, false);

// 4. Cinematic fade in/out
CinematicFade(true,  1.5, c_fadeStyleNormal, ColorWithAlpha(0, 0, 0, 0), 0.0, true);  // fade in from black
CinematicFade(false, 1.0, c_fadeStyleNormal, ColorWithAlpha(0, 0, 0, 0), 0.0, true);  // fade to black

// 5. Close the skippable block (must match every TriggerSkippableBegin)
TriggerSkippableEnd();

// 6. Restore HUD at end of cinematic
UISetMode(PlayerGroupAll(), c_uiModeGame, 0.0);
libNtve_gf_CinematicMode(false, PlayerGroupAll(), 0.5);
libNtve_gf_GlobalCinematicSetting(false);
```

## 任务触发队列

触发队列把长时间运行的过场/叙事序列串行化，避免互相重叠。

```galaxy
// Enter / exit the queue (wraps around a cinematic trigger body)
TriggerQueueEnter();
// ... all the cinematic steps ...
TriggerQueueExit();

// Pause / resume queue (blocks next trigger from starting)
TriggerQueuePause(true);
TriggerQueuePause(false);

// Discard pending queue items
TriggerQueueClear(c_triggerQueueRemove);

// Check if the queue is empty (no pending triggers)
bool lv_done = TriggerQueueIsEmpty();
```
