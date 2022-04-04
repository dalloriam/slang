mod cli;

use clap::Parser;

fn init_logger() {
    env_logger::init();
}

fn main() {
    init_logger();

    let root = cli::CLIRoot::parse();
    if let Err(e) = root.run() {
        log::error!("{}", e.to_string());
    }
}
