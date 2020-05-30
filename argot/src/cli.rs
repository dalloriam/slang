use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use clap::Clap;

use argot::Compiler;

const DEFAULT_OUTPUT_NAME: &str = "a.out";

#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "William Dussault")]
pub struct CLIRoot {
    file: PathBuf,

    #[clap(long = "asm")]
    asm: bool,

    output: Option<PathBuf>,
}

impl CLIRoot {
    pub fn run(&self) -> Result<()> {
        let prg_src = fs::read_to_string(&self.file)?;

        if self.asm {
            let asm_src = Compiler::new().compile_asm(&prg_src)?;
            println!("{}", asm_src);
        } else {
            let compiled = Compiler::new().compile(&prg_src)?;

            let path = match self.output.as_ref() {
                Some(p) => p.clone(),
                None => PathBuf::from(DEFAULT_OUTPUT_NAME),
            };

            fs::write(path, compiled)?;
        }

        Ok(())
    }
}
