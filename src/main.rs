mod app;
mod system;
mod theme;
mod views;
mod widgets;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([1024.0, 600.0])
            .with_title("LCARS-OPS"),
        ..Default::default()
    };

    eframe::run_native(
        "LCARS-OPS",
        options,
        Box::new(|cc| Ok(Box::new(app::LcarsApp::new(cc)))),
    )
}
