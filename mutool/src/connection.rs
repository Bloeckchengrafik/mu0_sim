use std::{io::Write, thread::sleep, time::Duration};

use crate::memory::Instruction;

pub struct Device(Box<dyn serialport::SerialPort>);

pub enum ClockMode {
    Off,
    Fast,
    Slow,
    ManualOff,
    ManualOn,
}

impl Into<u8> for ClockMode {
    fn into(self) -> u8 {
        match self {
            ClockMode::Off => 0,
            ClockMode::Fast => 1,
            ClockMode::Slow => 2,
            ClockMode::ManualOff => 3,
            ClockMode::ManualOn => 4,
        }
    }
}

#[derive(Debug)]
pub enum SystemState {
    Fetch,
    FetchStore,
    Exec,
    Store,
    Unknown,
}

impl From<u8> for SystemState {
    fn from(value: u8) -> Self {
        match value {
            0 => SystemState::Fetch,
            1 => SystemState::FetchStore,
            2 => SystemState::Exec,
            3 => SystemState::Store,
            _ => SystemState::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum AluOp {
    Zero,
    Add,
    Sub,
    AInc,
    B,
    Unknown,
}

impl From<u8> for AluOp {
    fn from(value: u8) -> Self {
        match value {
            0 => AluOp::Zero,
            1 => AluOp::Add,
            2 => AluOp::Sub,
            3 => AluOp::AInc,
            4 => AluOp::B,
            _ => AluOp::Unknown,
        }
    }
}

pub struct DebugInfo {
    pub ir: u16,
    pub pc: u16,
    pub acc: u16,
    pub state: SystemState,
    pub alu_op: AluOp,
    pub alu_out: u16,
}

impl DebugInfo {
    pub fn print(&self) {
        let inst: Instruction = self.ir.into();
        println!("IR:  {:016b}: {:?}", self.ir, inst);
        println!("PC:  {:016b}: {}", self.pc, self.pc);
        println!("ACC: {:016b}: {}", self.acc, self.acc);
        println!("AOT: {:016b}: {}", self.alu_out, self.alu_out);
        println!("AOP: {:?}", self.alu_op);
        println!("STA: {:?}", self.state);
    }
}

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
            self.0.write(&[buffer[i * 2]]).unwrap();
            sleep(Duration::from_millis(10));
            self.0.read(&mut buf).unwrap();
            print!("{}", char::from_u32(buf[0] as u32).unwrap());
            std::io::stdout().flush().unwrap();
            sleep(Duration::from_millis(10));

            self.0.write(&[buffer[i * 2 + 1]]).unwrap();
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
            buffer[i * 2 + 1] = read_buffer[0];
            self.0.write(&[0]).unwrap();
            self.0.read_exact(&mut read_buffer).unwrap();
            buffer[i * 2] = read_buffer[0];
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
        sleep(Duration::from_millis(10));
        self.0.write(b"s").unwrap();
        let mut buf = [0; 1024];
        self.0.read(&mut buf).unwrap();
        sleep(Duration::from_millis(10));
        return buf[0] != b'-';
    }

    pub fn set_clock(&mut self, clock: ClockMode) {
        let clock_id: u8 = clock.into();
        self.0.write(&[clock_id + b'0']).unwrap();
        let mut buf = [0; 1024];
        self.0.read(&mut buf).unwrap();
    }

    pub fn transact(&mut self, s: u8) -> u8 {
        self.0.write(&[s]).unwrap();
        sleep(Duration::from_millis(10));
        let mut buf = [0; 1024];
        self.0.read(&mut buf).unwrap();
        sleep(Duration::from_millis(10));
        return buf[0];
    }

    pub fn get_debug(&mut self) -> Option<DebugInfo> {
        let lower_ir = self.transact(b'I') as u16;
        let upper_ir = self.transact(b'i') as u16;
        let ir = upper_ir << 8 | lower_ir;

        let lower_pc = self.transact(b'C') as u16;
        let upper_pc = self.transact(b'c') as u16;
        let pc = upper_pc << 8 | lower_pc;

        let lower_acc = self.transact(b'A') as u16;
        let upper_acc = self.transact(b'a') as u16;
        let acc = upper_acc << 8 | lower_acc;

        let state: SystemState = self.transact(b'S').into();

        let lower_aluout = self.transact(b'L') as u16;
        let upper_aluout = self.transact(b'l') as u16;
        let alu_out = upper_aluout << 8 | lower_aluout;

        let alu_op: AluOp = self.transact(b'o').into();

        Some(DebugInfo {
            ir,
            pc,
            acc,
            state,
            alu_out,
            alu_op,
        })
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
