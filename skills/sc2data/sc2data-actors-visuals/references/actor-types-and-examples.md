# Actor 类型与子类型示例

Actor 类型清单，以及模型、声音、光束、飞弹的完整示例。

## Actor 类型

| XML 类型 | 用途 |
|---|---|
| `CActorUnit` | 单位的主视觉包装——关联模型、声音、动画 |
| `CActorModel` | 独立或依附的 3D 模型 |
| `CActorAction` | 驱动武器的攻击动画序列 |
| `CActorSite` | 附着点 / 位置锚点 |
| `CActorBeam` | 两点之间的激光/光束 |
| `CActorRange` | 单位脚下绘制的射程指示圈 |
| `CActorSound` | 作为 Actor 事件播放声音 |
| `CActorModelMaterial` | 修改模型上的材质/着色器参数 |
| `CActorTerrain` | 施加地形效果（小特效、烧痕） |
| `CActorMissile` | 视觉飞弹（无事件系统） |
| `CActorDoodad` | 环境物件（树、石头） |
| `CActorQuad` | 2D 精灵/面片叠加层 |
| `CActorQuery` | 查询游戏状态的 Actor |
| `CActorRegion` | 基于区域的 Actor |
| `CActorSiteOp` | 位点操作 Actor |
| `CActorSplat` | 地面贴花/溅射 |
| `CActorTurret` | 炮塔 Actor |
| `CActorVideo` | 视频播放 Actor |

> **注意：** 这不是完整清单。Actor 类型的全集及其确切字段/属性一律查 catalogsData.xsd。每种 Actor 类型支持哪些字段，以 schema 为唯一真源。

## CActorModel — 独立或依附的模型

```xml
<!-- Birth VFX: one-shot model, auto-destroys after animation plays -->
<CActorModel id="MyBirthEffectActor" parent="ModelAnimationStyleOneShot">
    <Model value="MyBirthModel"/>
    <On Terms="ActorCreation" Send="AnimPlay Birth"/>
    <On Terms="AnimDone" Send="Destroy"/>
</CActorModel>

<!-- Persistent looping area model -->
<CActorModel id="MyAuraModel" parent="ModelAnimationStyleContinuous">
    <Model value="MyAuraModel"/>
</CActorModel>
```

## CActorSound

```xml
<CActorSound id="MyReadySound" parent="SoundOneShot">
    <Sound value="MyUnit_Ready"/>
    <On Terms="ActorCreation" Send="AnimPlay"/>
    <On Terms="ActorOrphan" Send="Destroy"/>
</CActorSound>
```

## CActorBeam — 激光束

```xml
<CActorBeam id="MyLaserBeam">
    <Model value="MyBeamModel"/>
    <!-- Beam connects ::Creator (source) to ::Target -->
</CActorBeam>
```

## CActorMissile — 飞弹 Actor

**重要：** CActorMissile **没有** `<On>` 数组，无法接事件。它纯是视觉飞弹，没有事件系统。飞弹发射/命中前后的事件处理请用 CActorAction 或其他 Actor。

```xml
<CActorMissile id="MyMissileActor">
    <Model value="MyMissileModel"/>
    <!-- Missile follows its mover/physics, no events -->
</CActorMissile>
```
