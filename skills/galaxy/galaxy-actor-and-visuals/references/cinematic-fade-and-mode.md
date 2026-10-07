# 过场淡入淡出与过场模式

淡入/淡出黑屏、进入与退出过场模式、转场样式，以及镜头掠过辅助函数。

## 过场淡入淡出与过场模式

```galaxy
// Fade to black (and back)
CinematicFade(true, 1.5, c_fadeStyleLinear, Color(0.0, 0.0, 0.0), 1.0, PlayerGroupAll());
// (fadeIn, duration, style, color, alpha, players)

CinematicFade(false, 1.0, c_fadeStyleLinear, Color(0.0, 0.0, 0.0), 1.0, PlayerGroupAll());

// Full cinematic mode — correct param order: (bool onOff, playergroup, fixed duration)
libNtve_gf_CinematicMode(true, PlayerGroupAll(), 0.0);   // enter cinematic mode
libNtve_gf_CinematicMode(false, PlayerGroupAll(), 1.0);  // exit cinematic mode

// Check if a player is in cinematic mode
bool lv_inCine = libNtve_gf_PlayerInCinematicMode(lv_player);

// Global cinematic mode (affects all players, fixed seed)
libNtve_gf_GlobalCinematicSetting(true);
libNtve_gf_GlobalCinematicSettingFixedSeedOnOff(true, true);

// Cinematic transition style
libNtve_gf_SetCinematicTransitionStyle(libNtve_ge_CinematicTransitionStyle_Mission);
// Constants: libNtve_ge_CinematicTransitionStyle_Mission, _Story

// Swoosh camera to a location (player, dist1, dist2, point, duration)
libNtve_gf_SwooshCamera(lv_player, 10.0, 5.0, lv_point, 1.0);
```
