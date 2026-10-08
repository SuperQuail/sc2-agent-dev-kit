//! 开发套件安装器。
//!
//! 名字里不带 SC2：它装的是「套件」这个抽象，SC2 Agent Dev Kit 只是当前接入的第一个。
//! 后续接别的套件时，套件元数据（清单、技能列表、目标目录）由发布方的 manifest 描述，
//! 安装器本身不需要改。
//!
//! 替代原先的 Electron 实现：那版运行时 70.88 MB，而应用本体只有 0.07 MB，
//! 且把安装逻辑塞在同步 IPC 处理器里，界面会僵住、看不到进度。
//! 这里用 Rust + egui 重写，单文件、无运行时依赖，安装全程在后台线程。
//!
//! 带 --cli 时走无界面模式，便于脚本化，输出与 GUI 同一套引擎。

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod cli;
mod fonts;
mod harness;
mod install;
mod install_engine;
mod manifest;
mod mirror;
mod net;
mod selfupdate;
mod source;
mod theme;
mod version;

/// 排查「界面中文是方块」时第一个要问的问题：它到底挑了哪个字体。
fn font_probe() {
    println!("字体目录：");
    for d in fonts::font_dirs() {
        println!("  {}", d.display());
    }
    match fonts::discover() {
        Some(p) => {
            println!("选中：{}（第 {} 个面）", p.path.display(), p.index);
            println!("来源：{}", if p.explicit { "SC2AGENT_FONT" } else { "自动发现" });
        }
        None => println!("未找到能显示中文的字体。可设 SC2AGENT_FONT 指向一个字体文件。"),
    }
}

/// 这个参数是不是「命令行形态」的。
///
/// 曾经这里只判了 --cli 与 --font-probe，于是 --help / --self-check / --self-update
/// 全都掉进 GUI 分支：进程开着一个窗口坐着不动，既不退出也没有任何输出 ——
/// 看起来就像「卡死」，其实是开错了模式。新增命令行开关时记得一起加进来。
fn is_cli_invocation(args: &[String]) -> bool {
    args.iter().any(|a| {
        matches!(
            a.as_str(),
            "--cli" | "--help" | "-h" | "--self-check" | "--self-update" | "--font-probe"
        )
    })
}

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // 上次自更新留下的 .old 在这里删掉。放在最前面，命令行模式也顺带清理。
    selfupdate::clean_leftovers();
    if is_cli_invocation(&args) {
        // windows_subsystem = "windows" 下没有控制台可写，先挂到父进程的控制台。
        cli::attach_parent_console();
        if args.iter().any(|a| a == "--font-probe") {
            font_probe();
            std::process::exit(0);
        }
        std::process::exit(cli::run(&args));
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_min_inner_size([840.0, 620.0])
            .with_title("开发套件安装器"),
        ..Default::default()
    };
    // --from 预填来源，方便做快捷方式或从命令行带参数启动界面。
    let preset = args
        .iter()
        .position(|a| a == "--from")
        .and_then(|i| args.get(i + 1).cloned());
    let preselect: Vec<String> = args
        .iter()
        .position(|a| a == "--harness")
        .and_then(|i| args.get(i + 1).cloned())
        .map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect())
        .unwrap_or_default();
    let autorun = args.iter().any(|a| a == "--run");
    eframe::run_native(
        "开发套件安装器",
        options,
        Box::new(move |cc| Ok(Box::new(app::App::new(cc, preset, preselect, autorun)))),
    )
}
