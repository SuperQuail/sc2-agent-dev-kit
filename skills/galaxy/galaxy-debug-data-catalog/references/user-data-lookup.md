# UserData

UserData 让你把自定义的带类型值直接嵌在游戏数据条目里（在编辑器的数据模块中配置）。

```galaxy
// Load a user data value by category and field
string lv_val = UserDataGetString("MyCategory", lv_entryName, "MyField", 0, lv_player);
int    lv_i   = UserDataGetInt("MyCategory", lv_entryName, "MyField", 0, lv_player);
fixed  lv_f   = UserDataGetFixed("MyCategory", lv_entryName, "MyField", 0, lv_player);
text   lv_t   = UserDataGetText("MyCategory", lv_entryName, "MyField", 0, lv_player);
color  lv_c   = UserDataGetColor("MyCategory", lv_entryName, "MyField", 0, lv_player);
```

> 参数顺序：`(category, entryName, field, instanceIndex, player)`。在 `galaxy-game-systems` 使用的短参数形式里，实例索引是从 1 开始的；照抄下标前先看清调用点。
