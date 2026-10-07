# 头像、Actor Ref 与 Scope 访问

游戏头像句柄，以及在 scope、单位与父 actor 中解析具名 actor/ref。

## 头像（Portrait）

```galaxy
// Get the game portrait (bottom-left)
portrait lv_p = PortraitGetGame();

// Set portrait to a unit
PortraitSetUnit(lv_p, lv_unit, 0.0, "", false);

// Portrait actors are accessed via ActorScopeFromPortrait / ActorFromPortrait:
actorscope lv_pScope = ActorScopeFromPortrait(lv_portraitIndex);
actor      lv_pActor = ActorFromPortrait(lv_portraitIndex);
```

## Actor Ref 与 Scope 访问

```galaxy
// Get a named actor within a scope or actor
actor lv_child = ActorRefGet(lv_a, "::Main");
actor lv_child2 = ActorScopeRefGet(lv_scope, "::Main");

// Set a named ref
ActorRefSet(lv_a, "::Target", lv_targetActor);
ActorScopeRefSet(lv_scope, "::Target", lv_targetActor);

// Get scope from a unit
actorscope lv_unitScope = ActorScopeFromUnit(lv_unit);

// Get actor from a scope by name
actor lv_named = ActorFromScope(lv_scope, "SomeActorInScope");

// Get actor from parent actor by name
actor lv_sub = ActorFromActor(lv_a, "SubActorName");
```
