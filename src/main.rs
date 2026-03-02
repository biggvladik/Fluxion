mod app;
mod ui;
mod serial;
mod plot;

use app::MyApp;

fn main() -> eframe::Result<()> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 600.0]) // размер окна
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Fluxion",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
}