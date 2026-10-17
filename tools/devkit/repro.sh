#!/usr/bin/env bash
# 从 CASC 原件到可分发整包 —— 全流程只用 sc2cat，不手工改任何文件。
# 用法: bash repro.sh <输出目录>
set -euo pipefail

SC2CAT="$(dirname "$0")/target/release/sc2cat.exe"
CASC_TOOL="${CASC_TOOL:-D:/Code/Python/银河编辑器/_recon/casctool/casc.exe}"
SC2DATA="${SC2DATA:-D:/Game/BLZ/StarCraft II/SC2Data}"
OUT="${1:?用法: repro.sh <输出目录>}"
WORK="$OUT/work/traynor01.SC2Map"
MOD="$OUT/work/kit_lotv_zeratul.SC2Mod"

rm -rf "$OUT"; mkdir -p "$OUT/work"

echo '== 1/6 从 CASC 抽原件（递归！漏一层就会丢本地化与本地 GameData）'
# 中文版只下载了 zhCN，其它语系读不到是正常的 —— 后一步 normalize 会补齐 12 个语系。
"$CASC_TOOL" extract -o "$OUT/raw" "$SC2DATA" \
  'campaigns/liberty.sc2campaign/base.sc2maps/maps/campaign/traynor01.sc2map/**' \
  2>&1 | grep -c 'Extraction failure' >/dev/null || true
cp -r "$OUT/raw/campaigns/liberty.sc2campaign/base.sc2maps/maps/campaign/traynor01.sc2map" "$WORK"

echo '== 2/6 源形态 -> 分发形态（驼峰 + 12 语系 + ComponentList）'
"$SC2CAT" normalize "$WORK"

echo '== 3/6 加依赖'
"$SC2CAT" add-dep "$WORK" 'file:Mods\kit_lotv_zeratul.SC2Mod'

echo '== 4/6 换单位：过场 1 处 + 落地 5 处 + 简报 8 处（triggers 与 galaxy 同时改）'
# Id 改 Triggers（编辑器源），galaxy 函数名改 MapScript.galaxy（运行时真正执行的）。
# 这张图没有 TriggerStrings，Id 推不出函数名，所以两个都给。
while read -r ID FN; do
  "$SC2CAT" set-unit "$WORK" --from Marine --to ZeratulZealot --in-trigger "$ID" --galaxy-fn "$FN"
done <<'PAIRS'
CC2D6739 gt_IntroCinematic_Func
EEEC2125 gt_IntroCleanupEscape_Func
5E09D203 gt_BriefingScene00_Func
F12B8538 gt_BriefingScene01_Func
PAIRS

echo '== 5/6 汉化（就地替换，不是追加）'
"$SC2CAT" localize "$MOD" --locale zhCN \
  --set 'Unit/Name/ZeratulZealot=萨尔那加狂热者' \
  --set 'Actor/Name/ZeratulZealot=萨尔那加狂热者' \
  --set 'Button/Name/ZeratulZealot=萨尔那加狂热者' \
  --set 'Abil/Name/ZeratulZealotWhirlwind=飓风' \
  --set 'Abil/Name/ZeratulZealotBlink=闪烁' \
  --set 'Behavior/Name/ZeratulZealotWhirlwind=飓风'

echo '== 6/6 打成单文件 MPQ'
mkdir -p "$OUT/pkg/Maps/Campaign"
"$SC2CAT" pack "$WORK" --out "$OUT/pkg/Maps/Campaign/traynor01.SC2Map"
cp -r "$MOD" "$OUT/pkg/Mods" 2>/dev/null || true
echo '== 完成 =='