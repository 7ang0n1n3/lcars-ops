use egui::Color32;

// LCARS color palette
pub const ORANGE: Color32 = Color32::from_rgb(0xFF, 0x99, 0x00);
pub const PEACH: Color32 = Color32::from_rgb(0xFF, 0xCC, 0x99);
pub const LAVENDER: Color32 = Color32::from_rgb(0xCC, 0x99, 0xCC);
pub const PERIWINKLE: Color32 = Color32::from_rgb(0x99, 0x99, 0xFF);
pub const MAGENTA: Color32 = Color32::from_rgb(0xCC, 0x66, 0x99);
pub const BLUE: Color32 = Color32::from_rgb(0x99, 0xCC, 0xFF);
pub const BLACK: Color32 = Color32::from_rgb(0x00, 0x00, 0x00);
pub const DARK_BG: Color32 = Color32::from_rgb(0x1A, 0x1A, 0x2E);
pub const GREEN: Color32 = Color32::from_rgb(0x33, 0xCC, 0x66);
pub const YELLOW: Color32 = Color32::from_rgb(0xFF, 0xCC, 0x00);
pub const RED: Color32 = Color32::from_rgb(0xFF, 0x33, 0x33);

pub fn disk_color(fraction: f32) -> Color32 {
    if fraction <= 0.49 {
        GREEN
    } else if fraction <= 0.80 {
        YELLOW
    } else {
        RED
    }
}

// Child row styling
pub const CHILD_DIM: f32 = 0.55;
pub const CHILD_ROW_H: f32 = 28.0;

// Layout dimensions
pub const SIDEBAR_WIDTH: f32 = 140.0;
pub const HEADER_HEIGHT: f32 = 50.0;
pub const FOOTER_HEIGHT: f32 = 50.0;
pub const ELBOW_RADIUS: f32 = 40.0;
pub const BAR_SPACING: f32 = 4.0;
pub const BUTTON_HEIGHT: f32 = 48.0;
pub const GAUGE_HEIGHT: f32 = 22.0;

// Decorative sidebar labels
pub const SIDEBAR_LABELS: &[&str] = &[
    "47-1138",
    "21-4077",
    "09-7461",
    "63-2501",
    "88-3014",
    "15-9927",
];

// Color rotation for sidebar buttons
pub const SIDEBAR_COLORS: &[Color32] = &[
    PEACH, LAVENDER, PERIWINKLE, MAGENTA, BLUE, ORANGE,
];

pub fn color_for_index(i: usize) -> Color32 {
    SIDEBAR_COLORS[i % SIDEBAR_COLORS.len()]
}

pub fn brighten(color: Color32, amount: u8) -> Color32 {
    Color32::from_rgb(
        color.r().saturating_add(amount),
        color.g().saturating_add(amount),
        color.b().saturating_add(amount),
    )
}
