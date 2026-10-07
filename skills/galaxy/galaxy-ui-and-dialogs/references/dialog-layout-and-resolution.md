# 对话框布局与分辨率

内部像素空间、安全宽度规则、边缘锚定，以及 `DialogControlCreateFromTemplate` 包装。

## 对话框布局与分辨率

> **来源：** [对话框指南 —— Dialog Formatting](https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/043_Dialogs/)

### 内部分辨率系统

对话框以**内部像素空间**计量尺寸——无论玩家显示器分辨率如何，看起来都一样。SC2 客户端把该内部空间映射到玩家屏幕：

| 屏幕比例 | 内部分辨率（宽 × 高） |
|---|---|
| 4:3 | 1600 × 1200 |
| 16:9 | 2133 × 1200 |
| 16:10 | 1920 × 1200 |

高度**永远是 1200 内部像素**。宽度按比例缩放。

**实用规则：** 对话框宽度不要超过 **1600 px**（最窄的 4:3 宽度），这样在任何屏幕比例下都不会被裁切或溢出。按 1600 px 宽设计，对所有玩家都安全。

```galaxy
// Safe full-screen-width dialog (works on all ratios)
dialog gv_mainDialog = DialogCreate(
    1600,           // safe max width
    1200,           // full height
    c_anchorCenter,
    0, 0,
    false
);

// For UI elements, anchor to edges rather than absolute coords
// so they stay on-screen across all ratios:
// c_anchorTopLeft, c_anchorTopRight, c_anchorBottomLeft, c_anchorBottomRight, c_anchorCenter
```

### DialogControlCreateFromTemplate

用模板字符串创建条目时，这个包装可以避开 GUI 编辑器崩溃：

```galaxy
// Use this helper instead of the GUI "Create Dialog Item From Template" action
int CreateDialogItemFromTemplate(int dialog, int type, string template) {
    return DialogControlCreateFromTemplate(dialog, type, template);
}
```
