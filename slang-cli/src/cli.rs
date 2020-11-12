use std::path::PathBuf;

use anyhow::Result;

use clap::Clap;

use vm::VM;

use crate::load::load_program;
use crate::repl::repl_loop;

#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "William Dussault")]
pub struct CLIRoot {
    /// Path to the .asm file to run. Starts the REPL if unspecified.
    #[clap(short = 'f', long = "file")]
    file: Option<PathBuf>,
}

impl CLIRoot {
    pub fn run(&self) -> Result<()> {
        match self.file.as_ref() {
            Some(f) => {
                // Compile & load the program, and start the VM.
                let program = load_program(&f)?;
                let mut vm = VM::new();
                vm.load_bytecode(program)?;
                vm.run();
                Ok(())
            }
            None => {
                // Start the REPL.
                repl_loop()
            }
        }
    }
}
