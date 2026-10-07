# 录像与战役任务完成

录制引擎内简报片段，并运行标准的任务胜利流程。

## 录像（简报）

用于引擎内录制的过场片段，直接交给视频播放器（战役简报）：

```galaxy
// Start recording a named video clip
MovieStartRecording("BriefingVideo");

// ... play the scene ...

// Stop recording
MovieStopRecording();
```

---

## 战役任务完成

```galaxy
// Run the standard mission victory cinematic + score screen
libCamp_gf_RunMissionVictorySequence(lv_trigger);
```
