# 足迹与 EditorCategories

## 足迹

改 `CUnit.Footprint` 之前先查 `DataEditorXML/* Footprints.txt`。足迹带 `Check`、`Place`、`Pathing` 三层以及 `Shape`。

对菌毯友好的建筑，保留尺寸、形状与各层；只清掉菌毯阻挡（`Negative[Creep]`）。

调整菌毯放置时，同样要保留原有的 `Check`/`Place`/`Pathing` 层，以及资源建筑的任何 `NearResources`/`DropOff`/气矿口约束。

## EditorCategories

不要从非活动导出里复制 `EditorCategories`。在当前 `DataEditorXML/` 里 grep 合法取值。把不受支持的阵营族（`FactionMecha`、`FactionPurifier` 等）收拢到 `ObjectFamily:Campaign`，同时保留 `ObjectType`。
