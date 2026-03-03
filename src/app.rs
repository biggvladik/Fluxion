use crate::serial::SerialManager;
use crate::plot::PlotData;
use crate::ui::{draw_ui, UiEvent};

pub struct MyApp {
    pub serial: SerialManager,
    pub plot: PlotData,
    error_message: Option<String>,
    baud_rate: u32,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            serial: SerialManager::new(),
            plot: PlotData::new(),
            error_message: None,
            baud_rate: 9600, // скорость по умолчанию
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Получаем события от UI
        let events = draw_ui(self, ctx);

        for event in events {
            match event {

                UiEvent::Open => {
                    if let Err(e) = self.serial.open(self.baud_rate) {
                        self.error_message = Some(e.to_string());
                    }
                }

                UiEvent::Close => {
                    self.serial.close();
                }

                UiEvent::Refresh => {
                    if let Err(e) = self.serial.refresh_ports() {
                        self.error_message = Some(e.to_string());
                    }
                }

                UiEvent::Single => {
                    // TODO: отправка одной команды
                }

                UiEvent::Live => {
                    // TODO: запуск live режима
                }

                UiEvent::Stop => {
                    // TODO: остановка live режима
                }
            }
        }

        // ---- Окно ошибки ----
        if let Some(error) = &self.error_message {
            let mut clear_error = false;

            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(error);

                    ui.add_space(10.0);

                    if ui.button("OK").clicked() {
                        clear_error = true;
                    }
                });

            // Мутируем self только после closure
            if clear_error {
                self.error_message = None;
            }
        }
    }
}