# 调试输出

```galaxy
// Print to the in-game debug window (F7 to open in test mode)
TriggerDebugOutput(c_triggerDebugOutputCode, StringToText("my value: " + IntToString(lv_n)), true);

// Enable/disable debug output globally
TriggerDebugOutputEnable(c_triggerDebugOutputCode, true);

// Open or close the debug window for a player
TriggerDebugWindowOpen(lv_player, true);

// Convenience debug functions (Template / unclassified)
DebugString(lv_player, "lv_unit type: " + UnitGetType(lv_unit));
DebugInt(lv_player, "lv_i", lv_i);
DebugFixed(lv_player, "lv_dist", lv_dist);
DebugUnit(lv_player, lv_unit);
DebugPoint(lv_player, lv_point);

// Debug message type constants
c_triggerDebugOutputCode      // code path output
c_triggerDebugOutputError     // error highlighting
c_triggerDebugOutputMessage   // general message
```
