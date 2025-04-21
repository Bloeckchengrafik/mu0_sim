use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Instruction {
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

pub fn sanitize_char(from: char) -> char {
    if from.is_ascii() && from.is_ascii_graphic() {
        from
    } else {
        '?'
    }
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

pub struct SymbolTable {
    map: HashMap<u16, String>,
}

impl SymbolTable {
    pub fn load(bin_file: &PathBuf) -> Option<Self> {
        let sym_file = bin_file.with_extension("bin.symbols");
        let mut table = HashMap::new();

        if let Ok(content) = fs::read_to_string(sym_file) {
            for line in content.lines() {
                let mut parts = line.split_whitespace();
                if let Some(addr_str) = parts.next() {
                    if let Some(sym) = parts.next() {
                        if let Ok(addr) = addr_str.parse::<u16>() {
                            table.insert(addr, sym.to_string());
                        }
                    }
                }
            }

            Some(Self { map: table })
        } else {
            None
        }
    }

    fn get_symbol(&self, address: u16) -> Option<String> {
        self.map.get(&address).cloned()
    }
}

pub fn print_memory(buf: &[u8], symbols: Option<SymbolTable>) {
    println!();
    for (i, val) in buf.chunks(2).enumerate() {
        let as_u16 = (val[0] as u16) << 8 | val[1] as u16;
        let as_charl = sanitize_char(char::from_u32(val[0] as u32).unwrap_or('?'));
        let as_charh = sanitize_char(char::from_u32(val[1] as u32).unwrap_or('?'));
        let inst = Instruction::from(as_u16);
        let label = symbols
            .as_ref()
            .map(|symbols| symbols.get_symbol(i as u16))
            .flatten()
            .map(|label| format!("{}:", label))
            .unwrap_or(" ".into());
        println!(
            "{:<4} {:02X}{:02X} | {:08b}{:08b} | {:<5}: {} {} | {:<10} {:?}",
            i, val[0], val[1], val[0], val[1], as_u16, as_charl, as_charh, label, inst
        );
    }
}
