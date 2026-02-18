use egui::{Color32, Pos2, Rect, Rounding, Shape, Ui, epaint::PathShape};

pub enum ElbowCorner {
    TopLeft,
    BottomLeft,
}

pub struct LcarsElbow {
    color: Color32,
    corner: ElbowCorner,
}

impl LcarsElbow {
    pub fn new(color: Color32, corner: ElbowCorner) -> Self {
        Self { color, corner }
    }

    /// Draw the elbow within the given rect.
    /// `sidebar_w` is the width of the vertical arm, `bar_h` is the height of the horizontal arm.
    pub fn draw(&self, ui: &Ui, rect: Rect, sidebar_w: f32, bar_h: f32, radius: f32) {
        let painter = ui.painter();

        match self.corner {
            ElbowCorner::TopLeft => {
                // Corner block: where sidebar meets header (top-left of rect)
                let corner_block = Rect::from_min_size(
                    rect.min,
                    egui::vec2(sidebar_w, bar_h),
                );
                painter.rect_filled(
                    corner_block,
                    Rounding {
                        nw: sidebar_w / 2.0,
                        ne: 0.0,
                        sw: 0.0,
                        se: 0.0,
                    },
                    self.color,
                );

                // Vertical arm: extends below the header
                let vert_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x, rect.min.y + bar_h),
                    egui::vec2(sidebar_w, rect.height() - bar_h),
                );
                painter.rect_filled(vert_rect, Rounding::ZERO, self.color);

                // Horizontal arm: extends right of sidebar, at same y as header
                let horiz_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x + sidebar_w, rect.min.y),
                    egui::vec2(rect.width() - sidebar_w, bar_h),
                );
                painter.rect_filled(horiz_rect, Rounding::ZERO, self.color);

                // Quarter-circle arc fill at inner corner
                let inner_corner = Pos2::new(rect.min.x + sidebar_w, rect.min.y + bar_h);
                self.draw_arc_fill(ui, inner_corner, radius, ArcQuadrant::BottomRight);
            }
            ElbowCorner::BottomLeft => {
                // Corner block: where sidebar meets footer (bottom-left of rect)
                let corner_block = Rect::from_min_size(
                    Pos2::new(rect.min.x, rect.max.y - bar_h),
                    egui::vec2(sidebar_w, bar_h),
                );
                painter.rect_filled(
                    corner_block,
                    Rounding {
                        nw: 0.0,
                        ne: 0.0,
                        sw: sidebar_w / 2.0,
                        se: 0.0,
                    },
                    self.color,
                );

                // Vertical arm: extends above the footer
                let vert_rect = Rect::from_min_size(
                    rect.min,
                    egui::vec2(sidebar_w, rect.height() - bar_h),
                );
                painter.rect_filled(vert_rect, Rounding::ZERO, self.color);

                // Horizontal arm: extends right of sidebar, at same y as footer
                let horiz_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x + sidebar_w, rect.max.y - bar_h),
                    egui::vec2(rect.width() - sidebar_w, bar_h),
                );
                painter.rect_filled(horiz_rect, Rounding::ZERO, self.color);

                // Quarter-circle arc fill at inner corner
                let inner_corner = Pos2::new(rect.min.x + sidebar_w, rect.max.y - bar_h);
                self.draw_arc_fill(ui, inner_corner, radius, ArcQuadrant::TopRight);
            }
        }
    }

    /// Fill a quarter-disc pie slice centered at `center` with the given radius.
    fn draw_arc_fill(&self, ui: &Ui, center: Pos2, radius: f32, quadrant: ArcQuadrant) {
        let painter = ui.painter();
        let steps = 20;
        let mut points = Vec::with_capacity(steps + 2);

        points.push(center);

        let (start_angle, end_angle) = match quadrant {
            // Bottom-right: arc from right (0) to down (PI/2)
            ArcQuadrant::BottomRight => (0.0_f32, std::f32::consts::FRAC_PI_2),
            // Top-right: arc from up (-PI/2) to right (0)
            ArcQuadrant::TopRight => (-std::f32::consts::FRAC_PI_2, 0.0_f32),
        };

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let angle = start_angle + t * (end_angle - start_angle);
            points.push(Pos2::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            ));
        }

        let shape = Shape::Path(PathShape::convex_polygon(
            points,
            self.color,
            egui::Stroke::NONE,
        ));
        painter.add(shape);
    }
}

enum ArcQuadrant {
    BottomRight,
    TopRight,
}
