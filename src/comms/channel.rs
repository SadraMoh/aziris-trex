use druid::image::EncodableLayout;
use once_cell::sync::Lazy;
use serialport::*;
use std::{sync::Mutex, thread, time::Duration};

// initialized COMMS
pub static COMMS: Lazy<Mutex<Channel>> = Lazy::new(|| {
    let channel = Channel::init().unwrap();
    Mutex::new(channel)
});

const CHANNEL_TIMEOUT: u64 = 20;
const CHANNEL_BAUDRATE: u32 = 9600;

const CHANNEL_INPUT_INTERVAL: u64 = 20;

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
            .flow_control(FlowControl::Software)
            .timeout(Duration::from_millis(CHANNEL_TIMEOUT))
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One);

        let serial = builder.open()?;

        let channel = Channel { serial };

        Ok(channel)
    }

    pub fn cmd(&mut self, msg: &[u8]) -> Result<usize> {

        let mut command = msg.to_owned();
        command.push('\0' as u8);
        
        let size = self.serial.write(command.as_bytes())?;
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

    // impl Fn(&mut EventCtx, &mut T, &Env) + 'static
    pub fn listen(callback: impl Fn(String) + std::marker::Sync + std::marker::Send + 'static) {
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(CHANNEL_INPUT_INTERVAL));

            let mut comms = COMMS.lock().unwrap();

            let to_read = comms.serial.bytes_to_read().unwrap();
            if to_read == 0 { continue; }

            // read_str
            let mut read_buf = [0; 32];
            let size = comms.serial.read(&mut read_buf).unwrap();
            comms.serial.flush().unwrap();

            let text = std::str::from_utf8(&read_buf[..size]).unwrap();
                        
            // println!("to read: {}  text: {}", to_read, text);
            callback(text.to_string());
        });
    }
}
