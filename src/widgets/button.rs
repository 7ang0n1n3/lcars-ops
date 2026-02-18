use egui::{Color32, Response, Rounding, Sense, Ui, Vec2};

use crate::theme;

#[allow(dead_code)]
pub struct LcarsButton {
    label: String,
    color: Color32,
    size: Vec2,
}

#[allow(dead_code)]
impl LcarsButton {
    pub fn new(label: impl Into<String>, color: Color32) -> Self {
        Self {
            label: label.into(),
            color,
            size: Vec2::new(theme::SIDEBAR_WIDTH, theme::BUTTON_HEIGHT),
        }
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn show(&self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::click());

        if ui.is_rect_visible(rect) {
            let color = if response.hovered() {
                theme::brighten(self.color, 40)
            } else {
                self.color
            };

            let rounding = Rounding::same(rect.height() / 2.0);
            ui.painter().rect_filled(rect, rounding, color);

            let text = self.label.to_uppercase();
            let font = egui::FontId::monospace(15.0);
            let text_color = theme::BLACK;
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                font,
                text_color,
            );
        }

        response
    }
}

#[allow(dead_code)]
pub fn sidebar_decoration(ui: &mut Ui, label: &str, color: Color32, width: f32, height: f32) {
    let (rect, _response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

    if ui.is_rect_visible(rect) {
        let rounding = Rounding::same(height / 2.0);
        ui.painter().rect_filled(rect, rounding, color);

        let font = egui::FontId::monospace(15.0);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label.to_uppercase(),
            font,
            theme::BLACK,
        );
    }
}
