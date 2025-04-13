mod flash;
mod list_ports;
mod read;

use clap::{Parser, Subcommand, command};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List all available ports that belong to a Tang Nano 9k
    ListPorts,
    /// Flash a program
    Flash {
        /// Path to the firmware image
        path: String,
        /// Device to flash
        device: String,
    },
    /// Read the memory of the mu0
    Read {
        /// Device to flash
        device: String,
    },
}

impl Commands {
    pub fn exec(&self) {
        match self {
            Commands::ListPorts => list_ports::list_ports(),
            Commands::Flash { path, device } => flash::flash(path, device),
            Commands::Read { device } => read::read(device),
        }
    }
}
