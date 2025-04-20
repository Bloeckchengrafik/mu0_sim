use std::path::PathBuf;

use crate::{
    connection::Device,
    memory::{SymbolTable, print_memory},
};

pub fn read(port: &str, path: &Option<String>) {
    let mut device = Device::connect(port);
    let Some(buffer) = device.read() else {
        panic!("read failed");
    };

    print_memory(
        &buffer,
        path.clone()
            .map(|path| SymbolTable::load(&PathBuf::from(path)))
            .flatten(),
    );
}
