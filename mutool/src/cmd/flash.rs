use std::{
    io::{Read, Write},
    thread::sleep,
    time::Duration,
};

pub fn flash(path: &str, port: &str) {
    let mut port = serialport::new(port, 115200)
        .timeout(Duration::from_millis(1000))
        .open()
        .unwrap();
    port.write(b"p").unwrap();
    sleep(Duration::from_millis(10));
    let mut buf = [0; 1024];
    port.read(&mut buf).unwrap();
    if buf[0] == b'P' {
        println!("Connection established");
    } else {
        println!("Connection failed");
    }
    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = vec![0u8; 64];
    let bytes_read = file.read(&mut buffer).unwrap();
    if bytes_read < 64 {
        buffer.resize(64, 0);
    }

    port.write(b"w").unwrap();
    port.read(&mut buf).unwrap();
    sleep(Duration::from_millis(10));
    for i in 0..64 {
        port.write(&[buffer[i]]).unwrap();
        sleep(Duration::from_millis(50));
        port.read(&mut buf).unwrap();
        print!("{}", char::from_u32(buf[0] as u32).unwrap());
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(50));
    }

    port.write(b"-").unwrap();
    sleep(Duration::from_millis(10));
    let mut buf = [0; 1024];
    port.read(&mut buf).unwrap();
    if buf[0] == b'E' {
        println!("\nFlash successful");
    } else {
        println!("\nFlash failed");
    }
}
