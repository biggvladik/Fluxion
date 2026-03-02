use serialport::SerialPortInfo;
use crossbeam_channel::Sender;
use std::thread;
use std::time::Duration;

pub struct SerialManager {
    pub ports: Vec<SerialPortInfo>,
    pub selected_port: Option<String>,
    pub live_mode: bool,
    sender: Sender<f64>,
}

impl SerialManager {
    pub fn new(sender: Sender<f64>) -> Self {
        Self {
            ports: serialport::available_ports().unwrap_or_default(),
            selected_port: None,
            live_mode: false,
            sender,
        }
    }

    pub fn refresh_ports(&mut self) {
        self.ports = serialport::available_ports().unwrap_or_default();
    }

    pub fn open(&mut self) {
        println!("Port opened");
    }

    pub fn close(&mut self) {
        println!("Port closed");
    }

    pub fn single(&self) {
        let value = rand::random::<f64>() * 100.0;
        let _ = self.sender.send(value);
    }

    pub fn start_live(&mut self) {
        self.live_mode = true;
        let sender = self.sender.clone();

        thread::spawn(move || {
            loop {
                let value = rand::random::<f64>() * 100.0;
                let _ = sender.send(value);
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    pub fn stop_live(&mut self) {
        self.live_mode = false;
    }
}