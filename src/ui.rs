use crate::app::MyApp;
use egui::{Align, Frame, Layout, Rounding, Vec2, Rect};
use egui_plot::{Plot, Line, PlotPoints};

pub enum UiEvent {
    Open,
    Close,
    Refresh,
    Single,
    Live,
    Stop,
}

pub fn draw_ui(app: &mut MyApp, ctx: &egui::Context) -> Vec<UiEvent> {
    let mut events = Vec::new();

    egui::SidePanel::left("left_panel")
        .min_width(220.0)
        .show(ctx, |ui| {

            let total_height = ui.available_height();
            let top_height = 140.0;
            let middle_height = total_height - top_height - 20.0;

            // --------------------
            // Верхний блок: COM порт в рамке, рамка по центру
            // --------------------
            let frame_width = 180.0;
            let frame_height = 120.0;
            let panel_width = ui.available_width();

            let frame_x = (panel_width - frame_width) / 2.0;

            ui.allocate_ui_at_rect(
                Rect::from_min_size(
                    ui.min_rect().min + Vec2::new(frame_x, 10.0),
                    Vec2::new(frame_width, frame_height),
                ),
                |ui| {
                    let com_frame = Frame::none()
                        .fill(egui::Color32::from_gray(50))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(10.0));

                    com_frame.show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("COM Port");
                            ui.add_space(8.0);

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

                                ui.add_space(5.0);

                                if ui
                                    .add_sized(Vec2::new(70.0, 25.0), egui::Button::new("Refresh"))
                                    .clicked()
                                {
                                    events.push(UiEvent::Refresh);
                                }
                            });

                            ui.add_space(5.0);

                            // Open + Close
                            ui.horizontal(|ui| {
                                let small = Vec2::new(70.0, 25.0);

                                if ui.add_sized(small, egui::Button::new("Open")).clicked() {
                                    events.push(UiEvent::Open);
                                }

                                ui.add_space(5.0);

                                if ui.add_sized(small, egui::Button::new("Close")).clicked() {
                                    events.push(UiEvent::Close);
                                }
                            });
                        });
                    });
                },
            );

            ui.add_space(20.0);

            // --------------------
            // Средняя зона: кнопки центрированы
            // --------------------
            ui.allocate_ui_at_rect(
                Rect::from_min_size(
                    ui.min_rect().min + Vec2::new(0.0, top_height),
                    Vec2::new(ui.available_width(), middle_height),
                ),
                |ui| {
                    ui.vertical_centered(|ui| {
                        let medium = Vec2::new(140.0, 35.0);

                        if ui.add_sized(medium, egui::Button::new("Single")).clicked() {
                            events.push(UiEvent::Single);
                        }

                        ui.add_space(20.0);

                        let live_size = Vec2::new(120.0, 120.0);
                        if ui.add_sized(live_size, egui::Button::new("LIVE")).clicked() {
                            events.push(UiEvent::Live);
                        }

                        ui.add_space(20.0);

                        if ui.add_sized(medium, egui::Button::new("Stop")).clicked() {
                            events.push(UiEvent::Stop);
                        }
                    });
                }
            );
        });

    // --------------------
    // Правая панель: график
    // --------------------
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
            ui.add_sized(Vec2::new(55.0, 20.0),
                         egui::TextEdit::singleline(&mut text));
        });
    });

    events
}