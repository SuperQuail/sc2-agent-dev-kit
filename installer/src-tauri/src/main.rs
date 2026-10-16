//! 开发套件安装器。
//!
//! 前端是 Tauri + React（见 ../ui），后端是这里的 Rust：安装、来源解析、
//! 镜像竞速、代理探测、自更新都在 Rust 侧，前端只负责画。
//!
//! 带 --cli 等命令行开关时走无界面模式，与图形界面共用同一套引擎。

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod commands;
mod harness;
mod install;
mod install_engine;
mod manifest;
mod mirror;
mod net;
mod selfupdate;
mod source;
mod version;

/// 这个参数是不是「命令行形态」的。
///
/// 曾经这里只判了 --cli，于是 --help / --self-check / --self-update 全都掉进
/// GUI 分支：进程开着窗口坐着不动，既不退出也没有任何输出。新增命令行开关时
/// 记得一起加进来。
fn is_cli_invocation(args: &[String]) -> bool {
    args.iter().any(|a| {
        matches!(a.as_str(), "--cli" | "--help" | "-h" | "--self-check" | "--self-update")
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // 上次自更新留下的 .old 在这里删掉。放在最前面，命令行模式也顺带清理。
    selfupdate::clean_leftovers();
    if is_cli_invocation(&args) {
        cli::attach_parent_console();
        std::process::exit(cli::run(&args));
    }
    commands::run_gui();
}
