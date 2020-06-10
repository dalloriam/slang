use assembler::Assembler;

use snafu::ResultExt;

use crate::{
    compiler::{error::*, first_pass::FirstPassVisitor, second_pass::SecondPassVisitor},
    syntax::program::program,
};

pub fn compile_asm(source: &str) -> Result<String> {
    let (rest, mut p) = program(source)
        .map_err(|e| ParseError {
            message: e.to_string(),
        })
        .context(IncompleteParse)?;

    assert_eq!(rest, "");

    let first_pass_output = FirstPassVisitor::new().apply(&mut p)?;
    let asm_source = SecondPassVisitor::new(first_pass_output.functions).apply(&mut p)?;

    Ok(asm_source)
}

pub fn compile(source: &str) -> Result<Vec<u8>> {
    let assembly_source = compile_asm(source)?;
    Assembler::new()
        .assemble(&assembly_source)
        .context(AssemblyFailed)
}
