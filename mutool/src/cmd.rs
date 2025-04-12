mod list_ports;

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
}

impl Commands {
    pub fn exec(&self) {
        match self {
            Commands::ListPorts => list_ports::list_ports(),
        }
    }
}
