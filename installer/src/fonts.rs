//! 中文字体发现与回退。
//!
//! egui 自带字体只有拉丁字母，不装 CJK 字体中文会渲染成方块（tofu）。
//!
//! 早期实现写死了一份 6 个文件名、只查 C:/Windows/Fonts 的列表，问题是：
//!
//!   * 非中文版 Windows 可能一个都没有；
//!   * 用户自己装的字体默认落在 %LOCALAPPDATA%\Microsoft\Windows\Fonts；
//!   * **文件名不可信**——本机 568 个字体里，正则 'deng' 把 OLDENGL.TTF（一种英文花体）
//!     也匹配了进来。所以每个候选都要真的验证字形覆盖。
//!
//! 策略：按优先级排序候选，逐个验证、**命中即停**（否则每次启动要读 500 多个字体文件），
//! 全部失败才退化成全目录扫描。

use std::path::{Path, PathBuf};

/// 判定「这个字体能显示界面里的中文」时抽查的字。
///
/// 前四个简繁共用，用来排除纯拉丁字体；后两个二选一，简繁字体都能通过。
const SHARED: [char; 4] = ['中', '文', '的', '一'];
const SIMPLIFIED_MARK: char = '设';
const TRADITIONAL_MARK: char = '設';

#[derive(Clone, Debug)]
pub struct Picked {
    pub path: PathBuf,
    pub index: u32,
    /// 是否来自 SC2AGENT_FONT 显式指定。
    pub explicit: bool,
    /// 验证时已经读过一次，带上免得调用方再读一遍（中文字体动辄十几 MB）。
    pub bytes: Vec<u8>,
}

/// 这个字体面能不能渲染界面需要的中文。
pub fn renders_chinese(bytes: &[u8], index: u32) -> bool {
    use skrifa::{FontRef, MetadataProvider};
    let Ok(font) = FontRef::from_index(bytes, index) else {
        return false;
    };
    let cmap = font.charmap();
    if !SHARED.iter().all(|c| cmap.map(*c).is_some()) {
        return false;
    }
    cmap.map(SIMPLIFIED_MARK).is_some() || cmap.map(TRADITIONAL_MARK).is_some()
}

/// 字体目录：系统级 + 用户级。用户自己装的字体在后者。
pub fn font_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(windir) = std::env::var("WINDIR") {
        dirs.push(PathBuf::from(windir).join("Fonts"));
    }
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        dirs.push(PathBuf::from(local).join("Microsoft/Windows/Fonts"));
    }
    dirs.retain(|d| d.is_dir());
    dirs
}

/// 文件名优先级。数字越小越先试。
fn rank(name: &str) -> u32 {
    let n = name.to_ascii_lowercase();
    let has = |keys: &[&str]| keys.iter().any(|k| n.contains(k));
    // 一档：中文版 Windows 的默认字体，覆盖率与字形都最好。
    if has(&["msyh", "deng", "simhei", "simsun", "simkai", "simfang"]) {
        return 0;
    }
    // 二档：常见的第三方 / 跨平台中文字体。
    if has(&[
        "notosanssc",
        "notoserifsc",
        "notosanscjk",
        "notoserifcjk",
        "sourcehansans",
        "sourcehanserif",
        "sarasa",
        "misans",
        "harmonyos",
        "puhuiti",
        "alibaba",
        "wqy",
        "droidsansfallback",
    ]) {
        return 1;
    }
    // 三档：其他 CJK 字体。汉字部分（中/文/的/一）与简体共用，聊胜于无。
    if has(&[
        "jhenghei",
        "mingliu",
        "yugothic",
        "yumincho",
        "meiryo",
        "malgun",
        "nanum",
        "arialuni",
        "unifont",
    ]) {
        return 2;
    }
    3
}

fn font_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    rd.filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            matches!(
                p.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref(),
                Some("ttf" | "ttc" | "otf" | "otc")
            )
        })
        .collect()
}

/// 试读一个文件，命中就返回。index 对 .ttc 会依次试几个面。
fn try_file(path: &Path, explicit: bool) -> Option<Picked> {
    let bytes = std::fs::read(path).ok()?;
    let faces = if path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase)
        == Some("ttc".to_string())
    {
        4
    } else {
        1
    };
    for index in 0..faces {
        if renders_chinese(&bytes, index) {
            return Some(Picked { path: path.to_path_buf(), index, explicit, bytes });
        }
    }
    None
}

/// SC2AGENT_FONT 支持 `路径` 或 `路径#面序号`。
fn from_env() -> Option<Picked> {
    let raw = std::env::var("SC2AGENT_FONT").ok()?;
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    let (path, index) = match raw.rsplit_once('#') {
        Some((p, i)) if i.chars().all(|c| c.is_ascii_digit()) => {
            (p.to_string(), i.parse::<u32>().unwrap_or(0))
        }
        _ => (raw.to_string(), 0),
    };
    let p = PathBuf::from(&path);
    if !p.is_file() {
        return None;
    }
    let bytes = std::fs::read(&p).ok()?;
    if renders_chinese(&bytes, index) {
        Some(Picked { path: p, index, explicit: true, bytes })
    } else {
        None
    }
}

/// 找一个能显示中文的字体。命中即停，不做无谓的全盘扫描。
pub fn discover() -> Option<Picked> {
    if let Some(p) = from_env() {
        return Some(p);
    }

    let dirs = font_dirs();
    let mut files: Vec<(u32, PathBuf)> = Vec::new();
    for d in &dirs {
        for f in font_files(d) {
            let r = f.file_name().and_then(|n| n.to_str()).map(rank).unwrap_or(3);
            files.push((r, f));
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    // 先只在一、二档里找：那是最常见的十余个文件。
    for (r, f) in files.iter().filter(|(r, _)| *r <= 1) {
        let _ = r;
        if let Some(p) = try_file(f, false) {
            return Some(p);
        }
    }
    // 退而求其次：其余全部。
    for (_, f) in files.iter() {
        if let Some(p) = try_file(f, false) {
            return Some(p);
        }
    }
    None
}
