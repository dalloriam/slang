use std::fs;
use std::path::Path;

use anyhow::Result;

use assembler::Assembler;

use instructor::ELIS_HEADER_PREFIX;

pub fn load_program<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let raw_prog = fs::read(path.as_ref())?;
    if raw_prog.starts_with(&ELIS_HEADER_PREFIX) {
        // Already compiled.
        Ok(raw_prog)
    } else {
        let raw_source = String::from_utf8(raw_prog)?;
        let compiled_program = Assembler::new().assemble(&raw_source)?;
        Ok(compiled_program)
    }
}
