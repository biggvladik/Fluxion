use crate::serial::SerialManager;
use crate::plot::PlotData;
use crate::ui::{draw_ui, UiEvent};

pub struct MyApp {
    pub serial: SerialManager,
    pub plot: PlotData,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            serial: SerialManager::new(),
            plot: PlotData::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let events = draw_ui(self, ctx);

        for event in events {
            match event {
                UiEvent::Open => self.serial.open(),
                UiEvent::Close => self.serial.close(),
                UiEvent::Refresh => self.serial.refresh_ports(),
                UiEvent::Single => {}, // можно добавить генерацию точки
                UiEvent::Live => {},   // пока нет
                UiEvent::Stop => {},   // пока нет
            }
        }
    }
}