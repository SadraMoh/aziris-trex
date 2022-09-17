use std::time::Duration;

use serialport::*;

fn main() {
    let ports: Vec<SerialPortInfo>;
    let port: &SerialPortInfo;
    let mut serial: Box<dyn SerialPort>;

    let scan = available_ports();
    match scan {
        Ok(avail) => ports = avail,
        Err(ex) => panic!("{}", ex),
    }

    match ports.first() {
        Some(p) => port = p,
        None => panic!("Avalibe ports list is empty"),
    }

    let builder = serialport::new(port.port_name.clone(), 9600).timeout(Duration::from_millis(100));

    match builder.open() {
        Ok(p) => serial = p,
        Err(err) => {
            panic!("[BUILDER-OPEN] {:#?}", err)
        }
    }

    loop {
        let mut read_buf = [0; 32];

        let msg = b"HELLO";
        _ = serial.write(msg);
        _ = serial.flush();

        if let Ok(size) = serial.read(&mut read_buf) {
            let text = std::str::from_utf8(&read_buf[..size]);
            println!("{:#?} {size}", text);
        }
    }
}
