mod cli;
mod load;
mod repl;

use clap::Clap;

fn main() {
    let root = cli::CLIRoot::parse();

    if let Err(e) = root.run() {
        eprintln!("Error: {}", e.to_string());
    }
}
