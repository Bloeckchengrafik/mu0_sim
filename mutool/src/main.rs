mod cmd;
pub mod connection;
pub mod memory;
use clap::Parser;
use cmd::Args;

fn main() {
    let args = Args::parse();
    args.command.exec();
}
