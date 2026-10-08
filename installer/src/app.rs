//! 界面。
//!
//! 视觉语言取自 Koishi：深蓝紫底、圆角卡片、低饱和描边、紫色强调色。
//! 布局不照搬它的侧栏控制台——这里是一个安装向导，步骤是线性的。
//!
//! 安装全程跑在后台线程：界面只负责渲染消息通道里的进度，所以不会僵住。

use crate::harness::Harness;
use crate::install::{self, Msg, PlanInfo, Summary};
use crate::install_engine::{self, Reporter};
use crate::theme::*;
use egui::{Color32, RichText, Ui};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(PartialEq)]
enum Stage {
    Idle,
    Planning,
    Installing,
    Done,
    Failed,
}

pub struct App {
    harnesses: Vec<Harness>,
    selected: Vec<String>,
    source_text: String,
    proxy_text: String,
    root: PathBuf,
    stage: Stage,
    plan: Option<PlanInfo>,
    summary: Option<Summary>,
    error: Option<String>,
    phase: String,
    done: usize,
    total: usize,
    current: String,
    log: Vec<String>,
    rx: Option<Receiver<Msg>>,
    tx: Option<Sender<Msg>>,
    ctx_wake: Option<egui::Context>,
    banner: bool,
    /// --run：启动即开始安装，便于做带预设的快捷方式。
    autorun: bool,
    started: bool,
    /// 是否找到了能显示中文的字体。false 时界面只剩拉丁字母。
    font_ok: bool,
}

impl App {
    /// preset 来自命令行的 --from，用来预填来源；双击快捷方式常见。
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        preset: Option<String>,
        preselect: Vec<String>,
        autorun: bool,
    ) -> Self {
        let font_ok = install_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(visuals());
        let harnesses = crate::harness::detect_all();
        Self {
            harnesses,
            // 默认一个都不勾：让用户自己决定把技能交付到哪里。
            selected: preselect,
            source_text: preset.unwrap_or_default(),
            proxy_text: std::env::var("SC2AGENT_PROXY").unwrap_or_default(),
            root: install::default_install_root(),
            stage: Stage::Idle,
            plan: None,
            summary: None,
            error: None,
            phase: String::new(),
            done: 0,
            total: 0,
            current: String::new(),
            log: Vec::new(),
            rx: None,
            tx: None,
            ctx_wake: None,
            banner: true,
            autorun,
            started: false,
            font_ok,
        }
    }

    fn start(&mut self, ctx: &egui::Context, install_now: bool) {
        let (tx, rx) = channel::<Msg>();
        self.rx = Some(rx);
        self.tx = Some(tx.clone());
        self.error = None;
        self.summary = None;
        self.done = 0;
        self.total = 0;
        self.phase.clear();
        self.current.clear();
        self.log.clear();
        self.stage = if install_now { Stage::Installing } else { Stage::Planning };

        let c = ctx.clone();
        let wake: Arc<dyn Fn() + Send + Sync> = Arc::new(move || c.request_repaint());
        let rep = Reporter::new(tx, wake);
        let from = self.source_text.trim().to_string();
        let proxy = if self.proxy_text.trim().is_empty() {
            None
        } else {
            Some(self.proxy_text.trim().to_string())
        };
        let root = self.root.clone();
        let selected = self.selected.clone();
        let banner = self.banner;

        std::thread::spawn(move || {
            if install_now {
                install_engine::run_install(from, root, proxy, selected, banner, rep);
            } else {
                install_engine::run_plan(from, root, proxy, rep);
            }
        });
    }

    fn drain(&mut self) {
        let Some(rx) = &self.rx else { return };
        while let Ok(m) = rx.try_recv() {
            match m {
                Msg::Phase(p) => { self.phase = p; self.current.clear(); }
                Msg::Total(t) => self.total = t,
                Msg::Step { done, total, label } => {
                    self.done = done;
                    self.total = total;
                    self.current = label;
                }
                Msg::Log(l) => self.log.push(l),
                Msg::PlanReady(p) => {
                    self.plan = Some(p);
                    self.stage = Stage::Idle;
                    self.log.push("预演完成".into());
                }
                Msg::Done(Ok(s)) => {
                    self.summary = Some(s);
                    self.stage = Stage::Done;
                }
                Msg::Done(Err(e)) => {
                    self.error = Some(e);
                    self.stage = Stage::Failed;
                }
            }
        }
    }
}

/// 装中文字体。返回是否找到了。
///
/// 找不到时界面只剩拉丁字母，所以要给用户一条英文提示——中文提示自己也渲染不出来。
fn install_fonts(ctx: &egui::Context) -> bool {
    let mut fonts = egui::FontDefinitions::default();
    let picked = crate::fonts::discover();
    if let Some(p) = &picked {
        let mut data = egui::FontData::from_owned(p.bytes.clone());
        data.index = p.index;
        fonts.font_data.insert("cjk".to_owned(), Arc::new(data));
        // 放在最前面：优先用它，缺字时 egui 会继续往后找内置字体。
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "cjk".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("cjk".to_owned());
    }
    ctx.set_fonts(fonts);
    picked.is_some()
}

fn section(ui: &mut Ui, index: usize, title: &str, add: impl FnOnce(&mut Ui)) {
    card_frame().show(ui, |ui| {
        // 卡片默认按内容收缩，内容短的那几节会缩成半宽、文字被裁。撑满可用宽度。
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("{index}"))
                    .color(ACCENT)
                    .strong()
                    .size(13.0),
            );
            ui.label(RichText::new(title).color(TEXT).strong().size(14.0));
        });
        ui.add_space(8.0);
        add(ui);
    });
    ui.add_space(10.0);
}

/// 单个 harness 的卡片。
///
/// 勾选状态由调用方传入/取回：egui::Grid 的单元格按内容最小宽度分配，在里面写
/// set_width(available_width()) 无效，文字会被挤成一列一个字。所以外层用 columns
/// 固定等分，这里只负责画。
fn harness_card(ui: &mut Ui, h: &Harness, on: &mut bool) {
    card_frame().show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            let cb = ui.add_enabled(h.installed, egui::Checkbox::without_text(on));
            if !h.installed {
                cb.on_hover_text("未检测到该 harness 的配置目录");
            }
            ui.vertical(|ui| {
                let name = if h.installed {
                    h.name.to_string()
                } else {
                    format!("{}（未安装）", h.name)
                };
                ui.label(RichText::new(name).color(if h.installed { TEXT } else { TEXT_FAINT }).strong());
                ui.label(
                    RichText::new(h.skills_dir.display().to_string())
                        .color(TEXT_FAINT)
                        .size(11.0),
                );
                if h.installed {
                    ui.label(
                        RichText::new(format!("现有 {} 个技能", h.existing_skills))
                            .color(TEXT_DIM)
                            .size(11.0),
                    );
                }
            });
        });
    });
}

impl eframe::App for App {
    /// 非 UI 逻辑：把后台线程发来的进度收进来。
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.drain();
    }

    /// eframe 0.35 起 App 实现的是 ui(&mut Ui)，面板也挂在 Ui 上而非 Context。
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        // --run：第一帧就开跑，界面照常渲染进度，不会阻塞。
        if self.autorun && !self.started && !self.source_text.trim().is_empty() {
            self.started = true;
            self.start(&ctx, true);
        }
        let busy = matches!(self.stage, Stage::Planning | Stage::Installing);

        egui::Panel::top("header")
            .frame(egui::Frame::new().fill(BG_PANEL).inner_margin(egui::Margin::symmetric(20, 16)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("StarCraftIIAgent 安装器").color(TEXT).strong().size(19.0));
                        ui.label(
                            RichText::new("安装套件，并把 29 个技能交付给你选择的 AI 助手")
                                .color(TEXT_DIM)
                                .size(12.5),
                        );
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("v{VERSION}")).color(ACCENT).size(12.0));
                    });
                });
            });

        egui::Panel::bottom("footer")
            .frame(egui::Frame::new().fill(BG_PANEL).inner_margin(egui::Margin::symmetric(20, 14)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let preview = egui::Button::new(RichText::new("预演").color(TEXT))
                        .fill(BG_CARD)
                        .min_size(egui::vec2(96.0, 34.0));
                    if ui.add_enabled(!busy, preview).clicked() {
                        self.start(&ctx, false);
                    }
                    ui.add_space(8.0);
                    let go = egui::Button::new(RichText::new("开始安装").color(Color32::WHITE).strong())
                        .fill(if busy { ACCENT_DIM } else { ACCENT })
                        .min_size(egui::vec2(120.0, 34.0));
                    if ui.add_enabled(!busy, go).clicked() {
                        self.start(&ctx, true);
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if busy {
                            ui.label(RichText::new(&self.phase).color(ACCENT).size(12.0));
                        } else if self.stage == Stage::Done {
                            ui.label(RichText::new("安装完成").color(OK).strong().size(12.0));
                        } else if self.stage == Stage::Failed {
                            ui.label(RichText::new("失败").color(ERR).strong().size(12.0));
                        }
                    });
                });
            });

        egui::CentralPanel::default_margins()
            .frame(egui::Frame::new().fill(BG_WINDOW).inner_margin(egui::Margin::symmetric(20, 16)))
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // 找不到中文字体时界面渲染不了中文，提示只能用英文写。
                    if !self.font_ok {
                        card_frame().show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.label(
                                RichText::new("No Chinese-capable font found on this system")
                                    .color(WARN)
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(
                                    "The interface is Chinese-only and cannot be rendered. \
                                     Install any CJK font (Microsoft YaHei, Noto Sans SC, \
                                     Source Han Sans, ...) and restart, or point SC2AGENT_FONT \
                                     at a font file you already have. The command line still works: \
                                     --cli plan|install --from <source>",
                                )
                                .color(TEXT_DIM)
                                .size(12.0),
                            );
                        });
                        ui.add_space(10.0);
                    }
                    section(ui, 1, "来源", |ui| {
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [ui.available_width() - 190.0, 32.0],
                                egui::TextEdit::singleline(&mut self.source_text)
                                    .hint_text("SuperQuail/sc2-agent-dev-kit@v0.1.0a1，或本地发行目录 / zip")
                                    .desired_width(f32::INFINITY),
                            );
                            if ui.button("选目录").clicked() {
                                if let Some(p) = rfd::FileDialog::new().pick_folder() {
                                    self.source_text = p.display().to_string();
                                }
                            }
                            if ui.button("选 zip").clicked() {
                                if let Some(p) = rfd::FileDialog::new().add_filter("发行包", &["zip"]).pick_file() {
                                    self.source_text = p.display().to_string();
                                }
                            }
                        });
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("代理").color(TEXT_DIM).size(12.0));
                            ui.add_sized(
                                [220.0, 28.0],
                                egui::TextEdit::singleline(&mut self.proxy_text)
                                    .hint_text("可选，如 http://127.0.0.1:7897"),
                            );
                            ui.label(
                                RichText::new("从 GitHub 安装时国内通常需要")
                                    .color(TEXT_FAINT)
                                    .size(11.0),
                            );
                        });
                    });

                    section(ui, 2, "目标 harness", |ui| {
                        ui.label(
                            RichText::new("默认不勾选，请自己决定交付到哪里。只有检测到的才能勾。")
                                .color(TEXT_FAINT)
                                .size(11.5),
                        );
                        ui.add_space(8.0);
                        let hs = self.harnesses.clone();
                        let mut chosen = std::mem::take(&mut self.selected);
                        let mut i = 0;
                        while i < hs.len() {
                            let end = (i + 2).min(hs.len());
                            ui.columns(2, |cols| {
                                for (k, h) in hs[i..end].iter().enumerate() {
                                    let mut on = chosen.contains(&h.id.to_string());
                                    harness_card(&mut cols[k], h, &mut on);
                                    if on && !chosen.contains(&h.id.to_string()) {
                                        chosen.push(h.id.to_string());
                                    } else if !on {
                                        chosen.retain(|s| s != h.id);
                                    }
                                }
                            });
                            ui.add_space(8.0);
                            i += 2;
                        }
                        self.selected = chosen;
                    });

                    section(ui, 3, "安装位置", |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(self.root.display().to_string()).color(TEXT_DIM).size(12.0));
                            if ui.button("更改").clicked() {
                                if let Some(p) = rfd::FileDialog::new().pick_folder() {
                                    self.root = p;
                                }
                            }
                        });
                        ui.label(
                            RichText::new("套件只装一份；各 harness 只收到技能，已拍平为 <技能目录>/<名称>/")
                                .color(TEXT_FAINT)
                                .size(11.0),
                        );
                    });

                    section(ui, 4, "进度", |ui| self.progress_ui(ui));
                });
            });
    }
}
impl App {
    fn progress_ui(&mut self, ui: &mut Ui) {
        match self.stage {
            Stage::Planning => {
                ui.label(RichText::new("预演中…").color(TEXT_DIM).size(12.5));
            }
            Stage::Installing => {
                let frac = if self.total > 0 { self.done as f32 / self.total as f32 } else { 0.0 };
                ui.add(
                    egui::ProgressBar::new(frac)
                        .show_percentage()
                        .fill(ACCENT)
                        .desired_height(18.0),
                );
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!("{} / {}  {}", self.done, self.total, self.current))
                        .color(TEXT_DIM)
                        .size(11.5),
                );
            }
            Stage::Idle => match &self.plan {
                None => {
                    ui.label(
                        RichText::new("填好来源后点「预演」看会改什么，或直接「开始安装」。")
                            .color(TEXT_FAINT)
                            .size(12.0),
                    );
                }
                Some(p) => {
                    egui::Grid::new("plan_grid").num_columns(2).spacing([16.0, 6.0]).show(ui, |ui| {
                        let row = |ui: &mut Ui, k: &str, v: RichText| {
                            ui.label(RichText::new(k).color(TEXT_DIM).size(12.0));
                            ui.label(v);
                            ui.end_row();
                        };
                        row(ui, "版本", RichText::new(format!("{}（layout {}）", p.version, p.layout_revision)).color(TEXT).size(12.0));
                        row(ui, "技能数", RichText::new(p.skills.to_string()).color(TEXT).size(12.0));
                        row(ui, "套件本体", RichText::new(format!("{} 个文件：{} 一致，{} 待更新，{} 缺失", p.core.total, p.core.same, p.core.stale, p.core.missing)).color(TEXT).size(12.0));
                        row(ui, "游戏数据", RichText::new(format!("{} 个文件：{} 一致，{} 待更新，{} 缺失", p.data.total, p.data.same, p.data.stale, p.data.missing)).color(TEXT).size(12.0));
                        let (txt, col) = if p.skip_data {
                            (format!("哈希一致 —— 整体跳过，不重写 {:.1} MB", p.data_mb), OK)
                        } else if p.data.total > 0 && p.data.stale == 0 && p.data.missing == 0 {
                            ("已是最新".to_string(), OK)
                        } else {
                            (format!("需要应用（{:.1} MB）", p.data_mb), WARN)
                        };
                        row(ui, "数据包处理", RichText::new(txt).color(col).strong().size(12.0));
                    });
                }
            },
            Stage::Done => {
                ui.label(RichText::new("安装完成").color(OK).strong().size(13.0));
                if let Some(s) = &self.summary {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("版本 {} · 套件位于 {}", s.version, s.kit.display()))
                            .color(TEXT_DIM)
                            .size(11.5),
                    );
                    if s.data_skipped {
                        ui.label(RichText::new("游戏数据未变化，已跳过").color(OK).size(11.5));
                    }
                    for (id, dir, n) in &s.linked {
                        ui.label(
                            RichText::new(format!("{id} → {} （{n} 个技能）", dir.display()))
                                .color(TEXT_DIM)
                                .size(11.5),
                        );
                    }
                }
            }
            Stage::Failed => {
                if let Some(e) = &self.error {
                    ui.label(RichText::new(e).color(ERR).size(12.0));
                }
            }
        }

        if !self.log.is_empty() {
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);
            egui::ScrollArea::vertical()
                .id_salt("log")
                .max_height(140.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for line in &self.log {
                        ui.label(RichText::new(line).color(TEXT_FAINT).size(11.0));
                    }
                });
        }
    }
}
