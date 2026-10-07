# Actor 基础与创建

actor 是什么、`actor`/`actorscope` 句柄、actor scope，以及每个创建/挂接辅助函数的准确参数顺序。

## 什么是 Actor？

**Actor** 是某个游戏内事物（模型、挂接物、光束效果等）的画面/声音表示。actor 与模拟层（单位、区域）相互独立，通过**消息**通信。

大多数视觉脚本走 `ActorSend`（向 actor 发消息），或者用 `ActorCreate` 直接生成 actor。

## Actor 类型与句柄

```galaxy
actor lv_a;            // model/effect actor
actorscope lv_scope;   // actor scope (groups related actors)
```

## 创建 Actor

```galaxy
// Spawn a named actor within a scope (name defined in data)
// ActorCreate(actorscope, actorName, content1, content2, content3)
lv_a = ActorCreate(lv_scope, "MyEffectActor", "", "", "");

// Simple: create model actor at a point (returns actor)
lv_a = libNtve_gf_CreateModelAtPoint(
    "Assets\\Units\\Zerg\\Hydralisk\\Hydralisk.m3",
    lv_point
);

// Attach model to unit — 3 params: (unit, modelPath, attachPoint)
lv_a = libNtve_gf_AttachModelToUnit(
    lv_unit,
    "Assets\\Units\\Zerg\\Hydralisk\\Hydralisk.m3",
    "Chest"     // attach point
);

// Attach model and inherit unit visibility (same params, separate function)
lv_a = libNtve_gf_AttachModelToUnitInheritVisibility(
    lv_unit,
    "Assets\\Effects\\SomeGlow.m3",
    "Origin"
);

// Attach an existing named actor (from data) to a unit at an attach point
// (unit, actorName, attachPoint) — returns actor
lv_a = libNtve_gf_AttachActorToUnit(lv_unit, "TalkIcon",         "Ref_Origin");
lv_a = libNtve_gf_AttachActorToUnit(lv_unit, "BriefingUnitSelectRed", "Ref_Center");

// Attach named actor from data to another actor
lv_a = libNtve_gf_AttachActorToActor(lv_hostingActor, "SomeEffectActor", "Ref_Center");
// (hostingActor, attachingActorName, attachPoint)

// Get the last actor created by any actor operation
actor lv_last = libNtve_gf_ActorLastCreated();
// Thread-safe variant (uses LastCreatedSend)
actor lv_lastSend = libNtve_gf_ActorLastCreatedSend();

// Get the last created actor scope
actorscope lv_scope2 = libNtve_gf_ActorScopeLastCreated();
ActorScopeKill(lv_scope2);   // destroy all actors in the scope

// Attach model to another actor (actor as host)
lv_a = libNtve_gf_AttachModelToActor2(
    lv_hostActor,
    "Assets\\Effects\\SomeEffect.m3",
    "Origin"
);
// (hostActor, modelPath, attachPoint)

// Create named actor (from data) at a point
lv_a = libNtve_gf_CreateActorAtPoint("SomeActorName", lv_point);
// Create a model (raw path) at a point
lv_a = libNtve_gf_CreateModelAtPoint("Assets\\Effects\\Explosion.m3", lv_point);

// Scope — ActorScopeCreate takes ONE optional actor name, not two params
lv_scope = ActorScopeCreate("");           // empty scope
lv_scope = libNtve_gf_ActorScopeLastCreated();
actorscope lv_scopeSend = libNtve_gf_ActorScopeLastCreatedSend();

// Get the main visible actor of a unit (use this instead of ActorFrom(unit))
actor lv_unitActor = libNtve_gf_MainActorofUnit(lv_unit);
```
