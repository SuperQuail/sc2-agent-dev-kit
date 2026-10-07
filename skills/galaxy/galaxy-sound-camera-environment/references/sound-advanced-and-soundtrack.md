# 声音 —— 进阶与配乐

挂在单位上的声音、最近播放句柄、阻塞式等待、按声音长度计时，以及背景音乐轨。

## 声音 —— 进阶

```galaxy
// Play sound attached to a unit (follows it; different param order from basic variant)
sound lv_snd = SoundPlayOnUnit(
    SoundLink("UnitDeath", -1),
    PlayerGroupAll(),    // player group (NOT single player like basic variant)
    lv_unit,
    0.0,                 // min distance
    100.0,               // volume
    0.0                  // fade-in duration
);

// Get the last played sound (for stopping/waiting)
sound lv_last = SoundLastPlayed();

// Stop with optional fade
SoundStop(lv_last, true);   // true = fade out

// Wait for a sound to finish (blocks trigger thread)
SoundWait(lv_last, 0.0, c_soundOffsetEnd);

// Time a Wait() to an exact sound length
Wait(SoundLengthSync(SoundLink("MySpeech", -1)), c_timeGame);
```

### 配乐（背景音乐）

```galaxy
// Start, pause, and restore background music tracks
SoundtrackDefault(PlayerGroupAll(), c_soundtrackCategoryMusic, "TrackName",
    c_soundtrackCueAny, c_soundtrackIndexAny);
SoundtrackPlay(PlayerGroupAll(), c_soundtrackCategoryMusic, "TrackName",
    c_soundtrackCueAny, c_soundtrackIndexAny, false);

// Pause / resume music (e.g. during a cinematic)
SoundtrackPause(PlayerGroupAll(), c_soundtrackCategoryMusic, true,  false);  // pause
SoundtrackPause(PlayerGroupAll(), c_soundtrackCategoryMusic, false, false);  // resume
```
