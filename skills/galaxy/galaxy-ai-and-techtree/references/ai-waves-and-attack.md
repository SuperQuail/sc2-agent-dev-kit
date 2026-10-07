# AI 波次与脚本化攻击波

## AI 波次

波次就是 AI 的攻击组或巡逻组。

```galaxy
// Get a wave
wave lv_wave = AIWaveGet(lv_player, c_waveTypeAttack, 0);

// Get units in a wave
unitgroup lv_waveUnits = AIWaveGetUnits(lv_wave);

// Wave target helpers
wavetarget lv_tgt = AIWaveTargetGatherMelee(lv_player, lv_gatherPoint);
wavetarget lv_tgt2 = AIWaveTargetMeleeDefend(lv_player, lv_defPoint);

// Turn waves on/off
libNtve_gf_CAIWavesEnable(lv_player, true);
libNtve_gf_CAIWaveEnable(lv_player, lv_wave, false);
```

## AI 攻击波——脚本化

需要手动指挥攻击波、而不是用 AI 默认索敌时使用：

```galaxy
// Direct the attack wave at a specific player (standard)
AIAttackWave(lv_player, lv_targetPlayer, c_aiAttackWaveGround, lv_gatherPoint);

// Direct the attack wave at a specific map point (not a player)
AIAttackWaveSetTargetPoint(lv_player, lv_targetPoint);

// Use an entire pre-built unit group as the attack wave (bypasses wave builder)
AIAttackWaveUseGroup(lv_player, lv_unitGroup);
```

> GUI 侧的攻击波数量缩放（在可编辑的 Triggers XML 里用难度修正器包裹单位数量）由 `sc2-attack-wave-scaling` 拥有；本文件只是脚本级 API。
