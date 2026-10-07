# 函数命名规范

`SystemName_ActionName` 规则：从函数名就能看出它属于哪个文件。

## 函数命名规范

所有函数都遵循 `SystemName_ActionName` 模式：
- `MapInit_ActivePlayers()` —— 属于 MapInit 系统
- `Bank_Save_RequestSave(int playerID)` —— 属于 Bank 系统、Save 子系统
- `PlayerBoard_UpdatePlayer(int playerID)` —— 属于 PlayerBoard UI
- `Utility_IsNumber(string input)` —— 属于 Utilities
- `PartTerran_AreaJunker_Second_Open()` —— Part + Area + 动作

这样一眼就能看出函数住在哪个文件。
