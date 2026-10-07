# 镜头

平移、吸附、读取与保存/恢复镜头、掠过、camera object，以及输入锁定。

## 镜头

### 平移镜头

```galaxy
// Pan to a point for a player
CameraPan(lv_player, lv_point, 0.0, -1, 10.0, false);
// (player, point, distance, yaw, pitch, sync)

// Snap instantly
CameraSetTarget(lv_player, lv_point, 0.0, -1, 10.0, false);

// Smoothed pan using camera object
CameraApply(lv_player, lv_camInfo, 2.0, false);
```

### 镜头信息 / 位置

```galaxy
// Get current camera position
point lv_camPos = CameraGetTarget(lv_player);
fixed lv_yaw    = CameraGetYaw(lv_player);
fixed lv_dist   = CameraGetDistance(lv_player);

// Save and restore camera state
CameraSave(lv_player);
CameraRestore(lv_player, 0.0, false);
```

### 镜头扫视 / 掠过

```galaxy
// Swoosh camera — confirmed NativeLib function
// (player, startDistance, endDistance, targetPoint, duration)
libNtve_gf_SwooshCamera(lv_player, 10.0, 5.0, lv_point, 1.5);

// Copy of camera info object (useful for saving/restoring cinematic cameras)
camerainfo lv_camCopy = libNtve_gf_CopyOfCameraObject(lv_camInfo);

// NOTE: CameraShake is not exposed via NativeLib helpers.
// Use CameraShake() native directly if available in your build:
// CameraShake(lv_player, lv_intensity, lv_duration, lv_frequency);
```

## 镜头 —— 进阶

```galaxy
// Apply a named camera object from the editor (by ID)
CameraApplyInfo(lv_player, CameraInfoFromId(lv_camId), lv_duration, -1, 10, true);

// Pan with smooth approach
CameraPan(lv_player, lv_point, lv_distance, -1.0, 20.0, false);
// (player, point, distance, yaw, pitch, synchronize?)

// Lock camera input during scripted pan (prevents player moving camera)
CameraLockInput(lv_player, true);
// ... (camera movement) ...
CameraLockInput(lv_player, false);

// Save and restore camera position
CameraSave(lv_player);
CameraRestore(lv_player, 1.5);   // restore over duration (seconds)

// Swoosh camera (cinematic sweep)
libNtve_gf_SwooshCamera(lv_player, lv_startDist, lv_endDist, lv_targetPoint, lv_duration);
```
