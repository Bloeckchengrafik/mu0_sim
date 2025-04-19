use std::{
    io::{Read, Write},
    thread::sleep,
    time::Duration,
};

#[derive(Debug)]
#[allow(dead_code)]
enum Instruction {
    Lda(u16),
    Sto(u16),
    Add(u16),
    Sub(u16),
    Jmp(u16),
    Jge(u16),
    Jne(u16),
    Stp,
    Unknown,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let opcode = value >> 12;
        let arg = value & 0b111111111111;
        match opcode {
            0 => Instruction::Lda(arg),
            1 => Instruction::Sto(arg),
            2 => Instruction::Add(arg),
            3 => Instruction::Sub(arg),
            4 => Instruction::Jmp(arg),
            5 => Instruction::Jge(arg),
            6 => Instruction::Jne(arg),
            7 => Instruction::Stp,
            _ => Instruction::Unknown,
        }
    }
}

pub fn read(port: &str) {
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
    let mut buffer = vec![0u8; 64];

    port.write(b"r").unwrap();
    port.read(&mut buf).unwrap();
    sleep(Duration::from_millis(10));
    for i in 0..64 {
        port.write(&[0]).unwrap();
        sleep(Duration::from_millis(10));
        port.read(&mut buf).unwrap();
        buffer[i] = buf[0];
        print!(".");
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(10));
    }

    port.write(b"-").unwrap();
    sleep(Duration::from_millis(10));
    let mut buf = [0; 1024];
    port.read(&mut buf).unwrap();
    if buf[0] == b'E' {
        println!("\nRead successful");

        for (i, val) in buffer.chunks(2).enumerate() {
            let as_u16 = (val[0] as u16) << 8 | val[1] as u16;
            let as_char = char::from_u32(as_u16 as u32).unwrap_or('?');
            let as_charl = char::from_u32(val[0] as u32).unwrap_or('?');
            let as_charh = char::from_u32(val[1] as u32).unwrap_or('?');
            println!(
                "{:<4} {:02X}{:02X} | {:08b}{:08b} | {:<5} {}: {} {} | {:?}",
                i,
                val[0],
                val[1],
                val[0],
                val[1],
                as_u16,
                as_char,
                as_charl,
                as_charh,
                Instruction::from(as_u16)
            );
        }
        println!();
    } else {
        println!("\nRead failed");
    }
}
