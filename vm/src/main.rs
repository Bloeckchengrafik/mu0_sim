use std::{
    collections::HashMap,
    fs,
    io::Read,
    path::{PathBuf, PrefixComponent},
    str::FromStr,
};

fn read_file(filename: PathBuf) -> Result<Vec<u16>, std::io::Error> {
    let file = fs::File::open(filename);
    let mut reader = std::io::BufReader::new(file?);
    let mut numbers = Vec::new();
    let mut buffer = [0u8; 2];

    while let Ok(_) = reader.read_exact(&mut buffer) {
        numbers.push(u16::from_le_bytes(buffer));
    }

    Ok(numbers)
}

#[derive(Debug)]
enum Instruction {
    Lda(u16),
    Sto(u16),
    Add(u16),
    Sub(u16),
    Jmp(u16),
    Jge(u16),
    Jne(u16),
    Stp,
}

impl TryFrom<u16> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let opcode = value >> 12;
        let arg = value & 0b111111111111;
        match opcode {
            0 => Ok(Instruction::Lda(arg)),
            1 => Ok(Instruction::Sto(arg)),
            2 => Ok(Instruction::Add(arg)),
            3 => Ok(Instruction::Sub(arg)),
            4 => Ok(Instruction::Jmp(arg)),
            5 => Ok(Instruction::Jne(arg)),
            6 => Ok(Instruction::Jge(arg)),
            7 => Ok(Instruction::Stp),
            _ => Err(anyhow::anyhow!("Unknown opcode")),
        }
    }
}

fn transmute_to_signed(unsigned: u16) -> i16 {
    unsafe { std::mem::transmute(unsigned) }
}

fn transmute_to_unsigned(signed: i16) -> u16 {
    unsafe { std::mem::transmute(signed) }
}

fn load_symbol_table(bin_file: &PathBuf) -> HashMap<String, u16> {
    let sym_file = bin_file.with_extension("bin.symbols");
    let mut table = HashMap::new();

    if let Ok(content) = fs::read_to_string(sym_file) {
        for line in content.lines() {
            let mut parts = line.split_whitespace();
            if let Some(addr_str) = parts.next() {
                if let Some(sym) = parts.next() {
                    if let Ok(addr) = addr_str.parse::<u16>() {
                        table.insert(sym.to_string(), addr);
                    }
                }
            }
        }
    }

    table
}

fn query_value(memory: &Vec<u16>, symbols: &HashMap<String, u16>) -> Result<(), anyhow::Error> {
    let mut input = String::new();
    println!("Enter memory address and format:");
    println!("Formats: i (signed), u (unsigned), x (hex), b (binary), n (instruction)");
    if !symbols.is_empty() {
        println!("Loaded syms! Use s to get symbols and use a symbol instead of an adress")
    }
    println!("Example: '42 i' or '100 x'");
    println!("Enter 'q' to quit");

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if input == "q" {
            break;
        }

        if input == "s" {
            println!("Loaded symbols: ");
            for (symbol, addr) in symbols {
                println!("{}: {}", symbol, addr);
            }
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input format");
            continue;
        }

        let addr = if let Ok(direct_addr) = parts[0].parse::<usize>() {
            direct_addr
        } else if let Some(&symbol_addr) = symbols.get(parts[0]) {
            symbol_addr as usize
        } else {
            println!("Invalid address or unknown symbol");
            continue;
        };

        match parts[1] {
            "i" => println!(
                "Signed value at {}: {}",
                addr,
                transmute_to_signed(memory[addr])
            ),
            "u" => println!("Unsigned value at {}: {}", addr, memory[addr]),
            "x" => println!("Hex value at {}: 0x{:04X}", addr, memory[addr]),
            "b" => println!("Binary value at {}: {:016b}", addr, memory[addr]),
            "n" => match Instruction::try_from(memory[addr]) {
                Ok(inst) => println!("Instruction at {}: {:?}", addr, inst),
                Err(_) => println!("Invalid instruction at {}", addr),
            },
            _ => println!("Invalid format specifier (use 'i', 'u', 'x', 'b' or 'n')"),
        }
    }
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(anyhow::anyhow!("Please provide a path to a file"));
    }
    let path = PathBuf::from_str(&args[1])?;

    let syms = load_symbol_table(&path);
    let mut memory = read_file(path).unwrap();
    memory.extend([0u16; 1024]);
    let mut pc = 0usize;
    let mut acc: i16 = 0;
    loop {
        let inst: Instruction = memory[pc].try_into()?;
        println!("pc={}; {:?}", pc, inst);
        pc += 1;
        match inst {
            Instruction::Add(addr) => {
                acc = acc.wrapping_add(transmute_to_signed(memory[addr as usize]))
            }
            Instruction::Sub(addr) => {
                acc = acc.wrapping_sub(transmute_to_signed(memory[addr as usize]))
            }
            Instruction::Lda(addr) => acc = transmute_to_signed(memory[addr as usize]),
            Instruction::Sto(addr) => memory[addr as usize] = transmute_to_unsigned(acc),
            Instruction::Jmp(addr) => pc = addr as usize,
            Instruction::Jne(addr) => {
                if acc != 0 {
                    pc = addr as usize
                }
            }
            Instruction::Jge(addr) => {
                if acc >= 0 {
                    pc = addr as usize
                }
            }
            Instruction::Stp => break,
        }
    }

    query_value(&memory, &syms)
}
