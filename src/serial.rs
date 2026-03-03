use serialport::{SerialPort, SerialPortInfo};
use std::time::Duration;

pub struct SerialManager {
    pub ports: Vec<SerialPortInfo>,
    pub selected_port: Option<String>,
    port: Option<Box<dyn SerialPort>>,
}

impl SerialManager {
    pub fn new() -> Self {
        Self {
            ports: serialport::available_ports().unwrap_or_default(),
            selected_port: None,
            port: None,
        }
    }

    /// Обнаружение доступных COM портов
    pub fn refresh_ports(&mut self) -> Result<(), serialport::Error> {
        self.ports = serialport::available_ports()?;
        Ok(())
    }

    /// Открытие выбранного COM порта
    pub fn open(&mut self, baud_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(port_name) = &self.selected_port {
            let port = serialport::new(port_name, baud_rate)
                .timeout(Duration::from_millis(1000))
                .open()?;

            self.port = Some(port);
            println!("Port {} opened", port_name);
            Ok(())
        } else {
            Err("No port selected".into())
        }
    }

    /// Закрытие COM порта
    pub fn close(&mut self) {
        if self.port.is_some() {
            self.port = None; // Drop автоматически закроет порт
            println!("Port closed");
        } else {
            println!("No port is currently open");
        }
    }

    /// Проверка — открыт ли порт
    pub fn is_open(&self) -> bool {
        self.port.is_some()
    }
}