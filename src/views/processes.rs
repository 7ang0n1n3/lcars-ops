use egui::{Ui, Vec2};

use crate::system::info::format_bytes;
use crate::system::process::{ProcessView, SortColumn, SortOrder};
use crate::theme;
use sysinfo::{System, Users};

fn arrow_str(pv: &ProcessView, col: SortColumn) -> &'static str {
    if pv.sort_column == col {
        match pv.sort_order {
            SortOrder::Ascending => " ^",
            SortOrder::Descending => " v",
        }
    } else {
        ""
    }
}

fn cpu_color(usage: f32) -> egui::Color32 {
    if usage <= 49.0 {
        theme::GREEN
    } else if usage <= 80.0 {
        theme::YELLOW
    } else {
        theme::RED
    }
}

// Column widths
const NAME_W: f32 = 220.0;
const PID_W:  f32 = 110.0;
const USER_W: f32 = 120.0;
const MEM_W:  f32 = 130.0;
const CPU_W:  f32 = 100.0;
const GPU_W:  f32 = 90.0;
const ROW_H:  f32 = 34.0;

// Child column widths
const CHILD_NAME_W: f32 = 160.0;
const CHILD_PID_W:  f32 = 80.0;
const CHILD_USER_W: f32 = 90.0;
const CHILD_MEM_W:  f32 = 100.0;
const CHILD_CPU_W:  f32 = 80.0;

/// Draw the sticky column headers (call outside scroll area)
pub fn show_header(ui: &mut Ui, pv: &mut ProcessView) {
    let font = egui::FontId::monospace(28.0);

    ui.horizontal(|ui| {
        // PROCESS (name) — sortable
        let (r, _) = ui.allocate_exact_size(Vec2::new(NAME_W, ROW_H), egui::Sense::click());
        if ui.allocate_rect(r, egui::Sense::click()).clicked() {
            pv.toggle_sort(SortColumn::Name);
        }
        ui.painter().text(
            r.left_center() + egui::vec2(4.0, 0.0),
            egui::Align2::LEFT_CENTER,
            format!("PROCESS{}", arrow_str(pv, SortColumn::Name)),
            font.clone(),
            theme::ORANGE,
        );

        // PID — sortable
        let (r, _) = ui.allocate_exact_size(Vec2::new(PID_W, ROW_H), egui::Sense::click());
        if ui.allocate_rect(r, egui::Sense::click()).clicked() {
            pv.toggle_sort(SortColumn::Pid);
        }
        ui.painter().text(
            r.left_center() + egui::vec2(4.0, 0.0),
            egui::Align2::LEFT_CENTER,
            format!("PROCESS ID{}", arrow_str(pv, SortColumn::Pid)),
            font.clone(),
            theme::ORANGE,
        );

        // USER — sortable
        let (r, _) = ui.allocate_exact_size(Vec2::new(USER_W, ROW_H), egui::Sense::click());
        if ui.allocate_rect(r, egui::Sense::click()).clicked() {
            pv.toggle_sort(SortColumn::User);
        }
        ui.painter().text(
            r.left_center() + egui::vec2(4.0, 0.0),
            egui::Align2::LEFT_CENTER,
            format!("USER{}", arrow_str(pv, SortColumn::User)),
            font.clone(),
            theme::ORANGE,
        );

        // MEMORY — sortable, right-aligned
        let (r, _) = ui.allocate_exact_size(Vec2::new(MEM_W, ROW_H), egui::Sense::click());
        if ui.allocate_rect(r, egui::Sense::click()).clicked() {
            pv.toggle_sort(SortColumn::Memory);
        }
        ui.painter().text(
            r.right_center() - egui::vec2(4.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            format!("MEMORY{}", arrow_str(pv, SortColumn::Memory)),
            font.clone(),
            theme::ORANGE,
        );

        // PROCESSOR (CPU%) — sortable, right-aligned
        let (r, _) = ui.allocate_exact_size(Vec2::new(CPU_W, ROW_H), egui::Sense::click());
        if ui.allocate_rect(r, egui::Sense::click()).clicked() {
            pv.toggle_sort(SortColumn::Cpu);
        }
        ui.painter().text(
            r.right_center() - egui::vec2(4.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            format!("PROCESSOR{}", arrow_str(pv, SortColumn::Cpu)),
            font.clone(),
            theme::ORANGE,
        );

        // GPU — static, right-aligned
        let (r, _) = ui.allocate_exact_size(Vec2::new(GPU_W, ROW_H), egui::Sense::hover());
        ui.painter().text(
            r.right_center() - egui::vec2(4.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            "GPU",
            font.clone(),
            theme::ORANGE,
        );
    });

    // Separator line
    let (sep_rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 2.0), egui::Sense::hover());
    if ui.is_rect_visible(sep_rect) {
        ui.painter().rect_filled(
            sep_rect,
            egui::Rounding::ZERO,
            theme::ORANGE.linear_multiply(0.4),
        );
    }

    ui.add_space(2.0);
}

/// Draw the scrollable process rows (call inside scroll area)
pub fn show_rows(ui: &mut Ui, pv: &mut ProcessView, system: &System, users: &Users) {
    let procs = pv.get_processes(system, users);
    let font = egui::FontId::monospace(28.0);
    let row_colors = [theme::PEACH, theme::BLUE];

    let mut toggle_pid: Option<u32> = None;

    for (i, proc_info) in procs.iter().enumerate() {
        let base_color = row_colors[i % 2].linear_multiply(0.8);
        let expanded = pv.is_expanded(proc_info.pid);

        let row_resp = ui.horizontal(|ui| {
            // PROCESS name
            let (r, _) = ui.allocate_exact_size(Vec2::new(NAME_W, ROW_H), egui::Sense::hover());
            let name = if proc_info.name.len() > 16 {
                format!("{:.16}", proc_info.name)
            } else {
                proc_info.name.clone()
            };
            ui.painter().text(
                r.left_center() + egui::vec2(4.0, 0.0),
                egui::Align2::LEFT_CENTER,
                name,
                font.clone(),
                base_color,
            );

            // PID
            let (r, _) = ui.allocate_exact_size(Vec2::new(PID_W, ROW_H), egui::Sense::hover());
            ui.painter().text(
                r.left_center() + egui::vec2(4.0, 0.0),
                egui::Align2::LEFT_CENTER,
                format!("{}", proc_info.pid),
                font.clone(),
                base_color,
            );

            // USER
            let (r, _) = ui.allocate_exact_size(Vec2::new(USER_W, ROW_H), egui::Sense::hover());
            let user = if proc_info.user.len() > 10 {
                format!("{:.10}", proc_info.user)
            } else {
                proc_info.user.clone()
            };
            ui.painter().text(
                r.left_center() + egui::vec2(4.0, 0.0),
                egui::Align2::LEFT_CENTER,
                user,
                font.clone(),
                base_color,
            );

            // MEMORY — right-aligned
            let (r, _) = ui.allocate_exact_size(Vec2::new(MEM_W, ROW_H), egui::Sense::hover());
            ui.painter().text(
                r.right_center() - egui::vec2(4.0, 0.0),
                egui::Align2::RIGHT_CENTER,
                format_bytes(proc_info.memory),
                font.clone(),
                base_color,
            );

            // CPU% — right-aligned, color-coded
            let (r, _) = ui.allocate_exact_size(Vec2::new(CPU_W, ROW_H), egui::Sense::hover());
            ui.painter().text(
                r.right_center() - egui::vec2(4.0, 0.0),
                egui::Align2::RIGHT_CENTER,
                format!("{:.1}%", proc_info.cpu_usage),
                font.clone(),
                cpu_color(proc_info.cpu_usage),
            );

            // GPU — always 0.0%, right-aligned
            let (r, _) = ui.allocate_exact_size(Vec2::new(GPU_W, ROW_H), egui::Sense::hover());
            ui.painter().text(
                r.right_center() - egui::vec2(4.0, 0.0),
                egui::Align2::RIGHT_CENTER,
                "0.0%",
                font.clone(),
                base_color.linear_multiply(0.6),
            );
        });

        // Entire row is clickable to expand/collapse children
        let row_rect = row_resp.response.rect;
        let click_resp = ui.allocate_rect(row_rect, egui::Sense::click());
        if click_resp.clicked() {
            toggle_pid = Some(proc_info.pid);
        }
        if click_resp.hovered() {
            ui.painter().rect_filled(
                row_rect,
                egui::Rounding::ZERO,
                egui::Color32::from_white_alpha(8),
            );
        }

        // Render children if expanded
        if expanded {
            let children = pv.get_children(proc_info.pid, system, users);
            if !children.is_empty() {
                show_children(ui, &children);
            }
        }
    }

    if let Some(pid) = toggle_pid {
        pv.toggle_expanded(pid);
    }
}

/// Render child processes with LCARS bracket accent
fn show_children(ui: &mut Ui, children: &[crate::system::process::ProcessInfo]) {
    let child_font = egui::FontId::monospace(22.0);
    let row_h = theme::CHILD_ROW_H;
    let dim = theme::CHILD_DIM;
    let accent_color = theme::LAVENDER;

    let mid = (children.len() + 1) / 2;
    let left = &children[..mid];
    let right = &children[mid..];
    let col_rows = left.len().max(right.len());

    let bracket_w: f32 = 6.0;
    let bracket_pad: f32 = 8.0;
    let col_w = CHILD_NAME_W + CHILD_PID_W + CHILD_USER_W + CHILD_MEM_W + CHILD_CPU_W;
    let col_gap: f32 = 16.0;
    let total_h = col_rows as f32 * row_h;

    let (block_rect, _) = ui.allocate_exact_size(
        Vec2::new(bracket_pad + bracket_w + bracket_pad + col_w + col_gap + col_w, total_h),
        egui::Sense::hover(),
    );

    if !ui.is_rect_visible(block_rect) {
        return;
    }

    let painter = ui.painter();

    let bracket_x = block_rect.left() + bracket_pad;
    let bracket_rect = egui::Rect::from_min_size(
        egui::pos2(bracket_x, block_rect.top() + 2.0),
        Vec2::new(bracket_w, total_h - 4.0),
    );
    painter.rect_filled(bracket_rect, egui::Rounding::same(2.0), accent_color.linear_multiply(0.5));

    let content_left = bracket_x + bracket_w + bracket_pad;

    for (col_idx, col_data) in [left, right].iter().enumerate() {
        let col_x = content_left + col_idx as f32 * (col_w + col_gap);

        for (row_idx, child) in col_data.iter().enumerate() {
            let y = block_rect.top() + row_idx as f32 * row_h;
            let cy = y + row_h / 2.0;
            let color = theme::PEACH.linear_multiply(dim);

            // Name
            let name = if child.name.len() > 12 { format!("{:.12}", child.name) } else { child.name.clone() };
            painter.text(egui::pos2(col_x + 4.0, cy), egui::Align2::LEFT_CENTER, name, child_font.clone(), color);

            // PID
            painter.text(egui::pos2(col_x + CHILD_NAME_W + 4.0, cy), egui::Align2::LEFT_CENTER,
                format!("{}", child.pid), child_font.clone(), color);

            // User
            let user = if child.user.len() > 8 { format!("{:.8}", child.user) } else { child.user.clone() };
            painter.text(egui::pos2(col_x + CHILD_NAME_W + CHILD_PID_W + 4.0, cy),
                egui::Align2::LEFT_CENTER, user, child_font.clone(), color);

            // Memory
            painter.text(
                egui::pos2(col_x + CHILD_NAME_W + CHILD_PID_W + CHILD_USER_W + CHILD_MEM_W - 4.0, cy),
                egui::Align2::RIGHT_CENTER, format_bytes(child.memory), child_font.clone(), color);

            // CPU%
            painter.text(
                egui::pos2(col_x + CHILD_NAME_W + CHILD_PID_W + CHILD_USER_W + CHILD_MEM_W + CHILD_CPU_W - 4.0, cy),
                egui::Align2::RIGHT_CENTER, format!("{:.1}%", child.cpu_usage), child_font.clone(),
                cpu_color(child.cpu_usage));
        }
    }
}
