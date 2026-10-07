# 动画

在 actor 与 doodad 上播放、清除和查询动画、区间（bracket）动画，以及动画属性。

## 动画

```galaxy
// Play animation on an ACTOR (not unit!) — 5 params:
// (actor, identifier/track, animName, flags, blendTime)
libNtve_gf_PlayAnimation(lv_unitActor, "Attack", "Attack", c_animFlagPlayForever, 0.0);
// Get the actor first:
actor lv_unitActor = libNtve_gf_MainActorofUnit(lv_unit);
libNtve_gf_PlayAnimation(lv_unitActor, c_animNameDefault, "Stand Work", c_animFlagPlayForever, 0.0);

// Clear animation on an ACTOR — (actor, identifier/track)
libNtve_gf_ClearAnimation(lv_unitActor, c_animNameDefault);

// Using raw actor messages with bracket animations:
ActorSend(lv_a, MakeMsgAnimBracketStart(
    "Walk",                   // opening anim
    "Stand",                  // content anim
    "",                       // closing anim
    "WalkBracket",            // bracket name
    c_animFlagPlayForever,    // flags
    c_animTimeVariantAsAutomatic,  // time variant
    0.0                       // time value
));
ActorSend(lv_a, MakeMsgAnimBracketStop("WalkBracket", c_animGroupApplyFlagInstant, 0.0));

// Simple anim message via MakeMsgAnimPlay:
ActorSend(lv_a, MakeMsgAnimPlay(
    "Attack",                 // anim props (space-separated hints)
    c_animFlagPlayForever,    // flags
    c_animTimeVariantAsAutomatic,
    0.0,                      // time value
    -1                        // variation (-1 = random)
));

// Turn on/off animation properties — (actor, propName) — 2 params each!
libNtve_gf_TurnAnimationPropertiesOn(lv_unitActor, "Charred");
libNtve_gf_TurnAnimationPropertiesOff(lv_unitActor, "Charred");
// Variant with blend-in / blend-out animations:
libNtve_gf_TurnAnimationPropertiesOnWithBlendInOut(
    lv_unitActor, "Charred", "Death", "Birth"
);
libNtve_gf_TurnAllAnimationPropertiesOff(lv_unitActor);

// Set animation completion / duration / time
libNtve_gf_SetAnimationCompletion(lv_unitActor, c_animNameDefault, 0.5);  // 50%
libNtve_gf_SetAnimationDuration(lv_unitActor, c_animNameDefault, 2.0);
libNtve_gf_SetAnimationTimeScale(lv_unitActor, c_animNameDefault, 2.0);   // 2x speed
libNtve_gf_SetAnimationTime(lv_unitActor, c_animNameDefault, 1.0, false); // (actor, id, time, scaled)

// On doodads in region — 6 params:
// (region, doodadType, identifier, animName, flags, blendTime)
libNtve_gf_PlayAnimationOnDoodadsInRegion(
    lv_region,
    "",                    // empty = all doodad types in region
    c_animNameDefault,     // animation track identifier
    "Stand Work",          // animation name
    c_animFlagPlayForever, // flags
    c_animTimeDefault      // blend time
);
libNtve_gf_ClearAnimationOnDoodadsInRegion(lv_region, "", c_animNameDefault);
libNtve_gf_KillDoodadsInRegion(lv_region);

// Wait for an animation length — AnimLengthQueryByName has 3 params:
// (actor, animName, scaledTime)
AnimLengthQueryByName(lv_unitActor, "Attack", false);
AnimLengthQueryWait();   // NO args — waits for the query to complete
generichandle lv_qHandle = AnimLengthQueryLastCreated();
fixed lv_len = AnimLengthSync(lv_qHandle);
fixed lv_rem = AnimLengthRemainingSync(lv_qHandle);
```
