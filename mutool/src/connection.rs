use std::{io::Write, thread::sleep, time::Duration};

pub struct Device(Box<dyn serialport::SerialPort>);

impl Device {
    pub fn connect(port: &str) -> Self {
        let port = serialport::new(port, 115200)
            .timeout(Duration::from_millis(1000))
            .open()
            .unwrap();

        let mut dev = Self(port);
        if dev.ping() {
            println!("Connection established");
        } else {
            println!("Connection failed");
        }

        return dev;
    }

    pub fn ping(&mut self) -> bool {
        self.0.write(b"p").unwrap();
        sleep(Duration::from_millis(10));
        let mut buf = [0; 1024];
        self.0.read(&mut buf).unwrap();
        return buf[0] == b'P';
    }

    pub fn write(&mut self, buffer: &[u8]) -> bool {
        let mut buf = [0; 1024];
        self.0.write(b"w").unwrap();
        self.0.read(&mut buf).unwrap();
        sleep(Duration::from_millis(10));
        for i in 0..32 {
            self.0.write(&[buffer[i * 2 + 1]]).unwrap();
            sleep(Duration::from_millis(10));
            self.0.read(&mut buf).unwrap();
            print!("{}", char::from_u32(buf[0] as u32).unwrap());
            std::io::stdout().flush().unwrap();
            sleep(Duration::from_millis(10));

            self.0.write(&[buffer[i * 2]]).unwrap();
            sleep(Duration::from_millis(10));
            self.0.read(&mut buf).unwrap();
            print!("{}", char::from_u32(buf[0] as u32).unwrap());
            std::io::stdout().flush().unwrap();
            sleep(Duration::from_millis(10));
        }

        self.0.write(b"-").unwrap();
        sleep(Duration::from_millis(10));
        let mut buf = [0; 1024];
        self.0.read(&mut buf).unwrap();
        return buf[0] == b'E';
    }

    pub fn read(&mut self) -> Option<Vec<u8>> {
        let mut buffer = vec![0u8; 64];
        let mut read_buffer = vec![0u8; 1];
        let mut buf = [0; 1024];

        self.0.write(b"r").unwrap();
        self.0.read(&mut buf).unwrap();
        sleep(Duration::from_millis(10));
        for i in 0..32 {
            self.0.write(&[0]).unwrap();
            self.0.read_exact(&mut read_buffer).unwrap();
            self.0.write(&[0]).unwrap();
            self.0.read_exact(&mut read_buffer).unwrap();
            buffer[i * 2] = read_buffer[0];
            self.0.write(&[0]).unwrap();
            self.0.read_exact(&mut read_buffer).unwrap();
            buffer[i * 2 + 1] = read_buffer[0];
            print!("-+");
            std::io::stdout().flush().unwrap();
            sleep(Duration::from_millis(10));
        }

        self.0.write(b"-").unwrap();
        sleep(Duration::from_millis(10));
        let mut buf = [0; 1024];
        self.0.read(&mut buf).unwrap();
        if buf[0] == b'E' {
            return Some(buffer);
        } else {
            return None;
        }
    }

    pub fn exec(&mut self) {
        self.0.write(b"x").unwrap();
        let mut buf = [0; 1024];
        self.0.read(&mut buf).unwrap();
    }

    pub fn is_running(&mut self) -> bool {
        self.0.write(b"s").unwrap();
        let mut buf = [0; 1024];
        self.0.read(&mut buf).unwrap();
        return buf[0] == '+' as u8;
    }
}

pub trait ToU16Vec {
    fn to_u16_vec(&self) -> Vec<u16>;
}

impl ToU16Vec for Vec<u8> {
    fn to_u16_vec(&self) -> Vec<u16> {
        return self
            .chunks(2)
            .map(|s| (s[0] as u16) << 8 | s[1] as u16)
            .collect();
    }
}
