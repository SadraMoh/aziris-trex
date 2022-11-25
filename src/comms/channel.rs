use once_cell::sync::{Lazy};
use serialport::*;
use std::{time::Duration, sync::Mutex};

pub static COMMS: Lazy<Mutex<Channel>> = Lazy::new(|| {
    let m = Channel::init().unwrap();
    Mutex::new(m)
});

const CHANNEL_TIMEOUT: u64 = 16;
const CHANNEL_BAUDRATE: u32 = 9600;

pub struct Channel {
    pub serial: Box<dyn SerialPort>,
}

impl Channel {

    pub fn init() -> Result<Self> {
        let ports = available_ports()?;

        let port: &SerialPortInfo = ports.first().ok_or(Error {
            description: "Avalibe ports list is empty".to_string(),
            kind: ErrorKind::NoDevice,
        })?;

        let builder = serialport::new(port.port_name.clone(), CHANNEL_BAUDRATE)
            .timeout(Duration::from_millis(CHANNEL_TIMEOUT))
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One)
            ;

        let serial = builder.open()?;

        Ok(Channel { serial })
    }

    pub fn send(&mut self, msg: &[u8]) -> Result<usize> {
        let size = self.serial.write(msg)?;
        self.serial.flush()?;

        Ok(size)
    }

    pub fn read(&mut self) -> Result<[u8; 32]> {
        let mut read_buf = [0; 32];
        let _size = self.serial.read(&mut read_buf)?;
        self.serial.flush()?;

        Ok(read_buf)
    }
    
    pub fn read_str(&mut self) -> Result<String> {
        let mut read_buf = [0; 32];
        let size = self.serial.read(&mut read_buf)?;
        self.serial.flush()?;

        if let Ok(text) = std::str::from_utf8(&read_buf[..size]) {
            Ok(String::from(text))
        } else {
            Ok(String::from("!"))
        }
    }
}