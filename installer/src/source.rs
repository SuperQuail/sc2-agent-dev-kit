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

/// GitHub API 每小时只允许 60 次未认证请求，触发 403 时改用 release 页面解析。
///
/// 页面路径 https://github.com/<repo>/releases/expanded_assets/<tag> 是 GitHub 给
/// 惰性加载用的纯 HTML 片段，不受 API 限流，链接形如 /owner/repo/releases/download/tag/name。
fn assets_from_page(repo: &str, tag: &str, proxy: Option<&str>) -> Result<Vec<(String, String)>, String> {
    let url = format!("https://github.com/{repo}/releases/expanded_assets/{tag}");
    let html = fetch(&url, proxy)?;
    let mut out: Vec<(String, String)> = Vec::new();
    for part in html.split("href=\"") {
        let Some(end) = part.find('"') else { continue };
        let href = &part[..end];
        if !href.contains("/releases/download/") {
            continue;
        }
        let Some(name) = href.rsplit('/').next() else { continue };
        if name.is_empty() || out.iter().any(|(n, _)| n == name) {
            continue;
        }
        out.push((name.to_string(), format!("https://github.com{href}")));
    }
    if out.is_empty() {
        return Err("release 页面里没有找到可下载的资产".into());
    }
    Ok(out)
}

/// 跟随重定向拿到最终 URL，用来把 latest 解析成具体 tag。
fn effective_url(url: &str, proxy: Option<&str>) -> Result<String, String> {
    let mut a: Vec<String> = vec!["-sSL".into(), "-o".into(), "NUL".into(), "-w".into(), "%{url_effective}".into()];
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
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn wanted(name: &str) -> bool {
    name.ends_with("-manifest.json") || name.ends_with("-core.zip") || name.ends_with("-data.zip")
}
fn resolve_github(
    from: &str,
    proxy: Option<&str>,
    want_artifacts: bool,
    say: &mut dyn FnMut(String),
) -> Result<PathBuf, String> {
    let s = from.trim();
    let (repo, tag_opt) = match s.split_once('@') {
        Some((r, t)) => (r.trim_start_matches("https://github.com/"), Some(t.to_string())),
        None => (s.trim_start_matches("https://github.com/"), None),
    };
    let repo = repo
        .trim_end_matches('/')
        .trim_end_matches("/releases")
        .to_string();

    // 先把 tag 定下来：没给就从 latest 的重定向里取。
    let tag = match tag_opt {
        Some(t) => t,
        None => effective_url(&format!("https://github.com/{repo}/releases/latest"), proxy)?
            .rsplit('/')
            .next()
            .unwrap_or_default()
            .to_string(),
    };
    if tag.is_empty() {
        return Err("无法确定 release 版本".into());
    }

    let tmp = std::env::temp_dir().join(format!("sc2agent-gh-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).map_err(|e| format!("建不了临时目录：{e}"))?;

    // 优先走 API。未认证的 API 每小时只有 60 次，触发 403 是常态，于是回退到页面解析。
    say(format!("解析 GitHub release：{repo}@{tag}"));
    let api = format!("https://api.github.com/repos/{repo}/releases/tags/{tag}");
    let mut pairs: Vec<(String, String)> = Vec::new();
    let api_result = fetch(&api, proxy).and_then(|body| {
        let v: serde_json::Value =
            serde_json::from_str(&body).map_err(|e| format!("release JSON 解析失败：{e}"))?;
        let assets = v
            .get("assets")
            .and_then(|a| a.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(assets
            .into_iter()
            .filter_map(|a| {
                let name = a.get("name")?.as_str()?.to_string();
                let url = a.get("browser_download_url")?.as_str()?.to_string();
                Some((name, url))
            })
            .collect::<Vec<_>>())
    });
    match api_result {
        Ok(list) if list.iter().any(|(n, _)| wanted(n)) => pairs = list,
        Ok(_) => say("API 返回的资产里没有我们要的包，改用页面解析".into()),
        Err(e) => {
            say(format!("API 不可用：{e}"));
            say("改用 release 页面解析（未认证 API 限流为每小时 60 次）".into());
        }
    }
    if pairs.is_empty() {
        pairs = assets_from_page(&repo, &tag, proxy)?;
    }

    // 预演只需要清单。把 20 MB 的包也拖下来只为读一个哈希，是纯粹的浪费。
    let interesting = |n: &str| {
        if want_artifacts {
            wanted(n)
        } else {
            n.ends_with("-manifest.json")
        }
    };
    let mut got = 0usize;
    for (name, url) in &pairs {
        if !interesting(name) {
            continue;
        }
        say(format!("下载 {name}"));
        download(url, &tmp.join(name), proxy)?;
        got += 1;
    }
    if got == 0 {
        return Err(format!("{repo}@{tag} 里没有 core/data/manifest 资产"));
    }
    Ok(tmp)
}

/// 把来源字符串解析成一个含清单的本地目录。
/// want_artifacts 为 false 时只取清单，用于「预演」——它不写任何文件，也就用不着包体。
pub fn resolve(
    from: &str,
    proxy: Option<&str>,
    want_artifacts: bool,
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
        let dir = resolve_github(from, proxy, want_artifacts, say)?;
        let manifest = read_manifest(&dir)?;
        return Ok(Resolved { dir, manifest });
    }

    Err(format!("认不出这个来源：{from}"))
}
