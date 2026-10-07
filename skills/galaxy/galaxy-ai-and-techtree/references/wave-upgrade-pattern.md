# 波次升级模式（难度缩放）

逐波施放升级来缩放敌方难度。用计数器记录波次，在每次波次切换时施加升级等级：

```galaxy
void AddUpgradeForWave(int lp_player, string lp_upgrade) {
    TechTreeUpgradeAddLevel(lp_player, lp_upgrade, 1);
}

void RemoveUpgradeForWave(int lp_player, string lp_upgrade) {
    TechTreeUpgradeAddLevel(lp_player, lp_upgrade, -1);
}

// Called at start of each new wave:
void ApplyWaveUpgrades(int lp_waveNumber, int lp_enemyPlayer) {
    if (lp_waveNumber == 3) {
        AddUpgradeForWave(lp_enemyPlayer, "InflictedDamageIncrease");
    }
    if (lp_waveNumber == 5) {
        AddUpgradeForWave(lp_enemyPlayer, "ArmorIncrease");
    }
}
```
