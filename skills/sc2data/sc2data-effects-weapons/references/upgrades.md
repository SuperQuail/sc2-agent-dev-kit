# 升级（UpgradeData.xml）

CUpgrade 条目：等级、研究时触发的效果链，以及升级通常触发的效果。

## 升级（`UpgradeData.xml`）

文件：`Base.SC2Data/GameData/UpgradeData.xml`

升级在研究完成时触发一条效果链，并可按等级叠加：

```xml
<CUpgrade id="MyUpgrade">
    <EditorCategories value="Race:Terran"/>
    <Name value="Upgrade/Name/MyUpgrade"/>
    <EffectArray index="0" value="MyUpgradeEffect"/>
    <MaxLevel value="3"/>   <!-- 0 = no limit, 1 = single-level, 3 = tri-level -->
</CUpgrade>
```

升级通常触发 `CEffectUpgradePlayer`（修改单位属性）或 `CEffectApplyBehavior`。
