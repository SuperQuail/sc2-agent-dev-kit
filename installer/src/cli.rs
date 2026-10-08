//! 无界面模式：与 GUI 走同一套引擎，便于脚本化与自检。

use crate::install::{self, Msg};
use crate::install_engine::{self, Reporter};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::Arc;

#[cfg(windows)]
mod win {
    use std::ffi::c_void;

    pub const ATTACH_PARENT_PROCESS: u32 = 0xFFFF_FFFF;
    pub const STD_OUTPUT_HANDLE: u32 = 0xFFFF_FFF5; // (DWORD)-11
    pub const STD_ERROR_HANDLE: u32 = 0xFFFF_FFF4; // (DWORD)-12
    pub const INVALID_HANDLE_VALUE: isize = -1;
    pub const GENERIC_READ: u32 = 0x8000_0000;
    pub const GENERIC_WRITE: u32 = 0x4000_0000;
    pub const FILE_SHARE_READ: u32 = 0x0000_0001;
    pub const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    pub const OPEN_EXISTING: u32 = 3;

    unsafe extern "system" {
        pub fn AttachConsole(dwProcessId: u32) -> i32;
        pub fn AllocConsole() -> i32;
        pub fn GetStdHandle(nStdHandle: u32) -> *mut c_void;
        pub fn SetStdHandle(nStdHandle: u32, hHandle: *mut c_void) -> i32;
        pub fn CreateFileW(
            lpFileName: *const u16,
            dwDesiredAccess: u32,
            dwShareMode: u32,
            lpSecurityAttributes: *mut c_void,
            dwCreationDisposition: u32,
            dwFlagsAndAttributes: u32,
            hTemplateFile: *mut c_void,
        ) -> *mut c_void;
    }
}

/// 让 `--cli` 的输出能被终端看到。
///
/// 程序以 windows_subsystem = "windows" 构建，所以默认没有 stdio。分两种情况：
///   * 输出已被重定向（管道或文件）：句柄本身有效，什么都不用做；
///   * 直接在终端里跑：需要挂到父进程的控制台，再把 std 句柄指向 CONOUT$。
pub fn attach_parent_console() {
    #[cfg(windows)]
    unsafe {
        let existing = win::GetStdHandle(win::STD_OUTPUT_HANDLE);
        if !existing.is_null() && existing as isize != win::INVALID_HANDLE_VALUE {
            return; // 已经被重定向，别覆盖调用方的管道
        }
        if win::AttachConsole(win::ATTACH_PARENT_PROCESS) == 0 {
            win::AllocConsole();
        }
        let name: Vec<u16> = "CONOUT$\0".encode_utf16().collect();
        let h = win::CreateFileW(
            name.as_ptr(),
            win::GENERIC_READ | win::GENERIC_WRITE,
            win::FILE_SHARE_READ | win::FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            win::OPEN_EXISTING,
            0,
            std::ptr::null_mut(),
        );
        if !h.is_null() && h as isize != win::INVALID_HANDLE_VALUE {
            win::SetStdHandle(win::STD_OUTPUT_HANDLE, h);
            win::SetStdHandle(win::STD_ERROR_HANDLE, h);
        }
    }
}

fn flag(args: &[String], name: &str) -> Option<String> {
    let i = args.iter().position(|a| a == name)?;
    args.get(i + 1).cloned()
}

pub fn run(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return 0;
    }
    let cmd = args.iter().find(|a| *a == "plan" || *a == "install").cloned();
    let Some(cmd) = cmd else {
        usage();
        return 2;
    };
    let Some(from) = flag(args, "--from") else {
        eprintln!("--from 是必填的");
        return 2;
    };
    let proxy = flag(args, "--proxy");
    let root = flag(args, "--root")
        .map(PathBuf::from)
        .unwrap_or_else(install::default_install_root);
    let selected: Vec<String> = flag(args, "--harness")
        .map(|s| {
            s.split(',')
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .collect()
        })
        .unwrap_or_default();

    let (tx, rx) = channel::<Msg>();
    let rep = Reporter::new(tx, Arc::new(|| {}));
    let install_now = cmd == "install";
    let root2 = root.clone();
    let sel = selected.clone();
    std::thread::spawn(move || {
        if install_now {
            install_engine::run_install(from, root2, proxy, sel, true, rep);
        } else {
            install_engine::run_plan(from, root2, proxy, rep);
        }
    });

    let mut last = usize::MAX;
    loop {
        match rx.recv() {
            Ok(Msg::Phase(p)) => println!("\n[{p}]"),
            Ok(Msg::Total(t)) => println!("共 {t} 步"),
            Ok(Msg::Step { done, total, label }) => {
                // 只在百分比变化时打印，几十万步否则会刷屏。
                let pct = done * 100 / total.max(1);
                if pct != last || done == total {
                    last = pct;
                    println!("  {done}/{total}  {pct}%  {label}");
                }
            }
            Ok(Msg::Log(l)) => println!("  · {l}"),
            Ok(Msg::PlanReady(p)) => {
                println!("\n版本 {}（layout {}）", p.version, p.layout_revision);
                println!("安装根 {}", root.display());
                println!(
                    "套件   {} 个文件：{} 一致，{} 待更新，{} 缺失",
                    p.core.total, p.core.same, p.core.stale, p.core.missing
                );
                println!(
                    "数据   {} 个文件：{} 一致，{} 待更新，{} 缺失",
                    p.data.total, p.data.same, p.data.stale, p.data.missing
                );
                let d = if p.skip_data {
                    format!("哈希一致，将整体跳过（{:.1} MB 不重写）", p.data_mb)
                } else {
                    format!("将应用（{:.1} MB）", p.data_mb)
                };
                println!("数据包 {d}");
                println!("技能   {} 个", p.skills);
                return 0;
            }
            Ok(Msg::Done(Ok(s))) => {
                println!("\n完成：版本 {}", s.version);
                println!("套件：{}", s.kit.display());
                for (id, dir, n) in &s.linked {
                    println!("  {id} -> {} （{n} 个技能）", dir.display());
                }
                if s.data_skipped {
                    println!("游戏数据未变化，已跳过");
                }
                return 0;
            }
            Ok(Msg::Done(Err(e))) => {
                eprintln!("\n失败：{e}");
                return 1;
            }
            Err(_) => return 1,
        }
    }
}

fn usage() {
    println!(
        "StarCraftIIAgent 安装器（无界面模式）\n\n\
         用法:\n\
           sc2agent-installer.exe                    启动图形界面\n\
           sc2agent-installer.exe --cli plan    --from <来源>\n\
           sc2agent-installer.exe --cli install --from <来源> [--harness dsh,claude]\n\n\
         来源可以是 GitHub release（owner/repo 或 owner/repo@tag）、本地发行目录、或本地 zip。\n\
         可用选项：--root <安装根>  --proxy <代理地址>"
    );
}
