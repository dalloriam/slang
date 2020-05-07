use std::fs;
use std::path::Path;

use anyhow::Result;

use assembler::Assembler;

pub fn load_program<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let raw_source = fs::read_to_string(path.as_ref())?;
    let compiled_program = Assembler::new().assemble(&raw_source);
    Ok(compiled_program)
}
