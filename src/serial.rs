use serialport::SerialPortInfo;

pub struct SerialManager {
    pub ports: Vec<SerialPortInfo>,
    pub selected_port: Option<String>,
}

impl SerialManager {
    pub fn new() -> Self {
        Self {
            ports: serialport::available_ports().unwrap_or_default(),
            selected_port: None,
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
}