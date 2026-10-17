//! 交付物自检：一条命令回答「这个包是不是半成品」。
//!
//! 每一条检查都对应一次真实踩坑，不是想象的：
//! - 改了 `Triggers` 没改 `MapScript.galaxy`  → 进游戏零变化（最贵的一次）
//! - 语系目录不是 12 个驼峰目录              → 文本全是裸键
//! - 依赖没打进包也没装                       → 弹「模块或地图关联内容已不可用」
//! - 没有 `metadata.json`                     → 启动器导入不了
//! - `metadata.json` 里 `mods` 写成字符串     → 整个文件解析失败
use anyhow::Result;
use std::collections::BTreeMap;
use std::path::Path;

use crate::{mpq, triggers};

/// 一条检查结论。
pub struct Finding {
    pub level: Level,
    pub item: String,
    pub detail: String,
}

pub enum Level {
    Ok,
    Warn,
    Bad,
}

impl Finding {
    fn ok(item: &str, detail: impl Into<String>) -> Self {
        Self { level: Level::Ok, item: item.into(), detail: detail.into() }
    }
    fn warn(item: &str, detail: impl Into<String>) -> Self {
        Self { level: Level::Warn, item: item.into(), detail: detail.into() }
    }
    fn bad(item: &str, detail: impl Into<String>) -> Self {
        Self { level: Level::Bad, item: item.into(), detail: detail.into() }
    }
}

/// 检查一个交付目录（形如 `pkg/Maps/Campaign/x.SC2Map` + `pkg/Mods/…`）。
pub fn check(pkg: &Path) -> Result<Vec<Finding>> {
    let mut out = Vec::new();

    let map_file = find_map(pkg)?;
    let (map_name, entries) = match &map_file {
        Some((name, path)) => {
            // 单文件 MPQ 直接读；目录就地遍历
            let e = if path.is_file() {
                mpq::read_entries(path)?
            } else {
                mpq::collect(path)?
                    .into_iter()
                    .map(|(n, p)| Ok((n, std::fs::read(&p)?)))
                    .collect::<Result<Vec<_>>>()?
            };
            (name.clone(), e)
        }
        None => {
            out.push(Finding::bad("地图", "包里找不到 .SC2Map"));
            return Ok(out);
        }
    };

    let get = |want: &str| -> Option<Vec<u8>> {
        entries
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(want))
            .map(|(_, b)| b.clone())
    };

    // 1) Triggers 与 galaxy 是否同步 —— 这条最值钱
    let trig = get("triggers").unwrap_or_default();
    let gal = get("mapscript.galaxy").unwrap_or_default();
    let xml = String::from_utf8_lossy(&trig).to_string();
    let gx = String::from_utf8_lossy(&gal).to_string();
    let names = Default::default();
    let vals = triggers::unit_values_named(&xml, &names)?;
    let mut in_trig: BTreeMap<String, usize> = BTreeMap::new();
    for v in &vals {
        *in_trig.entry(v.value.clone()).or_default() += 1;
    }
    let creates = |unit: &str| -> usize {
        let lit = format!("\"{unit}\"");
        gx.lines()
            .filter(|l| {
                (l.contains("CreateUnits") || l.contains("UnitCreate(") || l.contains("UnitCargoCreate"))
                    && l.contains(&lit)
            })
            .count()
    };
    let mut desync = Vec::new();
    for (unit, n) in &in_trig {
        // galaxy 里连一个字面量都没有 —— 多半是只改了源
        if creates(unit) == 0 && !gx.contains(&format!("\"{unit}\"")) {
            desync.push(format!("{unit}(Triggers {n} 处)"));
        }
    }
    if desync.is_empty() {
        out.push(Finding::ok("触发同步", format!("{} 个单位在两边都有", in_trig.len())));
    } else {
        out.push(Finding::bad(
            "触发同步",
            format!("这些单位只在 Triggers 里，galaxy 里没有 → 进游戏不会有变化: {}", desync.join(", ")),
        ));
    }

    // 2) 语系结构
    let locs: Vec<String> = entries
        .iter()
        .filter_map(|(n, _)| {
            let top = n.split(['\\', '/']).next()?;
            top.to_ascii_lowercase().ends_with(".sc2data").then(|| top.to_string())
        })
        .filter(|t| !t.eq_ignore_ascii_case("base.sc2data"))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let camel = locs.iter().all(|l| l != &l.to_ascii_lowercase());
    if locs.len() >= 12 && camel {
        out.push(Finding::ok("语系", format!("{} 个驼峰目录", locs.len())));
    } else {
        out.push(Finding::bad(
            "语系",
            format!(
                "只有 {} 个{}，成品图都是 12 个驼峰目录 —— 少了文本会变裸键（用 normalize 修）",
                locs.len(),
                if camel { "" } else { "、且不是驼峰" }
            ),
        ));
    }

    // 3) 依赖
    let hdr = get("documentheader").unwrap_or_default();
    let deps: Vec<String> = hdr
        .split(|b| *b == 0)
        .filter(|s| s.starts_with(b"file:"))
        .map(|s| String::from_utf8_lossy(s).trim_start_matches("file:").replace('/', "\\"))
        .collect();
    let game = std::path::Path::new("D:/Game/BLZ/StarCraft II");
    let mut missing = Vec::new();
    for d in &deps {
        let in_pkg = pkg.join(d);
        let in_game = game.join(d);
        if !in_pkg.exists() && !in_game.exists() {
            missing.push(d.clone());
        }
    }
    if missing.is_empty() {
        out.push(Finding::ok("依赖", format!("{} 条都能解析", deps.len())));
    } else {
        out.push(Finding::bad(
            "依赖",
            format!("这些既不在包里也没装 → 会弹「模块或地图关联内容已不可用」: {}", missing.join(", ")),
        ));
    }

    // 4) metadata.json
    let meta = pkg.join("metadata.json");
    if !meta.is_file() {
        out.push(Finding::bad("元数据", "没有 metadata.json → 启动器导入不了这个包"));
    } else {
        let t = std::fs::read_to_string(&meta)?;
        // 只查形状，不做完整 JSON 校验（不为几行代码引 serde_json）。
        // 查的是那次真踩的坑：把 mods 写成字符串，会让**整个文件**解析失败。
        match value_kind(&t, "mods") {
            Some('[') => out.push(Finding::ok("元数据", "mods 是数组")),
            Some(_) => out.push(Finding::bad(
                "元数据",
                "miyin.mods 必须是**数组** —— 写成字符串会让整个 metadata.json 解析失败，campaign 也会跟着丢",
            )),
            None => out.push(Finding::ok("元数据", "没声明 mods")),
        }
        if value_kind(&t, "campaign") != Some('"') {
            out.push(Finding::warn("元数据", "没写 campaign → 启动器无法识别归属资料片"));
        }
    }

    out.push(Finding::ok("地图", format!("{}（{} 个文件）", map_name, entries.len())));
    Ok(out)
}

/// 在包里找地图：先找 `Maps/` 下的单文件，再找目录。
fn find_map(pkg: &Path) -> Result<Option<(String, std::path::PathBuf)>> {
    let maps = pkg.join("Maps");
    let root = if maps.is_dir() { maps } else { pkg.to_path_buf() };
    for e in walk(&root)? {
        if e.is_file() && e.extension().is_some_and(|x| x.eq_ignore_ascii_case("SC2Map")) {
            return Ok(Some((e.file_name().unwrap().to_string_lossy().to_string(), e)));
        }
        if e.is_dir() && e.extension().is_some_and(|x| x.eq_ignore_ascii_case("SC2Map")) {
            return Ok(Some((e.file_name().unwrap().to_string_lossy().to_string(), e)));
        }
    }
    Ok(None)
}

fn walk(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut out = Vec::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    for e in std::fs::read_dir(dir)? {
        let p = e?.path();
        out.push(p.clone());
        if p.is_dir() {
            out.extend(walk(&p)?);
        }
    }
    Ok(out)
}

/// 找键后面第一个非空字符，用来判断值是数组还是字符串。
fn value_kind(json: &str, key: &str) -> Option<char> {
    let at = json.find(&format!("\"{key}\""))? + key.len() + 2;
    json[at..].trim_start().trim_start_matches(':').trim_start().chars().next()
}
