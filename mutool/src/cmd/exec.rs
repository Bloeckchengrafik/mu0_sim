use std::{thread::sleep, time::Duration};

use crate::connection::Device;

pub fn exec(path: &str) {
    let mut device = Device::connect(path);
    if !device.ping() {
        return;
    }

    device.exec();

    while device.is_running() {
        sleep(Duration::from_millis(1000));
        println!("Still running...");
    }

    println!("Done!");
}
