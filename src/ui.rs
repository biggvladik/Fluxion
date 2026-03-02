use crate::app::MyApp;
use egui::{Align, Layout, Vec2};
use egui_plot::{Plot, Line, PlotPoints};

pub fn draw_ui(app: &mut MyApp, ctx: &egui::Context) {

    // =========================
    // ЛЕВАЯ ПАНЕЛЬ
    // =========================
    egui::SidePanel::left("left_panel")
        .min_width(200.0)
        .show(ctx, |ui| {

            ui.with_layout(Layout::top_down(Align::Center), |ui| {

                ui.add_space(10.0);
                ui.heading("COM Port");
                ui.add_space(15.0);

                // ===== БЛОК ПО ЦЕНТРУ =====
                ui.vertical_centered(|ui| {

                    // ComboBox + Refresh
                    ui.horizontal(|ui| {

                        egui::ComboBox::from_id_source("ports_box")
                            .width(70.0)
                            .selected_text(
                                app.serial.selected_port
                                    .clone()
                                    .unwrap_or("COM1".to_string()),
                            )
                            .show_ui(ui, |ui| {
                                for port in &app.serial.ports {
                                    ui.selectable_value(
                                        &mut app.serial.selected_port,
                                        Some(port.port_name.clone()),
                                        &port.port_name,
                                    );
                                }
                            });

                        if ui
                            .add_sized(Vec2::new(70.0, 25.0), egui::Button::new("Refresh"))
                            .clicked()
                        {
                            app.serial.refresh_ports();
                        }
                    });

                    ui.add_space(5.0);

                    // Open + Close
                    ui.horizontal(|ui| {

                        let small = Vec2::new(70.0, 25.0);

                        if ui.add_sized(small, egui::Button::new("Open")).clicked() {
                            app.serial.open();
                        }

                        if ui.add_sized(small, egui::Button::new("Close")).clicked() {
                            app.serial.close();
                        }
                    });
                });

                ui.add_space(30.0);

                // ===== Остальные кнопки =====
                let medium = Vec2::new(140.0, 35.0);

                if ui.add_sized(medium, egui::Button::new("Single")).clicked() {
                    app.serial.single();
                }

                ui.add_space(20.0);

                let live_size = Vec2::new(120.0, 120.0);

                if ui.add_sized(live_size, egui::Button::new("LIVE")).clicked() {
                    app.serial.start_live();
                }

                ui.add_space(20.0);

                if ui.add_sized(medium, egui::Button::new("Stop")).clicked() {
                    app.serial.stop_live();
                }
            });
        });

    // =========================
    // ПРАВАЯ ЧАСТЬ
    // =========================
    egui::CentralPanel::default().show(ctx, |ui| {

        let line = Line::new(PlotPoints::from(app.plot.points.clone()));

        Plot::new("plot")
            .height(ui.available_height() - 45.0)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("T отгона:");

            let mut text = format!("{:.2}", app.plot.t_otgona);

            ui.add_sized(
                Vec2::new(55.0, 20.0),
                egui::TextEdit::singleline(&mut text)
            );
        });
    });
}