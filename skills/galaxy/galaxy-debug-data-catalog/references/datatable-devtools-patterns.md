# SC2-IngameDevTools 的 DataTable 模式（首选）

SC2-IngameDevTools 代码库大量使用 DataTable，作为 `ItemList` 抽象、聊天命令参数、每玩家状态与 catalog 查询的后备存储。关键模式：

```galaxy
// Namespaced keys prevent collisions — use "Module.Feature.Key" format
DataTableSetInt(true, "DataTableListPage."+IntToString(player), page);
int page = DataTableGetInt(true, "DataTableListPage."+IntToString(player));

// Store catalog type integer by catalog name string
DataTableSetInt(true, "Catalog.Unit",    c_gameCatalogUnit);
DataTableSetInt(true, "Catalog.Ability", c_gameCatalogAbil);
// Retrieve: int catalog = DataTableGetInt(true, "Catalog."+catalogName);

// Per-player listbox selection state (DataTable as a UI state cache)
string key = listId+".Selected["+IntToString(player)+"]:"+IntToString(listItem);
DataTableSetString(true, key, ItemListGetVal(itemList, index));
if (DataTableValueExists(true, key)) {
    string val = DataTableGetString(true, key);
}
DataTableValueRemove(true, key);

// Trigger event param passing via DataTable (scoped to trigger execution context)
string paramKey = TriggerEventParamName("DevTools_ChatCommand.Exec."+cmd, "_player");
DataTableSetInt(false, paramKey, player);
int p = DataTableGetInt(false, TriggerEventParamName(EventGenericName(), "_player"));

// ItemList backing store (the full ItemList implementation in ItemList/index.galaxy)
// ItemList is a string key; all data goes into the DataTable with structured keys:
// "ItemList::[listId][index]"      → the value stored at that slot
// "ItemList::[listId].Count"       → current item count
// "ItemList[listId].Active.Player1" → current selection per player
```
