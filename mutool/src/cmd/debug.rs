use std::{io::BufRead, thread::sleep, time::Duration};

use crate::connection::{ClockMode, Device};

pub fn debug(device: &str) {
    let mut device = Device::connect(device);

    device.set_clock(ClockMode::ManualOff);

    loop {
        let debug_info = device.get_debug();
        if let Some(debug_info) = debug_info {
            debug_info.print();
        } else {
            println!("No debug information available");
        }

        println!("Press 'q' to quit, enter to do a clock cycle");
        let input = std::io::stdin().lock().lines().next().unwrap().unwrap();
        match input.as_str() {
            "q" => break,
            "" => {
                device.set_clock(ClockMode::ManualOn);
                sleep(Duration::from_millis(10));
                device.set_clock(ClockMode::ManualOff);
                sleep(Duration::from_millis(10));
            }
            _ => println!("Invalid input"),
        }
    }
}
