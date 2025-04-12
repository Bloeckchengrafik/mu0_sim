mod cmd;
use clap::Parser;
use cmd::Args;

fn main() {
    let args = Args::parse();
    args.command.exec();
}
