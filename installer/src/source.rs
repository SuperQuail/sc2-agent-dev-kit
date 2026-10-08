//! 解析安装来源：本地发行目录、本地 zip、或 GitHub release。
//!
//! 下载走 curl.exe 而不是 Rust HTTP 客户端：Windows 10 起自带，代理环境变量
//! 与公司网络的处理行为都是用户已经熟悉的，也少一个依赖。

use crate::manifest::Manifest;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub struct Resolved {
    /// 存放 core/data/manifest 三个产物的目录。
    pub dir: PathBuf,
    pub manifest: Manifest,
}

fn curl(args: &[String]) -> Result<(), String> {
    let mut cmd = Command::new("curl.exe");
    cmd.args(args);
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let out = cmd.output().map_err(|e| format!("无法启动 curl.exe：{e}"))?;
    if !out.status.success() {
        return Err(format!(
            "下载失败（curl 退出码 {:?}）：{}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(())
}

fn download(url: &str, to: &Path, proxy: Option<&str>) -> Result<(), String> {
    let mut a: Vec<String> = vec!["-sSL".into(), "--fail".into(), "--retry".into(), "2".into()];
    if let Some(p) = proxy.filter(|p| !p.trim().is_empty()) {
        a.push("--proxy".into());
        a.push(p.trim().to_string());
    }
    a.push("-o".into());
    a.push(to.display().to_string());
    a.push(url.into());
    curl(&a)
}

fn fetch(url: &str, proxy: Option<&str>) -> Result<String, String> {
    let mut a: Vec<String> = vec![
        "-sSL".into(),
        "--fail".into(),
        "-H".into(),
        "Accept: application/vnd.github+json".into(),
        "-H".into(),
        "User-Agent: sc2agent-installer".into(),
    ];
    if let Some(p) = proxy.filter(|p| !p.trim().is_empty()) {
        a.push("--proxy".into());
        a.push(p.trim().to_string());
    }
    a.push(url.into());
    let mut cmd = Command::new("curl.exe");
    cmd.args(&a);
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let out = cmd.output().map_err(|e| format!("无法启动 curl.exe：{e}"))?;
    if !out.status.success() {
        return Err(format!("请求失败：{}", String::from_utf8_lossy(&out.stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn read_manifest(dir: &Path) -> Result<Manifest, String> {
    let entry = std::fs::read_dir(dir)
        .map_err(|e| format!("读不到目录 {}：{e}", dir.display()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with("-manifest.json"))
                .unwrap_or(false)
        });
    let Some(path) = entry else {
        return Err(format!("目录里没有 *-manifest.json：{}", dir.display()));
    };
    let text = std::fs::read_to_string(&path).map_err(|e| format!("读不到清单：{e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("清单格式不对：{e}"))
}

fn is_github_shorthand(s: &str) -> bool {
    let s = s.trim();
    if s.ends_with(".zip") || s.starts_with("http") && s.ends_with(".zip") {
        return false;
    }
    // owner/repo 或 owner/repo@tag
    let core = s.split('@').next().unwrap_or(s);
    let parts: Vec<&str> = core.trim_start_matches("https://github.com/").split('/').collect();
    parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty()
}

fn resolve_github(from: &str, proxy: Option<&str>, say: &mut dyn FnMut(String)) -> Result<PathBuf, String> {
    let s = from.trim();
    let (repo, tag) = match s.split_once('@') {
        Some((r, t)) => (r.trim_start_matches("https://github.com/"), Some(t)),
        None => (s.trim_start_matches("https://github.com/"), None),
    };
    let repo = repo.trim_end_matches('/');
    let api = match tag {
        Some(t) => format!("https://api.github.com/repos/{repo}/releases/tags/{t}"),
        None => format!("https://api.github.com/repos/{repo}/releases/latest"),
    };
    say(format!("解析 GitHub release：{api}"));
    let body = fetch(&api, proxy)?;
    let v: serde_json::Value = serde_json::from_str(&body).map_err(|e| format!("release JSON 解析失败：{e}"))?;
    let assets = v.get("assets").and_then(|a| a.as_array()).cloned().unwrap_or_default();
    if assets.is_empty() {
        return Err("该 release 没有任何资产".into());
    }
    let tmp = std::env::temp_dir().join(format!("sc2agent-gh-{}", std::process::id()));
    std::fs::create_dir_all(&tmp).map_err(|e| format!("建不了临时目录：{e}"))?;
    for a in assets {
        let name = a.get("name").and_then(|n| n.as_str()).unwrap_or_default();
        let url = a.get("browser_download_url").and_then(|u| u.as_str()).unwrap_or_default();
        let wanted = name.ends_with("-manifest.json")
            || name.ends_with("-core.zip")
            || name.ends_with("-data.zip");
        if !wanted || url.is_empty() {
            continue;
        }
        say(format!("下载 {name}"));
        download(url, &tmp.join(name), proxy)?;
    }
    Ok(tmp)
}

/// 把来源字符串解析成一个含清单的本地目录。
pub fn resolve(
    from: &str,
    proxy: Option<&str>,
    say: &mut dyn FnMut(String),
) -> Result<Resolved, String> {
    let from = from.trim();
    if from.is_empty() {
        return Err("请先填写来源".into());
    }

    let path = PathBuf::from(from);
    if path.is_dir() {
        let manifest = read_manifest(&path)?;
        return Ok(Resolved { dir: path, manifest });
    }

    if path.is_file() && from.to_ascii_lowercase().ends_with(".zip") {
        // 单个 core.zip 自带 sc2agent-release.json；同目录的 *-manifest.json 优先，
        // 因为它还知道 data 包的哈希。
        let sibling = PathBuf::from(from.replace("-core.zip", "-manifest.json"));
        let work = std::env::temp_dir().join(format!("sc2agent-src-{}", std::process::id()));
        std::fs::create_dir_all(&work).map_err(|e| format!("建不了临时目录：{e}"))?;
        say("解压压缩包".into());
        crate::install::extract_zip(&path, &work, &mut |_| {})?;
        if sibling.is_file() {
            let text = std::fs::read_to_string(&sibling).map_err(|e| format!("读不到清单：{e}"))?;
            let manifest = serde_json::from_str(&text).map_err(|e| format!("清单格式不对：{e}"))?;
            return Ok(Resolved { dir: path.parent().unwrap_or(Path::new(".")).to_path_buf(), manifest });
        }
        let inner = work.join("sc2agent-release.json");
        if !inner.is_file() {
            return Err("压缩包里没有 sc2agent-release.json，请改选发行目录或补一个清单".into());
        }
        let text = std::fs::read_to_string(&inner).map_err(|e| format!("读不到内嵌描述：{e}"))?;
        let mut manifest: Manifest =
            serde_json::from_str(&text).map_err(|e| format!("内嵌描述格式不对：{e}"))?;
        manifest.files.core.clear(); // 内容已经解压到 work 里
        return Ok(Resolved { dir: work, manifest });
    }

    if is_github_shorthand(from) {
        let dir = resolve_github(from, proxy, say)?;
        let manifest = read_manifest(&dir)?;
        return Ok(Resolved { dir, manifest });
    }

    Err(format!("认不出这个来源：{from}"))
}
