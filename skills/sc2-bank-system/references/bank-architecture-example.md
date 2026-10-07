# Galaxy Bank 架构示例

仅作架构示意——本样例里的原生名正是有争议的那些；照抄前先看 [references/bank-natives-and-caveats.md](bank-natives-and-caveats.md) 里的 Bank API 命名注意事项。

```galaxy
const string c_bankName = "MyCampaignBank";
const int c_currentBankSchema = 2;

bank gBank;

// Initialize a fresh bank with current schema defaults.
void libMy_Bank_InitDefaults(bank lp_bank) {
    BankValueSetInt(lp_bank, "Meta", "SchemaVersion", c_currentBankSchema);
    BankValueSetInt(lp_bank, "Meta", "CampaignCompleted", 0);
    BankValueSetInt(lp_bank, "Meta", "Difficulty", 1);
    BankValueSetString(lp_bank, "Meta", "LastPlayedMission", "");
}

// v1 -> v2 migration: introduced Difficulty and LastPlayedMission keys.
// Backfill them with safe defaults for saves made under the v1 schema.
void libMy_Bank_MigrateV1ToV2(bank lp_bank) {
    BankValueSetInt(lp_bank, "Meta", "Difficulty", 1);
    BankValueSetString(lp_bank, "Meta", "LastPlayedMission", "");
}

// Placeholder for the next schema gap. Uncomment and extend when
// c_currentBankSchema increments past 2 (rename keys, normalize enums,
// add new fields, etc.).
// void libMy_Bank_MigrateV2ToV3(bank lp_bank) {
//     // Example: shift Difficulty enum from 1..4 to 0..3, then add new key.
// }

void libMy_Bank_Init(int lp_player) {
    int lv_existingSchema;

    BankPreLoad(c_bankName);
    gBank = BankLoad(c_bankName, lp_player);

    if (!BankSectionExists(gBank, "Meta")) {
        libMy_Bank_InitDefaults(gBank);
        BankSave(gBank);
        return;
    }

    lv_existingSchema = BankValueGetInt(gBank, "Meta", "SchemaVersion");

    // Apply migration chain step by step. Each branch translates one schema gap;
    // they run sequentially so a v1 save reaches v2 before v3 logic (if any) runs.
    if (lv_existingSchema < 2) {
        libMy_Bank_MigrateV1ToV2(gBank);
    }
    // if (lv_existingSchema < 3) { libMy_Bank_MigrateV2ToV3(gBank); }

    if (lv_existingSchema < c_currentBankSchema) {
        BankValueSetInt(gBank, "Meta", "SchemaVersion", c_currentBankSchema);
        BankSave(gBank);
    }
}

void libMy_Bank_SaveMissionVictory(int lp_player, string lp_missionId) {
    BankValueSetInt(gBank, "Missions", lp_missionId + "_Completed", 1);
    BankSave(gBank);
}
```
