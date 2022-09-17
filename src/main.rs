mod comms;

use serialport::available_ports;

fn main() {
    println!("{:#?}", available_ports().unwrap());

    let mut channel = comms::Channel::init().unwrap();

    loop {
        let msg = b"ALPHA\n";
        _ = channel.send(msg);
    }

}
