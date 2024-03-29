mod cli;
mod load;
mod repl;

use clap::Parser;

fn init_logger() {
    env_logger::init();
}

fn main() {
    init_logger();

    let root = cli::CLIRoot::parse();

    if let Err(e) = root.run() {
        eprintln!("Error: {}", e);
    }
}
