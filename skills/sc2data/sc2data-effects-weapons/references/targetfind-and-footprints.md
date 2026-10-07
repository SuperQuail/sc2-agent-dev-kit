# TargetFind 与 Footprint

效果与武器在何处解析目标点，以及 CFootprint 如何为建筑阻挡地形。

## TargetFind（`TargetFindData.xml`）

TargetFind 类型决定效果或武器在何处「找到」它的目标点/目标单位：

| 常用 TargetFind | 含义 |
|---|---|
| `TargetPoint` | 最初选定的目标位置 |
| `SourcePoint` | 施法者的位置 |
| `BuffTarget` | 行为所施加到的单位 |

## Footprint（`FootprintData.xml` / `TerrainData.xml`）

Footprint 定义建筑阻挡的地形：

```xml
<CFootprint id="MyStructureFootprint">
    <Layers index="0">
        <Row index="0" value="PPPPPP"/>
        <Row index="1" value="PPPPPP"/>
    </Layers>
</CFootprint>
```

`P` = 可通行，`X` = 阻挡。
