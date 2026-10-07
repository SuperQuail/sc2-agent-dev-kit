# Actor 贴图、模型与外观

贴图组、换模型、贴图选择，以及模型销毁/朝向。

## Actor 贴图 / 外观

```galaxy
// Swap out a texture group
ActorSend(lv_a, libNtve_gf_TextureGroupApply("ZergPlayerColor"));
ActorSend(lv_a, libNtve_gf_TextureGroupRemove("ZergPlayerColor"));

// Swap model — (modelName, variation)
ActorSend(lv_a, libNtve_gf_ModelSwap("NewModelName", 0));

// Select texture by ID
ActorSend(lv_a, libNtve_gf_TextureSelectByID("TextureId"));

// Apply texture group globally (all actors)
ActorTextureGroupApplyGlobal("TextureGroupName");
ActorTextureGroupRemoveGlobal("TextureGroupName");

// Kill (destroy) a model actor
libNtve_gf_KillModel(lv_a);

// Make a model face an angle
libNtve_gf_MakeModelFaceAngle(lv_a, 90.0);  // (actor, angleDegrees)
```
