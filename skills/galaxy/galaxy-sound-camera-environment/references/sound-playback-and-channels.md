# 声音播放、通道与音乐

按 link 在点位或对玩家播放声音、声音通道与音量，以及简单的音乐播放。

## 声音

### 播放声音

```galaxy
// Play a sound by SoundLink (preferred — uses data-defined volumes)
sound lv_snd = SoundPlayForPlayer(
    SoundLink("UI_ButtonClick", -1),  // link name + index (-1 = any)
    lv_player
);

// Play at a 3D world position — single player
sound lv_snd2 = SoundPlayAtPointForPlayer(
    SoundLink("Zerg_Hydralisk_Death1", -1),
    lv_player,
    lv_point,
    0.0,    // min distance
    30.0,   // max distance
    100.0   // volume
);

// Play at a 3D world position — playergroup (use this for campaign / multi-player maps)
sound lv_snd3 = SoundPlayAtPoint(
    SoundLink("Zerg_Hydralisk_Death1", -1),
    PlayerGroupAll(),
    lv_point,
    0.0,    // min distance
    100.0,  // volume
    0.0     // fade-in duration
);

// Play attached to a unit (follows it) — playergroup form (6 params)
// See Sound – Advanced section for the correct signature

// Wait for sound to complete (blocks trigger thread)
SoundWait(lv_snd);

// Stop a playing sound
SoundStop(lv_snd, true);    // true = fade out
SoundStopAllTriggerSounds(true, PlayerGroupAll());
```

### 声音通道

```galaxy
// mute/unmute a channel
SoundChannelMute(c_soundChannelMusic, true, false);
SoundChannelMute(c_soundChannelSFX,   false, false);

// Set volume
SoundChannelSetVolume(c_soundChannelSFX, 80.0, 0.5);  // (channel, volume, duration)

// Common channel constants
c_soundChannelMusic
c_soundChannelSFX
c_soundChannelAmbient
c_soundChannelSpeech
c_soundChannelUI
```

### 音乐

```galaxy
// Play/stop music
libNtve_gf_PlayMusicForPlayer(lv_player, "Sound/Music/Terran/TerranMain1.ogg", 0, true);
SoundChannelMute(c_soundChannelMusic, false, false);
```
