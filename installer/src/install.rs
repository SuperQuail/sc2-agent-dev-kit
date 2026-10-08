//! 安装引擎。
//!
//! 全部工作在后台线程跑，通过 mpsc 把进度送回 UI，并调用 egui 的
//! request_repaint 唤醒界面。Electron 版把这一切塞在 IPC 处理器里同步执行，
//! 所以界面会僵住且看不到进度——这里从结构上避免了那件事。

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};

pub const KIT_DIR: &str = "kit";
pub const STATE_FILE: &str = "install.json";

#[derive(Debug, Clone, Default)]
pub struct Diff {
    pub same: usize,
    pub stale: usize,
    pub missing: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Default)]
pub struct PlanInfo {
    pub version: String,
    pub layout_revision: u32,
    pub skills: usize,
    pub core: Diff,
    pub data: Diff,
    pub data_mb: f64,
    pub skip_data: bool,
    pub kit_path: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct Summary {
    pub version: String,
    pub kit: PathBuf,
    pub linked: Vec<(String, PathBuf, usize)>,
    pub data_skipped: bool,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Phase(String),
    Total(usize),
    Step { done: usize, total: usize, label: String },
    Log(String),
    PlanReady(PlanInfo),
    Done(Result<Summary, String>),
}

pub fn sha256_file(path: &Path) -> Result<String, String> {
    let mut f = std::fs::File::open(path).map_err(|e| format!("读不到 {}：{e}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f.read(&mut buf).map_err(|e| format!("读 {} 失败：{e}", path.display()))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(to_hex(&hasher.finalize()))
}

/// sha2 0.11 的摘要类型不再实现 LowerHex，手写十六进制转换。
fn to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

pub fn default_install_root() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::home_dir().unwrap_or_default().join("AppData/Local"));
    base.join("sc2agent")
}

pub fn read_state(root: &Path) -> Option<serde_json::Value> {
    let text = std::fs::read_to_string(root.join(STATE_FILE)).ok()?;
    serde_json::from_str(&text).ok()
}

/// 解压 zip，逐条目回报进度。
pub fn extract_zip(
    zip_path: &Path,
    dest: &Path,
    tick: &mut dyn FnMut(&str),
) -> Result<usize, String> {
    let f = std::fs::File::open(zip_path)
        .map_err(|e| format!("打不开 {}：{e}", zip_path.display()))?;
    let mut archive = zip::ZipArchive::new(std::io::BufReader::new(f))
        .map_err(|e| format!("不是有效的 zip：{e}"))?;
    let total = archive.len();
    for i in 0..total {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("读取第 {i} 个条目失败：{e}"))?;
        // mangled_name 会剥掉 ../ 之类的目录穿越，直接用它比手工校验稳。
        let out = dest.join(entry.mangled_name());
        if entry.is_dir() {
            std::fs::create_dir_all(&out).map_err(|e| format!("建目录失败：{e}"))?;
            continue;
        }
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("建目录失败：{e}"))?;
        }
        let mut w = std::fs::File::create(&out).map_err(|e| format!("写 {} 失败：{e}", out.display()))?;
        std::io::copy(&mut entry, &mut w).map_err(|e| format!("解压失败：{e}"))?;
        tick(&entry.mangled_name().to_string_lossy());
    }
    Ok(total)
}

pub fn copy_dir(src: &Path, dest: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dest).map_err(|e| format!("建目录失败：{e}"))?;
    for e in std::fs::read_dir(src).map_err(|e| format!("读目录失败：{e}"))?.filter_map(Result::ok) {
        let to = dest.join(e.file_name());
        if e.path().is_dir() {
            copy_dir(&e.path(), &to)?;
        } else {
            std::fs::copy(e.path(), &to).map_err(|e| format!("复制失败：{e}"))?;
        }
    }
    Ok(())
}

/// 某个文件集合里有哪些和磁盘上的不一致。
pub fn diff_files(root: &Path, hashes: &HashMap<String, String>) -> Diff {
    let mut d = Diff { total: hashes.len(), ..Default::default() };
    for (rel, want) in hashes {
        let p = root.join(rel);
        if !p.is_file() {
            d.missing += 1;
        } else if sha256_file(&p).map(|h| h == *want).unwrap_or(false) {
            d.same += 1;
        } else {
            d.stale += 1;
        }
    }
    d
}

/// 把一个技能拍平复制到 harness，并在 frontmatter 之后注入套件路径。
///
/// 注入点必须在 frontmatter 之后：harness 把开头的 --- 块当 YAML 解析，
/// 任何前置文本都会静默破坏这个技能。
pub fn install_skill(
    src: &Path,
    skills_dir: &Path,
    name: &str,
    kit: &Path,
    banner: bool,
) -> Result<PathBuf, String> {
    let dest = skills_dir.join(name);
    if dest.exists() {
        std::fs::remove_dir_all(&dest).map_err(|e| format!("清理旧目录失败：{e}"))?;
    }
    copy_dir(src, &dest)?;
    if !banner {
        return Ok(dest);
    }
    let skill_file = dest.join("SKILL.md");
    let Ok(text) = std::fs::read_to_string(&skill_file) else {
        return Ok(dest);
    };
    let kit_str = kit.display().to_string();
    if text.contains(&kit_str) {
        return Ok(dest);
    }
    let lines: Vec<&str> = text.lines().collect();
    if lines.first() != Some(&"---") {
        return Ok(dest);
    }
    let Some(end) = lines.iter().skip(1).position(|l| *l == "---").map(|i| i + 1) else {
        return Ok(dest);
    };
    let mut out: Vec<String> = lines[..=end].iter().map(|s| s.to_string()).collect();
    out.push(String::new());
    out.push(format!("> 本技能由 StarCraftIIAgent 安装器部署。套件根目录：`{kit_str}`"));
    out.push("> 文中 `python tools/sc2.py ...` 命令请在该目录下执行。".to_string());
    out.push(String::new());
    out.extend(lines[end + 1..].iter().map(|s| s.to_string()));
    std::fs::write(&skill_file, out.join("\n")).map_err(|e| format!("写 SKILL.md 失败：{e}"))?;
    Ok(dest)
}
