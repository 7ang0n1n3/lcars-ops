use egui::{Color32, Ui};

use crate::theme;

pub struct LcarsPanel {
    title: String,
    color: Color32,
}

impl LcarsPanel {
    pub fn new(title: impl Into<String>, color: Color32) -> Self {
        Self {
            title: title.into(),
            color,
        }
    }

    pub fn show(&self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        ui.vertical(|ui| {
            // Title bar
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(6.0, 16.0),
                    egui::Sense::hover(),
                );
                if ui.is_rect_visible(rect) {
                    ui.painter().rect_filled(rect, egui::Rounding::same(3.0), self.color);
                }

                ui.label(
                    egui::RichText::new(self.title.to_uppercase())
                        .color(self.color)
                        .font(egui::FontId::monospace(30.0)),
                );

                // Decorative line
                let available = ui.available_width();
                if available > 20.0 {
                    let (line_rect, _) = ui.allocate_exact_size(
                        egui::Vec2::new(available - 8.0, 2.0),
                        egui::Sense::hover(),
                    );
                    if ui.is_rect_visible(line_rect) {
                        ui.painter().rect_filled(
                            line_rect,
                            egui::Rounding::ZERO,
                            self.color.linear_multiply(0.3),
                        );
                    }
                }
            });

            ui.add_space(4.0);

            // Content area
            ui.indent("panel_content", |ui| {
                content(ui);
            });

            ui.add_space(theme::BAR_SPACING * 2.0);
        });
    }
}
