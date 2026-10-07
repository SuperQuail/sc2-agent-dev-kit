# Wait 与定时器

## Wait

`Wait` 只阻塞当前触发器线程，不阻塞其他线程：

```galaxy
Wait(0.5, c_timeGame);   // wait 0.5 game seconds
Wait(2.0, c_timeReal);   // wait 2 real seconds
```

## 定时器（Timer）

```galaxy
timer lv_t = TimerCreate();
TimerStart(lv_t, 10.0, false, c_timeGame);   // one-shot after 10s
TimerStart(lv_t, 5.0, true, c_timeGame);     // repeating every 5s
TimerPause(lv_t, true);
fixed lv_remaining = TimerGetDuration(lv_t);
libNtve_gf_StopTimer(lv_t);

// Fire a trigger when it expires:
TriggerAddEventTimer(myTrigger, lv_t);
timer lv_fired = EventTimer(); // in handler
```
