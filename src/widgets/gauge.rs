use egui::{Color32, Rect, Rounding, Ui, Vec2};

use crate::theme;

pub struct LcarsGauge {
    label: String,
    value: f32, // 0.0 .. 1.0
    color: Color32,
    width: f32,
    font_size: f32,
}

impl LcarsGauge {
    pub fn new(label: impl Into<String>, value: f32, color: Color32) -> Self {
        Self {
            label: label.into(),
            value: value.clamp(0.0, 1.0),
            color,
            width: 300.0,
            font_size: 15.0,
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn show(&self, ui: &mut Ui) {
        let height = theme::GAUGE_HEIGHT;
        let label_width = 80.0;
        let pct_width = 50.0;
        let bar_width = self.width - label_width - pct_width - 8.0;

        ui.horizontal(|ui| {
            // Label
            let (label_rect, _) = ui.allocate_exact_size(Vec2::new(label_width, height), egui::Sense::hover());
            if ui.is_rect_visible(label_rect) {
                ui.painter().text(
                    label_rect.right_center() - egui::vec2(4.0, 0.0),
                    egui::Align2::RIGHT_CENTER,
                    self.label.to_uppercase(),
                    egui::FontId::monospace(self.font_size),
                    self.color,
                );
            }

            // Segmented bar (LCARS barcode style)
            let (bar_rect, _) = ui.allocate_exact_size(Vec2::new(bar_width, height), egui::Sense::hover());
            if ui.is_rect_visible(bar_rect) {
                let segment_width = 4.0;
                let gap = 2.0;
                let step = segment_width + gap;
                let total_segments = ((bar_rect.width()) / step).floor() as usize;
                let filled_segments = ((total_segments as f32) * self.value).round() as usize;
                let seg_rounding = Rounding::same(1.0);

                for i in 0..total_segments {
                    let x = bar_rect.min.x + (i as f32) * step;
                    let seg_rect = Rect::from_min_size(
                        egui::pos2(x, bar_rect.min.y),
                        Vec2::new(segment_width, height),
                    );
                    let color = if i < filled_segments {
                        self.color
                    } else {
                        theme::DARK_BG
                    };
                    ui.painter().rect_filled(seg_rect, seg_rounding, color);
                }
            }

            // Percentage text
            let (pct_rect, _) = ui.allocate_exact_size(Vec2::new(pct_width, height), egui::Sense::hover());
            if ui.is_rect_visible(pct_rect) {
                ui.painter().text(
                    pct_rect.left_center() + egui::vec2(4.0, 0.0),
                    egui::Align2::LEFT_CENTER,
                    format!("{:5.1}%", self.value * 100.0),
                    egui::FontId::monospace(self.font_size),
                    self.color,
                );
            }
        });
    }
}
