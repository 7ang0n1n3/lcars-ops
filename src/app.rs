use egui::{Pos2, Rect, Rounding, Vec2};

use crate::system::battery::BatteryInfo;
use crate::system::gpu::GpuInfo;
use crate::system::info::SystemInfo;
use crate::system::process::ProcessView;
use crate::theme;
use crate::widgets::elbow::{ElbowCorner, LcarsElbow};

#[derive(PartialEq, Clone, Copy)]
pub enum View {
    Dashboard,
    Processes,
    Battery,
    Gpu,
}

pub struct LcarsApp {
    sys_info: SystemInfo,
    process_view: ProcessView,
    battery_info: BatteryInfo,
    gpu_info: GpuInfo,
    current_view: View,
}

impl LcarsApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "helvetica_uc".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(
                include_bytes!("../Helvetica Ultra Compressed.otf"),
            )),
        );
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "helvetica_uc".to_owned());
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
            .insert(0, "helvetica_uc".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        Self {
            sys_info: SystemInfo::new(),
            process_view: ProcessView::default(),
            battery_info: BatteryInfo::new(),
            gpu_info: GpuInfo::new(),
            current_view: View::Dashboard,
        }
    }

    fn stardate() -> String {
        let now = chrono::Local::now();
        let year = now.format("%Y").to_string();
        let day_of_year = now.format("%j").to_string();
        let fraction = now.format("%H").to_string().parse::<f32>().unwrap_or(0.0) / 24.0;
        format!("{}{}.{:.0}", &year[2..], day_of_year, fraction * 10.0)
    }
}

impl eframe::App for LcarsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sys_info.refresh_if_needed();
        self.battery_info.refresh_if_needed();
        self.gpu_info.refresh_if_needed();
        ctx.request_repaint_after(std::time::Duration::from_millis(500));

        if ctx.input(|i| i.key_pressed(egui::Key::Q)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // Set dark background
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = theme::BLACK;
        visuals.window_fill = theme::BLACK;
        visuals.extreme_bg_color = theme::BLACK;
        ctx.set_visuals(visuals);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(theme::BLACK).inner_margin(0.0))
            .show(ctx, |ui| {
                let total_rect = ui.max_rect();
                let padding = 8.0;

                let sidebar_w = theme::SIDEBAR_WIDTH;
                let header_h = theme::HEADER_HEIGHT;
                let footer_h = theme::FOOTER_HEIGHT;
                let elbow_r = theme::ELBOW_RADIUS;

                // === Top-left elbow ===
                let top_elbow_rect = Rect::from_min_size(
                    Pos2::new(total_rect.min.x + padding, total_rect.min.y + padding),
                    Vec2::new(sidebar_w + elbow_r, header_h + elbow_r),
                );
                LcarsElbow::new(theme::ORANGE, ElbowCorner::TopLeft).draw(
                    ui,
                    top_elbow_rect,
                    sidebar_w,
                    header_h,
                    elbow_r,
                );

                // === Header bar (right of elbow): main section + end cap ===
                let header_bar_x = total_rect.min.x + padding + sidebar_w + elbow_r;
                let header_bar_total_w = total_rect.width() - padding * 2.0 - sidebar_w - elbow_r;
                let cap_w = header_h; // end cap width
                let header_main_w = header_bar_total_w - cap_w;

                // Main header bar (flat edges)
                let header_main_rect = Rect::from_min_size(
                    Pos2::new(header_bar_x, total_rect.min.y + padding),
                    Vec2::new(header_main_w, header_h),
                );
                ui.painter().rect_filled(header_main_rect, Rounding::ZERO, theme::ORANGE);

                // Header text
                let title_text = format!("LCARS-OPS   SYSTEM MONITOR   SD:{}", Self::stardate());
                ui.painter().text(
                    header_main_rect.center() + egui::vec2(0.0, 6.0),
                    egui::Align2::CENTER_CENTER,
                    title_text,
                    egui::FontId::monospace(40.0),
                    theme::BLACK,
                );

                // Header end cap (rounded right side)
                let header_cap_rect = Rect::from_min_size(
                    Pos2::new(header_main_rect.max.x, total_rect.min.y + padding),
                    Vec2::new(cap_w, header_h),
                );
                ui.painter().rect_filled(
                    header_cap_rect,
                    Rounding { nw: 0.0, ne: header_h / 2.0, sw: 0.0, se: header_h / 2.0 },
                    theme::BLUE,
                );

                // === Bottom-left elbow ===
                let bottom_elbow_rect = Rect::from_min_size(
                    Pos2::new(
                        total_rect.min.x + padding,
                        total_rect.max.y - padding - footer_h - elbow_r,
                    ),
                    Vec2::new(sidebar_w + elbow_r, footer_h + elbow_r),
                );
                LcarsElbow::new(theme::LAVENDER, ElbowCorner::BottomLeft).draw(
                    ui,
                    bottom_elbow_rect,
                    sidebar_w,
                    footer_h,
                    elbow_r,
                );

                // === Footer bar (right of elbow): main section + end cap ===
                let footer_bar_x = total_rect.min.x + padding + sidebar_w + elbow_r;
                let footer_bar_total_w = total_rect.width() - padding * 2.0 - sidebar_w - elbow_r;
                let footer_cap_w = footer_h;
                let footer_main_w = footer_bar_total_w - footer_cap_w;

                // Main footer bar (flat edges)
                let footer_main_rect = Rect::from_min_size(
                    Pos2::new(footer_bar_x, total_rect.max.y - padding - footer_h),
                    Vec2::new(footer_main_w, footer_h),
                );
                ui.painter().rect_filled(footer_main_rect, Rounding::ZERO, theme::LAVENDER);

                // Footer text
                ui.painter().text(
                    footer_main_rect.center() + egui::vec2(0.0, 6.0),
                    egui::Align2::CENTER_CENTER,
                    "UNITED FEDERATION OF PLANETS",
                    egui::FontId::monospace(40.0),
                    theme::BLACK,
                );

                // Footer end cap (rounded right side)
                let footer_cap_rect = Rect::from_min_size(
                    Pos2::new(footer_main_rect.max.x, total_rect.max.y - padding - footer_h),
                    Vec2::new(footer_cap_w, footer_h),
                );
                ui.painter().rect_filled(
                    footer_cap_rect,
                    Rounding { nw: 0.0, ne: footer_h / 2.0, sw: 0.0, se: footer_h / 2.0 },
                    theme::MAGENTA,
                );

                // === Sidebar buttons (between elbows) ===
                let sidebar_top = total_rect.min.y + padding + header_h + elbow_r + theme::BAR_SPACING;
                let sidebar_bottom =
                    total_rect.max.y - padding - footer_h - elbow_r - theme::BAR_SPACING;
                let sidebar_x = total_rect.min.x + padding;

                // View switching buttons
                let button_h = theme::BUTTON_HEIGHT;
                let mut y = sidebar_top;

                // Dashboard button
                let dash_rect = Rect::from_min_size(
                    Pos2::new(sidebar_x, y),
                    Vec2::new(sidebar_w, button_h),
                );
                let dash_color = if self.current_view == View::Dashboard {
                    theme::ORANGE
                } else {
                    theme::PEACH
                };
                let dash_resp = ui.allocate_rect(dash_rect, egui::Sense::click());
                let dash_draw_color = if dash_resp.hovered() {
                    theme::brighten(dash_color, 40)
                } else {
                    dash_color
                };
                let btn_rounding = Rounding { nw: 0.0, ne: button_h / 2.0, sw: 0.0, se: button_h / 2.0 };
                ui.painter().rect_filled(
                    dash_rect,
                    btn_rounding,
                    dash_draw_color,
                );
                ui.painter().text(
                    dash_rect.right_center() - egui::vec2(button_h / 2.0 + 4.0, -3.0),
                    egui::Align2::RIGHT_CENTER,
                    "SYSTEMS",
                    egui::FontId::monospace(30.0),
                    theme::BLACK,
                );
                if dash_resp.clicked() {
                    self.current_view = View::Dashboard;
                }

                y += button_h + theme::BAR_SPACING;

                // Processes button
                let proc_rect = Rect::from_min_size(
                    Pos2::new(sidebar_x, y),
                    Vec2::new(sidebar_w, button_h),
                );
                let proc_color = if self.current_view == View::Processes {
                    theme::ORANGE
                } else {
                    theme::BLUE
                };
                let proc_resp = ui.allocate_rect(proc_rect, egui::Sense::click());
                let proc_draw_color = if proc_resp.hovered() {
                    theme::brighten(proc_color, 40)
                } else {
                    proc_color
                };
                ui.painter().rect_filled(
                    proc_rect,
                    btn_rounding,
                    proc_draw_color,
                );
                ui.painter().text(
                    proc_rect.right_center() - egui::vec2(button_h / 2.0 + 4.0, -3.0),
                    egui::Align2::RIGHT_CENTER,
                    "PROCESSES",
                    egui::FontId::monospace(30.0),
                    theme::BLACK,
                );
                if proc_resp.clicked() {
                    self.current_view = View::Processes;
                }

                y += button_h + theme::BAR_SPACING;

                // Battery button
                let bat_rect = Rect::from_min_size(
                    Pos2::new(sidebar_x, y),
                    Vec2::new(sidebar_w, button_h),
                );
                let bat_color = if self.current_view == View::Battery {
                    theme::ORANGE
                } else {
                    theme::PERIWINKLE
                };
                let bat_resp = ui.allocate_rect(bat_rect, egui::Sense::click());
                let bat_draw_color = if bat_resp.hovered() {
                    theme::brighten(bat_color, 40)
                } else {
                    bat_color
                };
                ui.painter().rect_filled(bat_rect, btn_rounding, bat_draw_color);
                ui.painter().text(
                    bat_rect.right_center() - egui::vec2(button_h / 2.0 + 4.0, -3.0),
                    egui::Align2::RIGHT_CENTER,
                    "BATTERY",
                    egui::FontId::monospace(30.0),
                    theme::BLACK,
                );
                if bat_resp.clicked() {
                    self.current_view = View::Battery;
                }

                y += button_h + theme::BAR_SPACING;

                // GPU button
                let gpu_rect = Rect::from_min_size(
                    Pos2::new(sidebar_x, y),
                    Vec2::new(sidebar_w, button_h),
                );
                let gpu_color = if self.current_view == View::Gpu {
                    theme::ORANGE
                } else {
                    theme::MAGENTA
                };
                let gpu_resp = ui.allocate_rect(gpu_rect, egui::Sense::click());
                let gpu_draw_color = if gpu_resp.hovered() {
                    theme::brighten(gpu_color, 40)
                } else {
                    gpu_color
                };
                ui.painter().rect_filled(gpu_rect, btn_rounding, gpu_draw_color);
                ui.painter().text(
                    gpu_rect.right_center() - egui::vec2(button_h / 2.0 + 4.0, -3.0),
                    egui::Align2::RIGHT_CENTER,
                    "GPU",
                    egui::FontId::monospace(30.0),
                    theme::BLACK,
                );
                if gpu_resp.clicked() {
                    self.current_view = View::Gpu;
                }

                y += button_h + theme::BAR_SPACING * 3.0;

                // Decorative labels
                let remaining = sidebar_bottom - y;
                let num_decos = ((remaining / (button_h + theme::BAR_SPACING)) as usize)
                    .min(theme::SIDEBAR_LABELS.len());

                for i in 0..num_decos {
                    let deco_rect = Rect::from_min_size(
                        Pos2::new(sidebar_x, y),
                        Vec2::new(sidebar_w, button_h),
                    );
                    // Use allocate_rect so egui knows this space is taken
                    let _ = ui.allocate_rect(deco_rect, egui::Sense::hover());
                    let color = theme::color_for_index(i + 2);
                    ui.painter().rect_filled(
                        deco_rect,
                        btn_rounding,
                        color,
                    );
                    ui.painter().text(
                        deco_rect.right_center() - egui::vec2(button_h / 2.0 + 4.0, -3.0),
                        egui::Align2::RIGHT_CENTER,
                        theme::SIDEBAR_LABELS[i],
                        egui::FontId::monospace(30.0),
                        theme::BLACK,
                    );
                    y += button_h + theme::BAR_SPACING;
                }

                // === Content area ===
                let content_left =
                    total_rect.min.x + padding + sidebar_w + elbow_r + theme::BAR_SPACING;
                let content_top =
                    total_rect.min.y + padding + header_h + theme::BAR_SPACING;
                let content_right = total_rect.max.x - padding;
                let content_bottom =
                    total_rect.max.y - padding - footer_h - theme::BAR_SPACING;
                let content_rect = Rect::from_min_max(
                    Pos2::new(content_left, content_top),
                    Pos2::new(content_right, content_bottom),
                );

                let mut content_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(content_rect),
                );

                // Sticky header for processes (outside scroll)
                if self.current_view == View::Processes {
                    content_ui.add_space(8.0);
                    crate::views::processes::show_header(&mut content_ui, &mut self.process_view);
                }

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(&mut content_ui, |ui| {
                        match self.current_view {
                            View::Dashboard => {
                                ui.add_space(8.0);
                                crate::views::dashboard::show(ui, &self.sys_info);
                            }
                            View::Processes => {
                                crate::views::processes::show_rows(
                                    ui,
                                    &mut self.process_view,
                                    &self.sys_info.system,
                                    &self.sys_info.users,
                                );
                            }
                            View::Battery => {
                                ui.add_space(8.0);
                                crate::views::battery::show(ui, &self.battery_info);
                            }
                            View::Gpu => {
                                ui.add_space(8.0);
                                crate::views::gpu::show(ui, &self.gpu_info);
                            }
                        }
                    });
            });
    }
}
