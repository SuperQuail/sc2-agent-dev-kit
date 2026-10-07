# Catalog（运行时数据字段访问）

Catalog 让你在触发器中于运行时读取和修改游戏数据字段——伤害、射程、花费等。

## 读取 catalog 值

```galaxy
// Get the string value of a data field
string lv_val = CatalogFieldValueGet(
    c_gameCatalogUnit,           // which catalog
    "Marine",                    // entry name
    "LifeMax",                   // field name
    lv_player                    // player context
);

// Get as integer or fixed
int   lv_int  = CatalogFieldValueGetAsInt(c_gameCatalogUnit, "Marine", "LifeMax", lv_player);
fixed lv_real = libNtve_gf_CatalogFieldValueGetAsReal(c_gameCatalogWeapon, "C-14Rifle", "Range", lv_player);

// Catalog constants
c_gameCatalogUnit
c_gameCatalogWeapon
c_gameCatalogAbil
c_gameCatalogEffect
c_gameCatalogBehavior
c_gameCatalogUpgrade
c_gameCatalogModel
c_gameCatalogSound
c_gameCatalogActor
```

## 写入 catalog 值

```galaxy
// Set a field for a player (overrides for that player)
bool lv_ok = CatalogFieldValueSet(
    c_gameCatalogWeapon, "C-14Rifle", "Range",
    lv_player, "8"               // value as string
);

// Modify relative to current value
CatalogFieldValueModify(
    c_gameCatalogUnit, "Marine", "LifeMax",
    lv_player, c_upgradeOperAdd, "25"    // add 25 HP
);

// Set as real
libNtve_gf_CatalogFieldValueSetAsReal(
    c_gameCatalogWeapon, "C-14Rifle", "Range",
    lv_player, 9.0
);
```

## 数组字段

```galaxy
// Field array element — for fields like Weapons[0].Range
int lv_count = CatalogFieldValueCount(c_gameCatalogUnit, "Marine", "Weapons", lv_player);
string lv_wpn = CatalogFieldValueGet(c_gameCatalogUnit, "Marine", "Weapons[0]", lv_player);
```

## Catalog 引用

```galaxy
// Read a link reference (e.g., which weapon a unit uses)
string lv_ref = CatalogReferenceGet(c_gameCatalogUnit, "Marine", "Weapons[0]", lv_player);

// get as int
int lv_ri = CatalogReferenceGetAsInt(c_gameCatalogUnit, "Marine", "Weapons[0]", lv_player);
```

> NativeLib 还提供 `libNtve_gf_CatalogReferenceGetAsReal`、`libNtve_gf_CatalogFieldValueSetAsReal` 与 `libNtve_gf_CatalogReferenceModifyBasedOnDefaultValue`。
>
> 用 XML 编写 catalog（而不是在运行时覆盖它）由 `sc2data-units-abilities` 拥有。
