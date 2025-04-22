use std::{thread::sleep, time::Duration};

use crate::connection::{ClockMode, Device};

pub fn exec(path: &str) {
    let mut device = Device::connect(path);
    if !device.ping() {
        return;
    }

    device.exec();
    device.set_clock(ClockMode::ManualOff);

    while device.is_running() {
        device.set_clock(ClockMode::ManualOn);
        sleep(Duration::from_millis(10));
        device.set_clock(ClockMode::ManualOff);
        sleep(Duration::from_millis(10));

        let status = device.get_debug();
        if let Some(debug) = status {
            debug.print();
        }
    }

    println!("Device is done!");
}
