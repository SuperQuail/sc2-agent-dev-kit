//! 配色与视觉风格。
//!
//! 参考 Koishi 的暗色美术风格：深蓝紫底、圆角卡片、低饱和描边、紫色强调色。
//! 只取色彩与排版语言，不沿用它的侧栏布局——本程序是安装向导，不是控制台。

use egui::{Color32, CornerRadius, Stroke, Visuals};

pub const BG_WINDOW: Color32 = Color32::from_rgb(0x11, 0x11, 0x1e);
pub const BG_PANEL: Color32 = Color32::from_rgb(0x18, 0x18, 0x2b);
pub const BG_CARD: Color32 = Color32::from_rgb(0x22, 0x22, 0x37);
pub const BG_CARD_HOVER: Color32 = Color32::from_rgb(0x2a, 0x2a, 0x43);
pub const BG_INPUT: Color32 = Color32::from_rgb(0x1c, 0x1c, 0x30);
pub const BORDER: Color32 = Color32::from_rgb(0x2e, 0x2e, 0x49);
pub const BORDER_LIT: Color32 = Color32::from_rgb(0x44, 0x40, 0x6b);

pub const TEXT: Color32 = Color32::from_rgb(0xea, 0xea, 0xf8);
pub const TEXT_DIM: Color32 = Color32::from_rgb(0x9b, 0x9b, 0xba);
pub const TEXT_FAINT: Color32 = Color32::from_rgb(0x6a, 0x6a, 0x8b);

pub const ACCENT: Color32 = Color32::from_rgb(0x8b, 0x7c, 0xf0);
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(0x9d, 0x90, 0xf5);
pub const ACCENT_DIM: Color32 = Color32::from_rgb(0x54, 0x4a, 0x9e);

pub const OK: Color32 = Color32::from_rgb(0x4a, 0xde, 0x80);
pub const WARN: Color32 = Color32::from_rgb(0xfb, 0xbf, 0x24);
pub const ERR: Color32 = Color32::from_rgb(0xf8, 0x71, 0x71);

pub const CARD_RADIUS: u8 = 10;
pub const BTN_RADIUS: u8 = 8;

pub fn visuals() -> Visuals {
    let mut v = Visuals::dark();
    v.panel_fill = BG_WINDOW;
    v.window_fill = BG_PANEL;
    v.extreme_bg_color = BG_INPUT;
    v.faint_bg_color = BG_CARD;
    v.override_text_color = Some(TEXT);

    let w = &mut v.widgets;
    w.noninteractive.bg_fill = BG_CARD;
    w.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    w.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_DIM);
    w.noninteractive.corner_radius = CornerRadius::same(CARD_RADIUS);

    w.inactive.bg_fill = BG_CARD;
    w.inactive.weak_bg_fill = BG_CARD;
    w.inactive.bg_stroke = Stroke::new(1.0, BORDER);
    w.inactive.fg_stroke = Stroke::new(1.0, TEXT);
    w.inactive.corner_radius = CornerRadius::same(BTN_RADIUS);

    w.hovered.bg_fill = BG_CARD_HOVER;
    w.hovered.weak_bg_fill = BG_CARD_HOVER;
    w.hovered.bg_stroke = Stroke::new(1.0, BORDER_LIT);
    w.hovered.fg_stroke = Stroke::new(1.0, TEXT);
    w.hovered.corner_radius = CornerRadius::same(BTN_RADIUS);

    w.active.bg_fill = ACCENT_DIM;
    w.active.weak_bg_fill = ACCENT_DIM;
    w.active.bg_stroke = Stroke::new(1.0, ACCENT);
    w.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    w.active.corner_radius = CornerRadius::same(BTN_RADIUS);

    w.open.bg_fill = BG_CARD_HOVER;
    w.open.weak_bg_fill = BG_CARD_HOVER;
    w.open.bg_stroke = Stroke::new(1.0, BORDER_LIT);
    w.open.corner_radius = CornerRadius::same(CARD_RADIUS);

    v.window_corner_radius = CornerRadius::same(CARD_RADIUS);
    v.window_stroke = Stroke::new(1.0, BORDER);
    v.selection.bg_fill = ACCENT_DIM;
    v.selection.stroke = Stroke::new(1.0, TEXT);
    v.hyperlink_color = ACCENT;
    v
}

/// 卡片外框：圆角 + 描边 + 与背景可区分的填充色。
pub fn card_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(BG_CARD)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(CARD_RADIUS))
        .inner_margin(egui::Margin::symmetric(16, 14))
}
