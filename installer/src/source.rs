//! 解析安装来源：本地发行目录、本地 zip、或 GitHub release。
//!
//! 下载走 curl.exe 而不是 Rust HTTP 客户端：Windows 10 起自带，代理环境变量
//! 与公司网络的处理行为都是用户已经熟悉的。
//!
//! 包体走 crate::net 的**并发竞速**（直连 + 若干 GitHub 镜像同时开跑，
//! 谁先完成用谁），移植自 HSCL 的 update 模块。清单这类小文件直接单发。

use crate::manifest::Manifest;
use crate::net::{self, NetSettings};
use crate::{mirror};
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
    /// GitHub 来源时的「资产名 -> 下载地址」；本地来源为空。
    ///
    /// 有了它，安装引擎才能**按需**取包体：数据包哈希一致时连下载都省掉，
    /// 而不是先下 18.71 MB 再发现内容根本没变。
    pub assets: Vec<(String, String)>,
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

/// 下载一个发行产物。GitHub 的地址会展开成「直连 + 镜像」并发竞速。
///
/// 竞速只用于包体：清单只有几百 KB，为它开七个进程不值得。
fn download(
    url: &str,
    to: &Path,
    net_settings: &NetSettings,
    proxy_url: Option<&str>,
    say: &mut dyn FnMut(String),
) -> Result<(), String> {
    let race = net_settings.use_mirrors && mirror::is_github_url(url);
    if race {
        let urls = mirror::build_mirror_urls(url, None);
        let outcome = net::race_download(&urls, to, proxy_url, say)?;
        let via = if outcome.url == url {
            "直连".to_string()
        } else {
            format!("镜像 {}", mirror_prefix(&outcome.url).unwrap_or_default())
        };
        say(format!(
            "  {} 完成，{:.1} MB，走{}",
            to.file_name().unwrap_or_default().to_string_lossy(),
            outcome.bytes as f64 / 1048576.0,
            via
        ));
    } else {
        let outcome = net::download_one(url, to, proxy_url)?;
        say(format!(
            "  {} 完成，{:.1} MB",
            to.file_name().unwrap_or_default().to_string_lossy(),
            outcome.bytes as f64 / 1048576.0
        ));
    }
    Ok(())
}

/// 从镜像后的 URL 里挑出前缀，纯粹为了日志好读。
fn mirror_prefix(url: &str) -> Option<String> {
    let scheme_end = url.find("://")? + 3;
    let rest = &url[scheme_end..];
    let host_end = rest.find('/')?;
    Some(format!("{}", &url[..scheme_end + host_end]))
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
    let html = fetch_html(&url, proxy)?;
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

/// 抓 HTML 页面。
///
/// 不能复用 fetch：它带 `Accept: application/vnd.github+json`，而 GitHub 的 HTML
/// 页面会以 **406 Not Acceptable** 拒绝这个 Accept —— release 页面回退就是这样挂的。
fn fetch_html(url: &str, proxy: Option<&str>) -> Result<String, String> {
    let mut a: Vec<String> = vec![
        "-sSL".into(),
        "--fail".into(),
        "--connect-timeout".into(),
        "10".into(),
        "--max-time".into(),
        "30".into(),
        "-H".into(),
        "Accept: text/html,application/xhtml+xml".into(),
        "-H".into(),
        "User-Agent: devkit-installer".into(),
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

/// 找这个仓库最新的 tag，**包含预发布**。
///
/// 不能用 `/releases/latest`：它按定义跳过预发布，而本项目的版本全是 a/b 预发布，
/// 于是那个接口返回 404，重定向落到 /releases 列表页，tag 会被解析成字符串 "releases"。
/// 改成读发布列表，按版本号排序取最高的。
fn latest_tag(
    repo: &str,
    proxy: Option<&str>,
    say: &mut dyn FnMut(String),
) -> Result<String, String> {
    let mut candidates: Vec<String> = Vec::new();

    let api = format!("https://api.github.com/repos/{repo}/releases?per_page=30");
    if let Ok(body) = fetch(&api, proxy) {
        if let Ok(list) = serde_json::from_str::<serde_json::Value>(&body) {
            for r in list.as_array().cloned().unwrap_or_default() {
                if let Some(t) = r.get("tag_name").and_then(|v| v.as_str()) {
                    candidates.push(t.to_string());
                }
            }
        }
    }

    if candidates.is_empty() {
        say("API 拿不到发布列表，改从 releases 页面解析".into());
        let html = fetch_html(&format!("https://github.com/{repo}/releases"), proxy)?;
        let needle = format!("/{repo}/releases/tag/");
        for part in html.split("href=\"") {
            let Some(end) = part.find('"') else { continue };
            let href = &part[..end];
            let Some(idx) = href.find(&needle) else { continue };
            let t = &href[idx + needle.len()..];
            let t = t.split(['"', '?', '/']).next().unwrap_or("").to_string();
            if !t.is_empty() && !candidates.contains(&t) {
                candidates.push(t);
            }
        }
    }
    if candidates.is_empty() {
        return Err(format!("{repo} 一个发布都没有"));
    }

    let mut best = candidates[0].clone();
    for c in &candidates {
        if crate::version::is_newer(c, &best) {
            best = c.clone();
        }
    }
    Ok(best)
}

/// 跟随重定向拿到最终 URL。
fn effective_url(url: &str, proxy: Option<&str>) -> Result<String, String> {
    // 元数据请求必须自己设上限：没有 --max-time 时连接一卡就永久挂住。
    let mut a: Vec<String> = vec![
        "-sSL".into(),
        "--connect-timeout".into(),
        "10".into(),
        "--max-time".into(),
        "30".into(),
        "-o".into(),
        "NUL".into(),
        "-w".into(),
        "%{url_effective}".into(),
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
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn wanted(name: &str) -> bool {
    name.ends_with("-manifest.json") || name.ends_with("-core.zip") || name.ends_with("-data.zip")
}
/// 解析 GitHub release 的资产列表。返回 (tag, [(资产名, 下载地址)])。
///
/// 优先走 API；未认证的 API 每小时只有 60 次，触发 403 是常态，于是回退到页面解析。
/// 安装器自更新也走这里，所以它是公共的。
pub fn release_assets(
    repo: &str,
    tag: Option<&str>,
    settings: &NetSettings,
    say: &mut dyn FnMut(String),
) -> Result<(String, Vec<(String, String)>), String> {
    let detected = net::detect_proxy(settings);
    let proxy = detected.as_ref().map(|p| p.url.as_str());
    let repo = repo
        .trim_start_matches("https://github.com/")
        .trim_end_matches('/')
        .trim_end_matches("/releases")
        .to_string();

    let tag = match tag {
        Some(t) if !t.is_empty() => t.to_string(),
        _ => latest_tag(&repo, proxy, say)?,
    };
    if tag.is_empty() {
        return Err("无法确定 release 版本".into());
    }

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
        Ok(list) if !list.is_empty() => pairs = list,
        Ok(_) => say("API 返回的资产为空，改用页面解析".into()),
        Err(e) => {
            say(format!("API 不可用：{e}"));
            say("改用 release 页面解析（未认证 API 限流为每小时 60 次）".into());
        }
    }
    if pairs.is_empty() {
        pairs = assets_from_page(&repo, &tag, proxy)?;
    }
    Ok((tag, pairs))
}

fn resolve_github(
    from: &str,
    settings: &NetSettings,
    say: &mut dyn FnMut(String),
) -> Result<PathBuf, String> {
    let s = from.trim();
    let (repo, tag_opt) = match s.split_once('@') {
        Some((r, t)) => (r, Some(t)),
        None => (s, None),
    };
    let detected = net::detect_proxy(settings);
    let proxy = detected.as_ref().map(|p| p.url.as_str());
    if let Some(p) = &detected {
        say(format!("使用代理 {}（{}）", p.url, p.source));
    }

    let (_tag, pairs) = release_assets(repo, tag_opt, settings, say)?;
    if !pairs.iter().any(|(n, _)| wanted(n)) {
        return Err(format!("{repo} 的 release 里没有 core/data/manifest 资产"));
    }
    let tmp = std::env::temp_dir().join(format!("sc2agent-gh-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).map_err(|e| format!("建不了临时目录：{e}"))?;


    // 这里只取清单。包体交给安装引擎按需下载 —— 数据包没变时连 18.71 MB 都不该下。
    let mut got = 0usize;
    for (name, url) in &pairs {
        if !name.ends_with("-manifest.json") {
            continue;
        }
        say(format!("下载 {name}"));
        download(url, &tmp.join(name), settings, proxy, say)?;
        got += 1;
    }
    if got == 0 {
        return Err(format!("{repo} 的 release 里没有清单资产"));
    }
    Ok(tmp)
}

/// 取 GitHub 来源的资产表，供引擎按需下载包体。失败就返回空表（引擎会退回目录里的文件）。
pub fn github_assets(
    from: &str,
    settings: &NetSettings,
    say: &mut dyn FnMut(String),
) -> Result<Vec<(String, String)>, String> {
    let s = from.trim();
    let (repo, tag_opt) = match s.split_once('@') {
        Some((r, t)) => (r, Some(t)),
        None => (s, None),
    };
    let (_tag, pairs) = release_assets(repo, tag_opt, settings, say)?;
    Ok(pairs)
}

/// 按需下载一个包体到目录里。已经存在就直接返回。
pub fn fetch_asset(
    dir: &Path,
    assets: &[(String, String)],
    name: &str,
    settings: &NetSettings,
    say: &mut dyn FnMut(String),
) -> Result<PathBuf, String> {
    let target = dir.join(name);
    if target.is_file() {
        return Ok(target);
    }
    let Some((_, url)) = assets.iter().find(|(n, _)| n == name) else {
        return Err(format!("资产表里没有 {name}"));
    };
    let detected = net::detect_proxy(settings);
    let proxy = detected.as_ref().map(|p| p.url.as_str());
    say(format!("下载 {name}"));
    download(url, &target, settings, proxy, say)?;
    Ok(target)
}

/// 把来源字符串解析成一个含清单的本地目录。
/// want_artifacts 为 false 时只取清单，用于「预演」——它不写任何文件，也就用不着包体。
pub fn resolve(
    from: &str,
    settings: &NetSettings,
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
        return Ok(Resolved { dir: path, manifest, assets: Vec::new() });
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
            return Ok(Resolved {
                dir: path.parent().unwrap_or(Path::new(".")).to_path_buf(),
                manifest,
                assets: Vec::new(),
            });
        }
        let inner = work.join("sc2agent-release.json");
        if !inner.is_file() {
            return Err("压缩包里没有 sc2agent-release.json，请改选发行目录或补一个清单".into());
        }
        let text = std::fs::read_to_string(&inner).map_err(|e| format!("读不到内嵌描述：{e}"))?;
        let mut manifest: Manifest =
            serde_json::from_str(&text).map_err(|e| format!("内嵌描述格式不对：{e}"))?;
        manifest.files.core.clear(); // 内容已经解压到 work 里
        return Ok(Resolved { dir: work, manifest, assets: Vec::new() });
    }

    if is_github_shorthand(from) {
        // want_artifacts 已经不需要了：包体一律按需下载，见 install_engine。
        let _ = want_artifacts;
        let dir = resolve_github(from, settings, say)?;
        let manifest = read_manifest(&dir)?;
        let assets = github_assets(from, settings, say).unwrap_or_default();
        return Ok(Resolved { dir, manifest, assets });
    }

    Err(format!("认不出这个来源：{from}"))
}
