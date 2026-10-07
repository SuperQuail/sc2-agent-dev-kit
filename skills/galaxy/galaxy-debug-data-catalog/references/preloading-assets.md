# 资源预加载

在用到之前预加载，避免卡顿：

```galaxy
PreloadAsset("Assets\Textures\MyHero.dds");
PreloadModel("Assets\Units\Hero\Hero.m3");
PreloadImage("Assets\UI\HeroPortrait.dds");
PreloadLayout("Assets\UI\MyDialog.SC2Layout");

// Preload model animations — native function (NOT a libNtve helper):
ModelAnimationLoad("Assets\Units\Hero\Hero.m3",  "Assets\Units\Hero\Hero_Attack.m3a");
ModelAnimationLoadOverriding("Assets\Units\Hero\Hero.m3", "Assets\Override\Hero_Walk.m3a");
ModelAnimationUnload("Assets\Units\Hero\Hero.m3", "Assets\Units\Hero\Hero_Attack.m3a");
// (modelPath, animPath) — animPath is the .m3a animation file
```
