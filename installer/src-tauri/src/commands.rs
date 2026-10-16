//! Tauri 命令层：前端能调的全部入口。
//!
//! 安装逻辑一行没改 —— install_engine 仍然往 mpsc 里送 Msg，这里把同一条流
//! 转发成 Tauri 事件。少一层耦合，也少一处会和 CLI 行为分叉的地方。

use crate::harness::Harness;
use crate::install::{self, Msg, PlanInfo, Summary};
use crate::install_engine::{self, Reporter};
use crate::net::{self, NetSettings, ProxyMode};
use crate::selfupdate::{self, ReleaseInfo};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::{mpsc::channel, Arc};
use tauri::{AppHandle, Emitter};

/// 前端启动时要的一切：版本、安装根、探测到的代理、各 harness 的状态。
#[derive(Serialize)]
pub struct Detect {
    pub version: String,
    pub root: String,
    pub proxy: Option<String>,
    pub proxy_source: Option<String>,
    pub harnesses: Vec<Harness>,
}

fn net_settings(proxy: Option<String>, use_mirrors: bool) -> NetSettings {
    let mut s = NetSettings::default();
    s.use_mirrors = use_mirrors;
    if let Some(p) = proxy.filter(|p| !p.trim().is_empty()) {
        s.proxy_mode = ProxyMode::Manual;
        s.proxy_url = Some(p.trim().to_string());
    }
    s
}

#[tauri::command]
fn detect() -> Detect {
    let root = install::default_install_root();
    let detected = net::detect_proxy(&NetSettings::default());
    Detect {
        version: selfupdate::CURRENT.to_string(),
        root: root.display().to_string(),
        proxy: detected.as_ref().map(|p| p.url.clone()),
        proxy_source: detected.map(|p| p.source.to_string()),
        harnesses: crate::harness::detect_all(),
    }
}

#[tauri::command]
async fn plan(
    from: String,
    root: String,
    proxy: Option<String>,
    use_mirrors: bool,
) -> Result<PlanInfo, String> {
    let settings = net_settings(proxy, use_mirrors);
    let (tx, rx) = channel::<Msg>();
    let rep = Reporter::new(tx, Arc::new(|| {}));
    let root = PathBuf::from(root);
    std::thread::spawn(move || install_engine::run_plan(from, root, settings, rep));
    // 预演没有进度，只等最终结果。
    loop {
        match rx.recv() {
            Ok(Msg::PlanReady(p)) => return Ok(p),
            Ok(Msg::Done(Err(e))) => return Err(e),
            Ok(_) => continue,
            Err(_) => return Err("预演线程意外结束".into()),
        }
    }
}

#[tauri::command]
async fn install(
    app: AppHandle,
    from: String,
    root: String,
    proxy: Option<String>,
    use_mirrors: bool,
    harnesses: Vec<String>,
) -> Result<Summary, String> {
    let settings = net_settings(proxy, use_mirrors);
    let (tx, rx) = channel::<Msg>();
    let rep = Reporter::new(tx, Arc::new(|| {}));
    let root = PathBuf::from(root);
    std::thread::spawn(move || {
        install_engine::run_install(from, root, settings, harnesses, true, rep)
    });
    // 同一条流：转发给前端的进度事件 + 在这里捕获最终结果。
    loop {
        match rx.recv() {
            Ok(Msg::Done(Ok(s))) => return Ok(s),
            Ok(Msg::Done(Err(e))) => return Err(e),
            Ok(m) => {
                let _ = app.emit("install://progress", &m);
            }
            Err(_) => return Err("安装线程意外结束".into()),
        }
    }
}

/// 查有没有新版安装器。None 表示已是最新。
#[tauri::command]
async fn self_check(
    proxy: Option<String>,
    use_mirrors: bool,
) -> Result<Option<ReleaseInfo>, String> {
    let settings = net_settings(proxy, use_mirrors);
    let mut quiet = |_s: String| {};
    selfupdate::check(&settings, &mut quiet)
}

/// 下载新版并就地替换自己。返回新 exe 的路径；前端据此提示重启。
#[tauri::command]
async fn self_update(proxy: Option<String>, use_mirrors: bool) -> Result<String, String> {
    let settings = net_settings(proxy, use_mirrors);
    let mut quiet = |_s: String| {};
    match selfupdate::check(&settings, &mut quiet)? {
        None => Err("已是最新版本".into()),
        Some(info) => {
            selfupdate::apply(&info, &settings, &mut quiet).map(|p| p.display().to_string())
        }
    }
}

pub fn run_gui() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            detect,
            plan,
            install,
            self_check,
            self_update
        ])
        .run(tauri::generate_context!())
        .expect("启动失败");
}
