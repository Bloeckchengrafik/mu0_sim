use std::io::Read;

use crate::connection::Device;

pub fn flash(path: &str, port: &str) {
    let mut device = Device::connect(port);
    if !device.ping() {
        println!("Connection failed");
    }
    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = vec![0u8; 64];
    file.read(&mut buffer).unwrap();

    if device.write(&buffer) {
        println!("\nFlash successful");
    } else {
        println!("\nFlash failed");
    }
}
