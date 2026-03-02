use crossbeam_channel::{unbounded, Receiver};
use crate::serial::SerialManager;
use crate::plot::PlotData;

pub struct MyApp {
    pub serial: SerialManager,
    pub plot: PlotData,
    pub receiver: Receiver<f64>,
}

impl MyApp {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();

        Self {
            serial: SerialManager::new(tx),
            plot: PlotData::new(),
            receiver: rx,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Получаем данные из потока
        while let Ok(value) = self.receiver.try_recv() {
            self.plot.add_point(value);
        }

        crate::ui::draw_ui(self, ctx);

        if self.serial.live_mode {
            ctx.request_repaint();
        }
    }
}