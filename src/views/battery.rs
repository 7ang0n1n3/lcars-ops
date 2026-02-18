use egui::Ui;

use crate::system::battery::BatteryInfo;
use crate::theme;
use crate::widgets::gauge::LcarsGauge;
use crate::widgets::panel::LcarsPanel;

fn battery_color(capacity: u32) -> egui::Color32 {
    if capacity > 50 {
        theme::GREEN
    } else if capacity > 20 {
        theme::YELLOW
    } else {
        theme::RED
    }
}

pub fn show(ui: &mut Ui, bat: &BatteryInfo) {
    if !bat.available {
        ui.add_space(20.0);
        ui.label(
            egui::RichText::new("NO BATTERY DETECTED")
                .color(theme::ORANGE)
                .font(egui::FontId::monospace(40.0)),
        );
        return;
    }

    let full_width = ui.available_width() - 20.0;

    // Usage Panel
    LcarsPanel::new("Usage", theme::GREEN).show(ui, |ui| {
        let charge_color = battery_color(bat.capacity);

        LcarsGauge::new("CHARGE", bat.capacity as f32 / 100.0, charge_color)
            .width(full_width)
            .font_size(20.0)
            .show(ui);
        ui.label(
            egui::RichText::new(format!("          {}", bat.status.to_uppercase()))
                .color(charge_color)
                .font(egui::FontId::monospace(20.0)),
        );

        ui.add_space(8.0);

        const MAX_POWER: f64 = 60.0;
        let power_fraction = (bat.power_now / MAX_POWER).clamp(0.0, 1.0) as f32;

        LcarsGauge::new("POWER", power_fraction, theme::PEACH)
            .width(full_width)
            .font_size(20.0)
            .show(ui);
        ui.label(
            egui::RichText::new(format!("          {:.1} W", bat.power_now))
                .color(theme::PEACH)
                .font(egui::FontId::monospace(20.0)),
        );
    });

    // Properties Panel
    LcarsPanel::new("Properties", theme::LAVENDER).show(ui, |ui| {
        let label_font = egui::FontId::monospace(18.0);
        let value_font = egui::FontId::monospace(22.0);
        let label_color = theme::LAVENDER.linear_multiply(0.65);
        let value_color = theme::LAVENDER;
        let row_h = 30.0;
        let label_w = 220.0;

        let props: &[(&str, String)] = &[
            ("BATTERY HEALTH", format!("{:.0}%", bat.health)),
            ("DESIGN CAPACITY", format!("{:.1} Wh", bat.energy_full_design)),
            ("CHARGE CYCLES", format!("{}", bat.cycle_count)),
            ("TECHNOLOGY", bat.technology.clone()),
            ("MANUFACTURER", bat.manufacturer.clone()),
            ("MODEL NAME", bat.model_name.clone()),
            ("DEVICE", bat.device.clone()),
        ];

        for (label, value) in props {
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(
                    egui::Vec2::new(label_w, row_h),
                    egui::Sense::hover(),
                );
                ui.painter().text(
                    r.left_center(),
                    egui::Align2::LEFT_CENTER,
                    *label,
                    label_font.clone(),
                    label_color,
                );
                ui.label(
                    egui::RichText::new(value.as_str())
                        .color(value_color)
                        .font(value_font.clone()),
                );
            });
            ui.add_space(2.0);
        }
    });
}
