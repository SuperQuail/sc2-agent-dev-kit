//! StarCraftIIAgent 安装器。
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
mod source;
mod theme;

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--cli" || a == "--font-probe") {
        // windows_subsystem = "windows" 下没有控制台可写，先挂到父进程的控制台。
        cli::attach_parent_console();
    }
    if args.iter().any(|a| a == "--font-probe") {
        // 排查「界面中文是方块」时第一个要问的问题：它到底挑了哪个字体。
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
        std::process::exit(0);
    }
    if args.iter().any(|a| a == "--cli") {
        std::process::exit(cli::run(&args));
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_min_inner_size([840.0, 620.0])
            .with_title("StarCraftIIAgent 安装器"),
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
        "StarCraftIIAgent 安装器",
        options,
        Box::new(move |cc| Ok(Box::new(app::App::new(cc, preset, preselect, autorun)))),
    )
}
