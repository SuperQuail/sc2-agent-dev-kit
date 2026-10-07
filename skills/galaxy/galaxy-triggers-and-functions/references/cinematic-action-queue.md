# 过场动画队列

触发器队列把长时间运行的过场触发器串行化，避免它们重叠。

```galaxy
// Enter the queue at the START of a cinematic trigger
TriggerQueueEnter();
// ... all cinematic steps ...
TriggerQueueExit();   // release queue at END

// Pause / resume the queue
TriggerQueuePause(true);   // block next trigger from starting
TriggerQueuePause(false);  // resume

// Discard pending items
TriggerQueueClear(c_triggerQueueRemove);

// Check if the queue is empty (useful in victory checks)
bool lv_empty = TriggerQueueIsEmpty();
```
