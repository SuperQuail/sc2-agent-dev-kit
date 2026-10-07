# 帮助面板（战役）

帮助面板的科技树按钮、页面导航、单位帮助条目与战役教程条目。

## 帮助面板（战役）

```galaxy
// Hide the tech tree button inside the help panel
HelpPanelEnableTechTreeButton(PlayerGroupAll(), false);

// Navigate to a specific help panel page
HelpPanelDisplayPage(PlayerGroupAll(), c_helpPanelPageTutorials);

// Register a unit type on the help panel's unit tab
libCamp_gf_AddUnitTypeToUnitHelpPanel("Marine", false, lv_player);

// Create a campaign tutorial entry (shows in the help panel)
libCamp_gf_CreateCampaignTutorial(
    StringToText("Tutorial Title"),
    StringToText("Explanation body text here."),
    "Assets\\Textures\\btn-unit-terr-marine.dds",   // icon path
    ""   // video path (or empty)
);
```
