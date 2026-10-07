# Actor 消息

向 actor 发消息、裸消息字符串与库构造函数的对比，以及常用消息表。

## 发送 Actor 消息

消息控制 actor 的一切——动画、颜色、缩放、可见性等。

```galaxy
// Send a constructed message string to an actor
ActorSend(lv_a, "SetTintColor {1,0,0,1}");
ActorSend(lv_a, "SetScale 0.800000");   // scale by string

// Using library message constructors:
ActorSend(lv_a, libNtve_gf_SetTintColor(1.0, 0.0, 0.0, 1.0));
ActorSend(lv_a, libNtve_gf_SetScale("1.5 1.5 1.5"));
ActorSend(lv_a, libNtve_gf_SetVisibility("Hide"));
ActorSend(lv_a, libNtve_gf_SetVisibility("Show"));

// Destroy actor
ActorSend(lv_a, "Destroy");

// Team color
ActorSend(lv_a, libNtve_gf_SetTeamColor(lv_player));

// Send to a unit's main actor
ActorSendTo(ActorFrom(lv_unit), libNtve_gf_SetTintColor(1.0, 0.5, 0.5, 1.0));

// Send actor message to a unit (shortcut — targets the unit’s own actor)
libNtve_gf_SendActorMessageToUnit(lv_unit, "AnimGroupApply Stand,Victory");
libNtve_gf_SendActorMessageToUnit(lv_unit, "StatusSet MarinePortrait 7");
```

### 常用消息构造函数

| 消息 | 构造函数 | 作用 |
|---|---|---|
| 设置颜色 | `libNtve_gf_SetTintColor(r,g,b,a)` | 颜色着色 |
| 设置缩放 | `libNtve_gf_SetScale("x y z")` | 缩放模型 |
| 设置可见性 | `libNtve_gf_SetVisibility("Show"/"Hide")` | 显示/隐藏 |
| 设置方位 | `libNtve_gf_SetBearings(x,y,z,fx,fy,fz)` | 位置+朝向 |
| 发信号 | `libNtve_gf_Signal("EventName")` | 触发数据事件 |
| 播放动画 | `MakeMsgAnimPlay("Walk Stand Birth Death",flags,blend,0,0)` | 动画 |
