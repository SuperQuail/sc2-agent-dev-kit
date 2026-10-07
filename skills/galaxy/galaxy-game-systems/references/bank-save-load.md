# Bank 系统（保存 / 读取玩家数据）

Bank 按玩家在会话之间持久化数据。一个 bank = 一个具名文件，绑定一个玩家槽位。

```galaxy
// Open (create if missing) — use your map/mod name as the bank name
BankLoad("MyMapName", lv_player);
bank lv_bank = BankLastCreated();

// Wait for async load to complete (needed if called at game start)
BankWait(lv_bank);

// Check section/key existence
bool lv_has = BankKeyExists(lv_bank, "Stats", "TotalKills");

// Read values
int   lv_kills   = BankValueGetAsInt(lv_bank, "Stats", "TotalKills");
bool  lv_flag    = BankValueGetAsFlag(lv_bank, "Flags", "CompletedTutorial");
fixed lv_score   = BankValueGetAsFixed(lv_bank, "Stats", "BestScore");
text  lv_name    = BankValueGetAsText(lv_bank, "Profile", "Name");

// Write values
BankValueSetFromInt(lv_bank, "Stats", "TotalKills", lv_kills + 1);
BankValueSetFromFlag(lv_bank, "Flags", "CompletedTutorial", true);
BankValueSetFromFixed(lv_bank, "Stats", "BestScore", 9999.0);

// Save to disk
BankSave(lv_bank);

// Remove section
BankSectionRemove(lv_bank, "OldData");

// Remove a key
BankKeyRemove(lv_bank, "Stats", "OldKey");

// Enable signature/encryption (prevents tampering by the player)
// Call this immediately after BankLoad, before BankWait
BankSetOptionSignature(lv_bank, true);
```

> **Bank 文件在磁盘上的位置（Windows）：**
> `Documents\StarCraft II\StarCraftPlayer.ID@#\Banks\[MapAuthorID]\[BankName].SC2Bank`
>
> 启用签名的 bank 由引擎做校验和。手工编辑已签名的 bank 文件会让它在下次读取时失效，引擎会当作空档/损坏处理。
>
> **完整参考：** [Banks guide](https://s2editor-guides.readthedocs.io/New_Tutorials/03_Trigger_Editor/051_Banks/)

## Bank 结构（层级）

```
Bank file
 └─ Section  (e.g. "Stats", "Flags", "Profile")
     └─ Key  (e.g. "TotalKills", "BestScore")
         └─ Value  (typed: Int, Fixed, Bool, String, Text, Point, Unit)
```
