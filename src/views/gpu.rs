use egui::Ui;

use crate::system::gpu::GpuInfo;
use crate::system::info::format_bytes;
use crate::theme;
use crate::widgets::gauge::LcarsGauge;
use crate::widgets::panel::LcarsPanel;

pub fn show(ui: &mut Ui, gpu: &GpuInfo) {
    if !gpu.available {
        ui.add_space(20.0);
        ui.label(
            egui::RichText::new("NO GPU DETECTED")
                .color(theme::ORANGE)
                .font(egui::FontId::monospace(40.0)),
        );
        return;
    }

    let full_width = ui.available_width() - 20.0;

    // Usage Panel
    LcarsPanel::new("Usage", theme::MAGENTA).show(ui, |ui| {
        LcarsGauge::new("TOTAL", gpu.gpu_usage as f32 / 100.0, theme::MAGENTA)
            .width(full_width)
            .font_size(20.0)
            .show(ui);

        ui.add_space(8.0);

        let vram_fraction = if gpu.vram_total > 0 {
            gpu.vram_used as f32 / gpu.vram_total as f32
        } else {
            0.0
        };

        LcarsGauge::new("VRAM", vram_fraction, theme::PEACH)
            .width(full_width)
            .font_size(20.0)
            .show(ui);
        ui.label(
            egui::RichText::new(format!(
                "          {} / {}  \u{2022}  {:.0}%",
                format_bytes(gpu.vram_used),
                format_bytes(gpu.vram_total),
                vram_fraction * 100.0,
            ))
            .color(theme::PEACH)
            .font(egui::FontId::monospace(20.0)),
        );

        ui.add_space(8.0);

        let freq_str = gpu
            .gpu_freq_mhz
            .map(|f| format!("{:.2} MHz", f))
            .unwrap_or_else(|| "N/A".to_string());
        let mfreq_str = gpu
            .mem_freq_mhz
            .map(|f| format!("{:.2} MHz", f))
            .unwrap_or_else(|| "N/A".to_string());

        show_stat(ui, "GPU FREQUENCY", &freq_str, theme::MAGENTA);
        show_stat(ui, "MEM FREQUENCY", &mfreq_str, theme::MAGENTA);
        show_stat(ui, "POWER USAGE", &format!("{:.1} W", gpu.power_w), theme::MAGENTA);
    });

    // Sensors Panel
    LcarsPanel::new("Sensors", theme::RED).show(ui, |ui| {
        let temp_max = if gpu.temp_max > 0.0 { gpu.temp_max } else { gpu.temp_celsius };
        let temp_fraction = (gpu.temp_celsius / 120.0).clamp(0.0, 1.0);

        LcarsGauge::new("TEMP", temp_fraction, theme::RED)
            .width(full_width)
            .font_size(20.0)
            .show(ui);
        ui.label(
            egui::RichText::new(format!(
                "          {:.0}\u{00b0}C  \u{2022}  Highest: {:.0}\u{00b0}C",
                gpu.temp_celsius, temp_max
            ))
            .color(theme::RED)
            .font(egui::FontId::monospace(20.0)),
        );
    });

    // Properties Panel
    LcarsPanel::new("Properties", theme::PERIWINKLE).show(ui, |ui| {
        let power_cap_str = gpu
            .power_cap_w
            .map(|p| format!("{:.1} W", p))
            .unwrap_or_else(|| "N/A".to_string());

        let props: &[(&str, String)] = &[
            ("MANUFACTURER", gpu.manufacturer.clone()),
            ("PCI SLOT", gpu.pci_slot.clone()),
            ("DRIVER", gpu.driver.clone()),
            ("MAX POWER CAP", power_cap_str),
            ("LINK", gpu.pcie_link.clone()),
        ];

        for (label, value) in props {
            show_prop(ui, label, value);
        }
    });
}

fn show_stat(ui: &mut Ui, label: &str, value: &str, color: egui::Color32) {
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(
            egui::Vec2::new(200.0, 28.0),
            egui::Sense::hover(),
        );
        ui.painter().text(
            r.left_center(),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::monospace(18.0),
            color.linear_multiply(0.65),
        );
        ui.label(
            egui::RichText::new(value)
                .color(color)
                .font(egui::FontId::monospace(20.0)),
        );
    });
    ui.add_space(2.0);
}

fn show_prop(ui: &mut Ui, label: &str, value: &String) {
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(
            egui::Vec2::new(200.0, 30.0),
            egui::Sense::hover(),
        );
        ui.painter().text(
            r.left_center(),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::monospace(18.0),
            theme::PERIWINKLE.linear_multiply(0.65),
        );
        ui.label(
            egui::RichText::new(value)
                .color(theme::PERIWINKLE)
                .font(egui::FontId::monospace(22.0)),
        );
    });
    ui.add_space(2.0);
}
