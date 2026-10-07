# 传送（头像语音）

脚本化头像语音、清除传送，以及 WoL 战役传送管线。

## 传送（头像语音）

```galaxy
// Play a scripted speech transmission
TransmissionSend(
    PlayerGroupAll(),               // who sees/hears it
    TransmissionSourceFromUnit(lv_unit),  // portrait
    StringToText("Speaker"),        // speaker name
    "",                             // sound link (or "" for none)
    StringToText("Your base is under attack!"),  // subtitle
    c_transmissionDurationAdd,      // how duration is applied
    3.0,                            // extra duration
    true                            // block until done
);

// Clear a specific transmission or all active transmissions
TransmissionClear(TransmissionLastSent());
TransmissionClearAll();
```

---

## 战役传送范式（libCamp / libLbty）

WoL 战役地图对所有传送都用同一条管线。**永远**先调用 `libLbty_PlayTransmissionCueSound` 设置合适的音频通道，再用 `libCamp_gf_SendTransmissionCampaign` 播放真正的语音。

```galaxy
// 0. Duck game audio (lower non-speech channels during transmissions)
libCamp_gf_SetAllSoundChannelVolumesCampaign(libNtve_ge_VolumeChannelMode_Speech);

// 1. Set audio channels to speech mode (required before each campaign transmission)
libLbty_gf_PlayTransmissionCueSound(PlayerGroupAll());

// 2. Send the transmission (blocks until it finishes when last param is true)
libCamp_gf_SendTransmissionCampaign(
    lv_unit,                         // portrait unit (or null for narrator)
    SoundLink("SoundName", -1),      // audio cue (-1 = any variation)
    c_transmissionDurationAdd,       // duration mode
    0.0,                             // extra seconds added
    true                             // wait (block) until done
);

// 3. After all transmissions, restore normal game audio
libCamp_gf_SetAllSoundChannelVolumesCampaign(libNtve_ge_VolumeChannelMode_Game);

// Simple non-portrait transmission (narrator/radio with no unit portrait)
libNtve_gf_SendTransmissionSimple(
    TransmissionSourceFromModel(null),
    c_invalidPortraitId,
    SoundLink("TransmissionSound", 0),
    0.0,
    c_transmissionDurationAdd,
    true
);

// Wait for a specific transmission to finish (non-blocking variant)
TransmissionWait(TransmissionLastSent(), 0.0);

// Tip text that shows in the upper-right of the screen
libNtve_gf_ShowTip(libNtve_gf_FormatTipTitle(StringExternal("Param/TipTitle/..."), 1), StringExternal("Param/TipBody/..."), PlayerGroupAll());
```
