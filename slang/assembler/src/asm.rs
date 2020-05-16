use std::fmt;

use instructor::{Instruction, Operand, Program, ELIS_HEADER_LENGTH, ELIS_HEADER_PREFIX};

use snafu::{ensure, ResultExt, Snafu};

use crate::program_parser;
use crate::section::Section;
use crate::symbol::{Symbol, SymbolTable, SymbolType};

#[derive(PartialEq)]
enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug, Snafu)]
pub enum AssemblerError {
    #[snafu(display("{}", message))]
    ParseError {
        message: String,
    },

    LabelOutsideOfSection {},

    InvalidAsciizDeclaration,

    #[snafu(display("Symbol '{}' defined multiple times", name))]
    SymbolAlreadyDefined {
        name: String,
    },

    #[snafu(display("Unknown directive: {}", name))]
    UnknownDirective {
        name: String,
    },

    #[snafu(display("Unkown Section Header: {}", name))]
    UnknownSectionHeader {
        name: String,
    },
}

type Result<T> = std::result::Result<T, AssemblerError>;

fn parse_program(src: &str) -> Result<Program> {
    let (rest, program) =
        program_parser::program(src).map_err(|_e| AssemblerError::ParseError {
            message: String::from("Parse Error"),
        })?;

    if rest != "" {
        Err(AssemblerError::ParseError {
            message: format!("Incomplete parse: {}", rest),
        })
    } else {
        Ok(program)
    }
}

#[derive(Debug)]
pub struct Assembler {
    current_phase: AssemblerPhase,
    current_section: Option<Section>,

    sections: Vec<Section>,

    symbols: SymbolTable,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            current_phase: AssemblerPhase::First,
            current_section: None,
            sections: Vec::new(),
            symbols: SymbolTable::new(),
        }
    }

    fn extract_labels(&mut self, program: &Program) -> Result<()> {
        let mut current_offset = 0;

        for instruction in program.instructions.iter() {
            ensure!(self.current_section.is_some(), LabelOutsideOfSection);
            if let Some(name) = instruction.label_name() {
                ensure!(
                    !self.symbols.has_symbol(name),
                    SymbolAlreadyDefined { name: name.clone() }
                );
                let symbol = Symbol::new(String::from(name), SymbolType::Label, current_offset);
                self.symbols.add(symbol);
            }
            current_offset += 4; // TODO: Extract 4 as an INSTRUCTION_WIDTH constant.
        }
        Ok(())
    }

    fn process_section_header(&mut self, header_name: &str) -> Result<()> {
        let section = Section::from(header_name);

        ensure!(
            section != Section::Unknown,
            UnknownSectionHeader {
                name: header_name.clone()
            }
        );

        self.sections.push(section);
        self.current_section = Some(section);

        Ok(())
    }

    fn process_asciiz_directive(&mut self, ins: &Instruction) -> Result<()> {
        if self.current_phase != AssemblerPhase::First {
            return Ok(());
        }

        panic!("STOPPED WORKING HERE: TODO: CONTINUE HERE");

        // For asciiz directives, the first operand is the string constant itself.
        if let Some(Operand::Str(str_val)) = &ins.operand_1 {
        } else {
            return Err(AssemblerError::InvalidAsciizDeclaration);
        }

        Ok(())
    }

    fn extract_directives(&mut self, program: &Program) -> Result<()> {
        for instruction in program.instructions.iter() {
            if let Some(name) = instruction.directive.as_ref() {
                if instruction.has_operands() {
                    // Match which directive it is.
                    match name {
                        "asciiz" => {
                            // Null-terminated ascii string.
                        }
                        _ => Err(AssemblerError::UnknownDirective { name: name.clone() }),
                    }
                } else {
                    // No operands => section header.
                    self.process_section_header(name)?;
                }
            }
        }

        Ok(())
    }

    /// Phase one is the assembler pre-processing routine.
    ///
    /// It is mainly tasked with extracting labels and directives.
    fn phase_one(&mut self, program: &Program) -> Result<()> {
        self.current_phase = AssemblerPhase::First;
        self.extract_labels(program)?;
        self.extract_directives(program)
    }

    fn phase_two(&mut self, program: &Program) -> Vec<u8> {
        self.current_phase = AssemblerPhase::Second;
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

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>> {
        match parse_program(raw) {
            Ok(prog) => {
                // Actual assembly steps.
                self.phase_one(&prog)?;
                return Ok(self.phase_two(&prog));
            }
            Err(e) => Err(e),
        }
    }
}
