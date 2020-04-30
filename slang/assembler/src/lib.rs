use std::fmt;

mod assembler;
mod common;
mod directive_parser;
mod instruction_parser;
mod label_parser;
mod opcode_parser;
mod operand_parser;
mod program_parser;
mod symbol;

pub use assembler::Assembler;
pub use instructor::Program;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

fn parse_program(src: &str) -> Result<Program, ParseError> {
    let (rest, program) = program_parser::program(src).map_err(|_e| ParseError {
        message: String::from("Parse Error"),
    })?;

    if rest != "" {
        Err(ParseError {
            message: String::from("Incomplete parse"),
        })
    } else {
        Ok(program)
    }
}
