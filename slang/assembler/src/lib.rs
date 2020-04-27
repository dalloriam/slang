use std::fmt;

mod common;
mod instruction_parser;
mod opcode_parser;
mod operand_parser;
mod program_parser;
mod register_parser;

use instructor::Opcode;
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

pub fn parse_program(src: &str) -> Result<Program, ParseError> {
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
