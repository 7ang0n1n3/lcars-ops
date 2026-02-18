use egui::Ui;

use crate::system::info::{format_bytes, rate_fraction, NetworkData, SystemInfo};
use crate::theme;
use crate::widgets::gauge::LcarsGauge;
use crate::widgets::panel::LcarsPanel;

pub fn show(ui: &mut Ui, sys: &SystemInfo) {
    // CPU Panel
    LcarsPanel::new("Processor", theme::ORANGE).show(ui, |ui| {
        let full_width = ui.available_width() - 20.0;

        // Total spans full width
        LcarsGauge::new("Total", sys.cpu_total() / 100.0, theme::ORANGE)
            .width(full_width)
            .font_size(20.0)
            .show(ui);

        let cores = sys.cpu_per_core();
        let colors = [theme::PEACH, theme::BLUE, theme::PERIWINKLE, theme::LAVENDER];
        let half = (cores.len() + 1) / 2;
        let col_width = (full_width - 16.0) / 2.0;

        ui.horizontal(|ui| {
            // Left column
            ui.vertical(|ui| {
                for (i, usage) in cores[..half].iter().enumerate() {
                    let label = format!("Core {}", i);
                    let color = colors[i % colors.len()];
                    LcarsGauge::new(label, usage / 100.0, color)
                        .width(col_width)
                        .font_size(20.0)
                        .show(ui);
                }
            });

            // Right column
            ui.vertical(|ui| {
                for (i, usage) in cores[half..].iter().enumerate() {
                    let idx = i + half;
                    let label = format!("Core {}", idx);
                    let color = colors[idx % colors.len()];
                    LcarsGauge::new(label, usage / 100.0, color)
                        .width(col_width)
                        .font_size(20.0)
                        .show(ui);
                }
            });
        });
    });

    // Memory Panel
    LcarsPanel::new("Memory", theme::PEACH).show(ui, |ui| {
        let full_width = ui.available_width() - 20.0;
        let col_width = (full_width - 16.0) / 2.0;

        ui.horizontal(|ui| {
            // RAM column
            ui.vertical(|ui| {
                LcarsGauge::new("RAM", sys.memory_fraction(), theme::PEACH)
                    .width(col_width)
                    .font_size(20.0)
                    .show(ui);
                ui.label(
                    egui::RichText::new(format!(
                        "          {} / {}",
                        format_bytes(sys.memory_used()),
                        format_bytes(sys.memory_total())
                    ))
                    .color(theme::PEACH)
                    .font(egui::FontId::monospace(20.0)),
                );
            });

            // Swap column
            ui.vertical(|ui| {
                LcarsGauge::new("Swap", sys.swap_fraction(), theme::LAVENDER)
                    .width(col_width)
                    .font_size(20.0)
                    .show(ui);
                ui.label(
                    egui::RichText::new(format!(
                        "          {} / {}",
                        format_bytes(sys.swap_used()),
                        format_bytes(sys.swap_total())
                    ))
                    .color(theme::LAVENDER)
                    .font(egui::FontId::monospace(20.0)),
                );
            });
        });
    });

    // Disk Panel
    let disks = sys.disk_info();
    if !disks.is_empty() {
        LcarsPanel::new("Storage", theme::PERIWINKLE).show(ui, |ui| {
            let full_width = ui.available_width() - 20.0;
            let col_width = (full_width - 16.0) / 2.0;
            let half = (disks.len() + 1) / 2;

            ui.horizontal(|ui| {
                // Left column
                ui.vertical(|ui| {
                    for disk in &disks[..half] {
                        let label = if disk.mount.len() > 10 {
                            format!("..{}", &disk.mount[disk.mount.len() - 8..])
                        } else {
                            disk.mount.clone()
                        };
                        let color = theme::disk_color(disk.fraction);
                        LcarsGauge::new(label, disk.fraction, color)
                            .width(col_width)
                            .font_size(20.0)
                            .show(ui);
                        ui.label(
                            egui::RichText::new(format!(
                                "          {} / {}",
                                format_bytes(disk.used),
                                format_bytes(disk.total)
                            ))
                            .color(color)
                            .font(egui::FontId::monospace(20.0)),
                        );
                    }
                });

                // Right column
                ui.vertical(|ui| {
                    for disk in &disks[half..] {
                        let label = if disk.mount.len() > 10 {
                            format!("..{}", &disk.mount[disk.mount.len() - 8..])
                        } else {
                            disk.mount.clone()
                        };
                        let color = theme::disk_color(disk.fraction);
                        LcarsGauge::new(label, disk.fraction, color)
                            .width(col_width)
                            .font_size(20.0)
                            .show(ui);
                        ui.label(
                            egui::RichText::new(format!(
                                "          {} / {}",
                                format_bytes(disk.used),
                                format_bytes(disk.total)
                            ))
                            .color(color)
                            .font(egui::FontId::monospace(20.0)),
                        );
                    }
                });
            });
        });
    }

    // Network Panel
    let nets = sys.network_info();
    if !nets.is_empty() {
        LcarsPanel::new("Network", theme::BLUE).show(ui, |ui| {
            let full_width = ui.available_width() - 20.0;
            let col_width = (full_width - 16.0) / 2.0;
            let half = (nets.len() + 1) / 2;

            ui.horizontal(|ui| {
                // Left column
                ui.vertical(|ui| {
                    for net in &nets[..half] {
                        show_network_iface(ui, net, col_width);
                    }
                });

                // Right column
                ui.vertical(|ui| {
                    for net in &nets[half..] {
                        show_network_iface(ui, net, col_width);
                    }
                });
            });
        });
    }
}

fn format_rate(bytes_per_sec: f64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec as u64))
}

fn show_network_iface(ui: &mut Ui, net: &NetworkData, width: f32) {
    ui.label(
        egui::RichText::new(net.name.to_uppercase())
            .color(theme::BLUE)
            .font(egui::FontId::monospace(20.0)),
    );

    LcarsGauge::new("RX", rate_fraction(net.rx_rate), theme::BLUE)
        .width(width)
        .font_size(20.0)
        .show(ui);
    ui.label(
        egui::RichText::new(format!(
            "          {} ({})",
            format_rate(net.rx_rate),
            format_bytes(net.rx_bytes)
        ))
        .color(theme::BLUE)
        .font(egui::FontId::monospace(20.0)),
    );

    LcarsGauge::new("TX", rate_fraction(net.tx_rate), theme::PEACH)
        .width(width)
        .font_size(20.0)
        .show(ui);
    ui.label(
        egui::RichText::new(format!(
            "          {} ({})",
            format_rate(net.tx_rate),
            format_bytes(net.tx_bytes)
        ))
        .color(theme::PEACH)
        .font(egui::FontId::monospace(20.0)),
    );

    ui.add_space(8.0);
}
