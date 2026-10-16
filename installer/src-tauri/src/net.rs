//! 代理探测与并发竞速下载。
//!
//! 移植自 HSCL（D:\Code\Rust\HSCL）的 crates/miyin-core/src/update/net.rs。
//!
//! 差异：HSCL 用 reqwest 在线程里竞速；这里统一走 curl.exe（Windows 10+ 自带，
//! 代理与公司网络的处理行为用户已经熟悉），所以竞速是**多个 curl 进程**：
//! 各自写自己的 .partN，第一个成功退出的胜出，其余立刻杀掉。

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

/// 单个候选的最长下载时间，交给 curl 自己掐。
const CURL_MAX_SECS: u64 = 900;
/// 整场竞速的总上限，超过就全部放弃。
const RACE_DEADLINE_SECS: u64 = 900;
/// 轮询间隔。
const POLL_INTERVAL: Duration = Duration::from_millis(120);

#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// 代理模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProxyMode {
    /// **自动**：环境变量 -> Windows 系统代理 -> 直连。
    #[default]
    Auto,
    /// 强制直连，不读任何系统设置。
    Off,
    /// 手动指定（用 NetSettings::proxy_url）。
    Manual,
}

/// 网络相关设置。
#[derive(Debug, Clone)]
pub struct NetSettings {
    pub proxy_mode: ProxyMode,
    /// Manual 模式下用的代理地址，如 http://127.0.0.1:7897。
    pub proxy_url: Option<String>,
    /// 是否允许走 GitHub 镜像加速。
    pub use_mirrors: bool,
}

impl Default for NetSettings {
    fn default() -> Self {
        Self { proxy_mode: ProxyMode::Auto, proxy_url: None, use_mirrors: true }
    }
}

/// 自动探测到的代理地址。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedProxy {
    pub url: String,
    /// 从哪来的，界面要如实告诉用户。
    pub source: &'static str,
}

/// 按设置挑一个代理；返回 None 表示直连。
///
/// 自动模式的顺序：**环境变量 -> Windows 系统代理 -> 直连**。
/// 环境变量优先是因为它更明确（用户或 CI 特意设的）。
pub fn detect_proxy(settings: &NetSettings) -> Option<DetectedProxy> {
    match settings.proxy_mode {
        ProxyMode::Off => None,
        ProxyMode::Manual => settings
            .proxy_url
            .as_deref()
            .map(str::trim)
            .filter(|url| !url.is_empty())
            .map(|url| DetectedProxy { url: normalize_proxy(url), source: "手动设置" }),
        ProxyMode::Auto => from_environment().or_else(system_proxy),
    }
}

/// 环境变量里的代理。大小写都认：Windows 上两种写法都常见。
fn from_environment() -> Option<DetectedProxy> {
    for key in [
        "HTTPS_PROXY",
        "https_proxy",
        "ALL_PROXY",
        "all_proxy",
        "HTTP_PROXY",
        "http_proxy",
    ] {
        if let Ok(value) = std::env::var(key) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(DetectedProxy { url: normalize_proxy(trimmed), source: "环境变量" });
            }
        }
    }
    None
}

/// 把 `host:port` 补成 `http://host:port`。
fn normalize_proxy(raw: &str) -> String {
    let raw = raw.trim();
    if raw.contains("://") {
        raw.trim_end_matches('/').to_string()
    } else {
        format!("http://{}", raw.trim_end_matches('/'))
    }
}

/// 读 Windows 系统代理（Internet Settings）。很多用户只在这里配了代理。
#[cfg(windows)]
fn system_proxy() -> Option<DetectedProxy> {
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;

    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Internet Settings")
        .ok()?;
    let enabled: u32 = key.get_value("ProxyEnable").unwrap_or(0);
    if enabled == 0 {
        return None;
    }
    let server: String = key.get_value("ProxyServer").ok()?;
    let trimmed = server.trim();
    if trimmed.is_empty() {
        return None;
    }
    // 可能是 http=host:port;https=host:port 这种分协议写法，取 https 那段。
    let picked = trimmed
        .split(';')
        .find_map(|part| part.trim().strip_prefix("https="))
        .or_else(|| trimmed.split(';').find_map(|part| part.trim().strip_prefix("http=")))
        .unwrap_or(trimmed);
    Some(DetectedProxy { url: normalize_proxy(picked), source: "Windows 系统代理" })
}

#[cfg(not(windows))]
fn system_proxy() -> Option<DetectedProxy> {
    None
}

/// 竞速下载的结果。
#[derive(Debug, Clone)]
pub struct RaceOutcome {
    /// 最终用的那个 URL（镜像还是直连）。
    pub url: String,
    pub bytes: u64,
}

/// 每个候选写自己的临时文件，避免互相踩。
fn part_path(destination: &Path, index: usize) -> PathBuf {
    let mut name = destination.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".part{index}"));
    destination.with_file_name(name)
}

fn spawn_curl(url: &str, out: &Path, proxy: Option<&str>) -> std::io::Result<Child> {
    let mut cmd = Command::new("curl.exe");
    cmd.arg("-sSL")
        .arg("--fail")
        .arg("--connect-timeout")
        .arg("10")
        .arg("--max-time")
        .arg(CURL_MAX_SECS.to_string());
    if let Some(p) = proxy.filter(|p| !p.trim().is_empty()) {
        cmd.arg("--proxy").arg(p.trim());
    }
    cmd.arg("-o").arg(out).arg(url);
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd.spawn()
}

/// **并发竞速**下载：所有候选同时开跑，第一个成功的胜出，其余立刻放弃。
///
/// 失败信息会一并返回，方便告诉用户「直连超时、镜像 403」之类。
pub fn race_download(
    urls: &[String],
    destination: &Path,
    proxy: Option<&str>,
    say: &mut dyn FnMut(String),
) -> Result<RaceOutcome, String> {
    if urls.is_empty() {
        return Err("没有可用的下载地址".into());
    }
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("建目录失败：{e}"))?;
    }

    let mut running: Vec<(String, PathBuf, Child)> = Vec::new();
    let mut failures: Vec<String> = Vec::new();
    for (i, url) in urls.iter().enumerate() {
        let part = part_path(destination, i);
        match spawn_curl(url, &part, proxy) {
            Ok(child) => running.push((url.clone(), part, child)),
            Err(e) => failures.push(format!("{url} -> 启动失败：{e}")),
        }
    }
    if running.is_empty() {
        return Err(format!("所有下载地址都没能启动：{}", failures.join("；")));
    }
    say(format!("{} 个地址并发竞速", running.len()));

    // 外层必须循环：一轮扫描里可能没有任何子进程退出。
    //
    // 曾经这里只写了内层 while，第一轮 index 就走到末尾、循环退出并直接报失败 ——
    // 等于竞速从来没等过任何人。另外必须有 sleep 和总超时，否则是忙等，
    // 且某个镜像卡住会让整个下载永远挂着。
    let started = std::time::Instant::now();
    loop {
        let mut index = 0usize;
        let mut winner: Option<RaceOutcome> = None;
        while index < running.len() {
            match running[index].2.try_wait() {
                Ok(Some(status)) => {
                    let (url, part, _) = running.remove(index);
                    let size = part.metadata().map(|m| m.len()).unwrap_or(0);
                    if status.success() && size > 0 {
                        std::fs::rename(&part, destination)
                            .map_err(|e| format!("落盘失败：{e}"))?;
                        winner = Some(RaceOutcome { url, bytes: size });
                        break;
                    }
                    failures.push(format!("{url} -> 退出码 {:?}", status.code()));
                    let _ = std::fs::remove_file(&part);
                    // 不递增 index：remove 已经把后面的元素前移了。
                }
                Ok(None) => index += 1,
                Err(e) => {
                    let (url, part, _) = running.remove(index);
                    failures.push(format!("{url} -> 等待失败：{e}"));
                    let _ = std::fs::remove_file(&part);
                }
            }
        }

        // 无论胜负，先掐掉剩下的候选：赢了是不浪费带宽，输了是不留僵尸进程。
        if winner.is_some() || running.is_empty() {
            for (_, other, mut child) in running.drain(..) {
                let _ = child.kill();
                let _ = child.wait();
                let _ = std::fs::remove_file(other);
            }
        }
        if let Some(outcome) = winner {
            return Ok(outcome);
        }
        if running.is_empty() {
            break;
        }
        if started.elapsed() > Duration::from_secs(RACE_DEADLINE_SECS) {
            for (_, other, mut child) in running.drain(..) {
                let _ = child.kill();
                let _ = child.wait();
                let _ = std::fs::remove_file(other);
            }
            failures.push(format!("整体超时（{RACE_DEADLINE_SECS} 秒）"));
            break;
        }
        // 别忙等：每次探查之间睡一下，否则这个循环会吃满一个核。
        std::thread::sleep(POLL_INTERVAL);
    }

    Err(format!("所有下载地址都失败了：{}", failures.join("；")))
}

/// 单个地址的下载，不走竞速。
pub fn download_one(
    url: &str,
    destination: &Path,
    proxy: Option<&str>,
) -> Result<RaceOutcome, String> {
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("建目录失败：{e}"))?;
    }
    let status = spawn_curl(url, destination, proxy)
        .and_then(|mut c| c.wait())
        .map_err(|e| format!("{url} -> 启动失败：{e}"))?;
    if !status.success() {
        return Err(format!("{url} -> 退出码 {:?}", status.code()));
    }
    let bytes = destination.metadata().map(|m| m.len()).unwrap_or(0);
    if bytes == 0 {
        return Err(format!("{url} -> 下载到 0 字节"));
    }
    Ok(RaceOutcome { url: url.to_string(), bytes })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_proxy_addresses() {
        assert_eq!(normalize_proxy("127.0.0.1:7897"), "http://127.0.0.1:7897");
        assert_eq!(normalize_proxy("http://a:1/"), "http://a:1");
        assert_eq!(normalize_proxy("  socks5://a:2  "), "socks5://a:2");
    }

    #[test]
    fn off_mode_never_returns_a_proxy() {
        let s = NetSettings {
            proxy_mode: ProxyMode::Off,
            proxy_url: Some("http://x:1".into()),
            use_mirrors: true,
        };
        assert!(detect_proxy(&s).is_none());
    }

    #[test]
    fn manual_mode_uses_the_given_url() {
        let s = NetSettings {
            proxy_mode: ProxyMode::Manual,
            proxy_url: Some("127.0.0.1:7897".into()),
            use_mirrors: true,
        };
        let p = detect_proxy(&s).expect("manual proxy should be used");
        assert_eq!(p.url, "http://127.0.0.1:7897");
        assert_eq!(p.source, "手动设置");
    }

    #[test]
    fn empty_manual_url_falls_back_to_direct() {
        let s = NetSettings {
            proxy_mode: ProxyMode::Manual,
            proxy_url: Some("   ".into()),
            use_mirrors: true,
        };
        assert!(detect_proxy(&s).is_none());
    }

    #[test]
    fn part_paths_do_not_collide() {
        let d = Path::new("C:/tmp/x.zip");
        assert_ne!(part_path(d, 0), part_path(d, 1));
        assert!(part_path(d, 0).to_string_lossy().ends_with("x.zip.part0"));
    }
}
