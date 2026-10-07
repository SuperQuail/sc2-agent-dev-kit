# 飘字（世界空间浮动文本）

在地图位置创建浮动文本，或附着到单位上，带颜色、速度、存活时间与淡出。

## 飘字（世界空间浮动文本）

在地图位置上方显示浮动文本：

```galaxy
TextTagCreate();
int lv_tag = TextTagLastCreated();

TextTagSetText(lv_tag, StringToText("+50"), 14, false, PlayerGroupAll());
TextTagSetColor(lv_tag, ColorWithAlpha(1.0, 0.9, 0.0, 1.0), PlayerGroupAll());
TextTagSetPosition(lv_tag, lv_point, 1.0, false);
TextTagSetVelocity(lv_tag, 0.0, 45.0, 0.5, false);   // float upward
TextTagSetLifespan(lv_tag, 2.5);                       // disappear after 2.5s
TextTagSetFadeTime(lv_tag, 1.0);                       // fade last 1s
TextTagShow(lv_tag, true, PlayerGroupAll());

// Attach to a unit (floats with the unit)
TextTagAttachToUnit(lv_tag, lv_unit, 1.0, false);
```
