use std::fmt;

use instructor::{Program, ELIS_HEADER_LENGTH, ELIS_HEADER_PREFIX};

use crate::program_parser;
use crate::symbol::{Symbol, SymbolTable, SymbolType};

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
            message: format!("Incomplete parse: {}", rest),
        })
    } else {
        Ok(program)
    }
}

#[derive(Debug)]
pub struct Assembler {
    symbols: SymbolTable,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            symbols: SymbolTable::new(),
        }
    }

    fn extract_labels(&mut self, program: &Program) {
        let mut current_offset = 0;

        for instruction in program.instructions.iter() {
            if let Some(name) = instruction.label_name() {
                let symbol = Symbol::new(String::from(name), SymbolType::Label, current_offset);
                self.symbols.add(symbol);
            }
            current_offset += 4; // TODO: Extract 4 as an INSTRUCTION_WIDTH constant.
        }
    }

    fn phase_one(&mut self, program: &Program) {
        self.extract_labels(program)
    }

    fn phase_two(&mut self, program: &Program) -> Vec<u8> {
        // TODO: Pre-allocate for program size.
        let mut compiled_prg = Vec::new();
        compiled_prg.extend_from_slice(&ELIS_HEADER_PREFIX);
        for _i in 4..ELIS_HEADER_LENGTH {
            compiled_prg.push(0 as u8);
        }

        for instruction in program.instructions.iter() {
            instruction.write_bytes(&mut compiled_prg, &self.symbols);
        }

        compiled_prg
    }

    pub fn assemble(&mut self, raw: &str) -> Vec<u8> {
        match parse_program(raw) {
            Ok(prog) => {
                // Actual assembly steps.
                self.phase_one(&prog);
                return self.phase_two(&prog);
            }
            Err(e) => panic!(e.to_string()),
        }
    }
}
