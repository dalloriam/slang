use instructor::Program;

use crate::symbol::SymbolType;
use crate::{
    parse_program,
    symbol::{Symbol, SymbolTable},
};

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

        Vec::new()
    }
}
