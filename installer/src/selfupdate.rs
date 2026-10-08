//! 安装器自更新。
//!
//! 参考 HSCL（D:\Code\Rust\HSCL）的 update 模块：查 release -> 比版本 -> 下载 -> 就地替换。
//!
//! 替换手法：Windows **允许重命名正在运行的 exe**，所以不需要额外的批处理脚本 ——
//!
//!   1. 新版下到 <exe>.new
//!   2. 自己改名成 <exe>.old
//!   3. .new 改名到原路径
//!   4. 启动新进程，自己退出；下次启动时顺手删掉 .old
//!
//! 这比「写个 cmd 等进程退出再替换」少一个会失败的环节。

use crate::net::{self, NetSettings};
use crate::source;
use crate::version::is_newer;
use std::path::{Path, PathBuf};

/// 安装器自己的发布仓库。
pub const REPO: &str = "SuperQuail/sc2-agent-dev-kit";
/// 当前版本，来自 Cargo.toml。
pub const CURRENT: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct ReleaseInfo {
    pub tag: String,
    pub version: String,
    pub asset_name: String,
    pub asset_url: String,
}

/// 在资产里挑安装器本体。
///
/// 认名字而不是认位置：release 里还有 core.zip / data.zip / manifest.json。
fn pick_installer(assets: &[(String, String)]) -> Option<(String, String)> {
    assets
        .iter()
        .find(|(name, _)| {
            let n = name.to_ascii_lowercase();
            n.ends_with(".exe") && (n.contains("installer") || n.contains("devkit"))
        })
        .cloned()
}

/// 查有没有新版的安装器。返回 None 表示已是最新。
pub fn check(settings: &NetSettings, say: &mut dyn FnMut(String)) -> Result<Option<ReleaseInfo>, String> {
    let (tag, assets) = source::release_assets(REPO, None, settings, say)?;
    let Some((asset_name, asset_url)) = pick_installer(&assets) else {
        return Err("release 里没有找到安装器 exe".into());
    };
    let version = tag.trim_start_matches(['v', 'V']).to_string();
    if is_newer(&version, CURRENT) {
        Ok(Some(ReleaseInfo { tag, version, asset_name, asset_url }))
    } else {
        Ok(None)
    }
}

/// 就地替换自己。成功后返回新版 exe 的路径。
///
/// 调用方应当在返回后立刻退出，让新进程接管。
pub fn apply(
    info: &ReleaseInfo,
    settings: &NetSettings,
    say: &mut dyn FnMut(String),
) -> Result<PathBuf, String> {
    let current = std::env::current_exe().map_err(|e| format!("拿不到自己的路径：{e}"))?;
    let new_path = current.with_extension("exe.new");
    let old_path = current.with_extension("exe.old");

    let detected = net::detect_proxy(settings);
    let proxy = detected.as_ref().map(|p| p.url.as_str());
    if let Some(p) = &detected {
        say(format!("使用代理 {}（{}）", p.url, p.source));
    }

    // 先清掉上次留下的残骸，否则改名会撞名。
    let _ = std::fs::remove_file(&new_path);
    let _ = std::fs::remove_file(&old_path);

    say(format!("下载 {}（{} MB 量级）", info.asset_name, ""));
    let mirror_urls = if settings.use_mirrors {
        crate::mirror::build_mirror_urls(&info.asset_url, None)
    } else {
        vec![info.asset_url.clone()]
    };
    let outcome = if mirror_urls.len() > 1 {
        net::race_download(&mirror_urls, &new_path, proxy, say)?
    } else {
        net::download_one(&info.asset_url, &new_path, proxy)?
    };
    say(format!("下载完成，{:.2} MB", outcome.bytes as f64 / 1048576.0));

    // 校验是 PE 文件。镜像站返回一个 HTML 错误页是最常见的失败形态，
    // 直接改名会把安装器换成一段 HTML。
    if !looks_like_pe(&new_path) {
        let _ = std::fs::remove_file(&new_path);
        return Err("下载到的不是可执行文件（可能是镜像返回了错误页）".into());
    }

    // Windows 允许重命名正在运行的 exe，所以这三步不用等自己退出。
    std::fs::rename(&current, &old_path).map_err(|e| {
        format!("改名失败：{e}（安装器若放在 Program Files 等受保护目录，请手动下载替换）")
    })?;
    if let Err(e) = std::fs::rename(&new_path, &current) {
        // 尽力还原，别把自己搞没了
        let _ = std::fs::rename(&old_path, &current);
        return Err(format!("替换失败：{e}"));
    }
    say("替换完成".into());
    Ok(current)
}

/// 是不是 Windows 可执行文件（PE：以 MZ 开头）。
fn looks_like_pe(path: &Path) -> bool {
    use std::io::Read;
    let Ok(mut f) = std::fs::File::open(path) else {
        return false;
    };
    let mut head = [0u8; 2];
    if f.read_exact(&mut head).is_err() {
        return false;
    }
    &head == b"MZ"
}

/// 删掉上次自更新留下的 .old。启动时顺手做，失败也无所谓。
pub fn clean_leftovers() {
    let Ok(current) = std::env::current_exe() else {
        return;
    };
    let _ = std::fs::remove_file(current.with_extension("exe.old"));
}

/// 启动新版本的自己。
pub fn relaunch(path: &Path) -> Result<(), String> {
    std::process::Command::new(path)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("启动新版失败：{e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // 版本号解析与比较的用例在 crate::version 里。

    #[test]
    fn compares_numeric_parts_first() {
        assert!(is_newer("0.2.0", "0.1.9"));
        assert!(!is_newer("0.1.0", "0.2.0"));
        assert!(!is_newer("0.1.0", "0.1.0"));
    }

    #[test]
    fn compares_prerelease_suffixes() {
        assert!(is_newer("0.1.0-a2", "0.1.0-a1"));
        assert!(!is_newer("0.1.0-a1", "0.1.0-a2"));
        // 正式版比同号的预发布新
        assert!(is_newer("0.1.0", "0.1.0-a1"));
        assert!(!is_newer("0.1.0-a1", "0.1.0"));
    }

    #[test]
    fn suffix_style_does_not_change_ordering() {
        // 0.1.0a1 与 0.1.0-a1 是同一个版本
        assert!(!is_newer("0.1.0a1", "0.1.0-a1"));
        assert!(!is_newer("0.1.0-a1", "0.1.0a1"));
    }

    #[test]
    fn picks_the_installer_asset() {
        let assets = vec![
            ("StarCraftIIAgent-0.1.0a1-core.zip".to_string(), "u1".to_string()),
            ("DevKit-Installer-0.1.0-a1.exe".to_string(), "u2".to_string()),
            ("StarCraftIIAgent-0.1.0a1-manifest.json".to_string(), "u3".to_string()),
        ];
        let picked = pick_installer(&assets).expect("should pick the exe");
        assert_eq!(picked.1, "u2");
    }

    #[test]
    fn ignores_non_installer_exes() {
        let assets = vec![("something-else.exe".to_string(), "u".to_string())];
        assert!(pick_installer(&assets).is_none());
    }
}