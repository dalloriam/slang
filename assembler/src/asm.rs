use std::io;

use byteorder::{LittleEndian, WriteBytesExt};

use instructor::{Instruction, Operand, Program, ELIS_HEADER_LENGTH, ELIS_HEADER_PREFIX};

use snafu::{ensure, ResultExt, Snafu};

use crate::program_parser;
use crate::section::Section;
use crate::symbol::{Symbol, SymbolTable, SymbolType};

#[derive(Debug, PartialEq)]
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

    #[snafu(display("Label outside of section: {}", label))]
    LabelOutsideOfSection {
        label: String,
    },

    InvalidAsciizDeclaration,

    InvalidReadonlyBlockLength {
        source: io::Error,
    },

    InvalidWordDeclaration,

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

    readonly_block: Vec<u8>,

    sections: Vec<Section>,
    symbols: SymbolTable,
}

impl Default for Assembler {
    fn default() -> Assembler {
        Assembler::new()
    }
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            current_phase: AssemblerPhase::First,
            current_section: None,
            readonly_block: Vec::new(),
            sections: Vec::new(),
            symbols: SymbolTable::new(),
        }
    }

    fn process_section_header(&mut self, header_name: &str) -> Result<()> {
        let section = Section::from(header_name);

        ensure!(
            section != Section::Unknown,
            UnknownSectionHeader {
                name: String::from(header_name)
            }
        );

        self.sections.push(section);
        self.current_section = Some(section);

        Ok(())
    }

    fn process_word_directive(&mut self, ins: &Instruction) -> Result<()> {
        if self.current_phase != AssemblerPhase::First {
            return Ok(());
        }

        match ins.operand_1.as_ref() {
            None => {
                // Word constant was empty.
                // Typed: ".word"
            }
            // TODO: Enforce that the readonly block size doesn't exceed the 16-bit addressable space.
            Some(Operand::Integer(w)) => {
                match ins.label_name() {
                    Some(label_name) => {
                        // Got a label name and a word value.
                        self.symbols
                            .update_offset(label_name, self.readonly_block.len() as u16);

                        // 4 other bytes for the length of the data block.
                        self.readonly_block.write_i32::<LittleEndian>(*w).unwrap();
                    }
                    _ => {
                        // Got no label, we can ditch the word
                        return Ok(());
                    }
                }
            }
            Some(_) => return Err(AssemblerError::InvalidWordDeclaration),
        }

        Ok(())
    }

    fn process_asciiz_directive(&mut self, ins: &Instruction) -> Result<()> {
        if self.current_phase != AssemblerPhase::First {
            return Ok(());
        }

        match ins.operand_1.as_ref() {
            None => {
                // String constant was empty.
                // Typed: ".asciiz"
            }
            Some(Operand::Str(s)) => {
                match ins.label_name() {
                    Some(label_name) => {
                        // Got a label name and a string literal.
                        // Let's insert it in the ro table.
                        self.symbols
                            .update_offset(label_name, self.readonly_block.len() as u16);

                        for byte in s.as_bytes() {
                            self.readonly_block.push(*byte)
                        }
                        self.readonly_block.push(0); // Strings are null-terminated in the ro block.
                    }
                    _ => {
                        // Got .asciiz "Hello", no label
                        return Ok(());
                    }
                }
            }
            Some(_) => {
                // Operand is not a string.
                return Err(AssemblerError::InvalidAsciizDeclaration);
            }
        }

        // For asciiz directives, the first operand is the string constant itself.
        if let Some(Operand::Str(_str_val)) = &ins.operand_1 {
            // What to do here?
            // TODO: Might be able to remove this whole block...
        } else {
            return Err(AssemblerError::InvalidAsciizDeclaration);
        }

        Ok(())
    }

    /// Phase one is the assembler pre-processing routine.
    ///
    /// It is mainly tasked with extracting labels and directives.
    fn phase_one(&mut self, program: &Program) -> Result<()> {
        self.current_phase = AssemblerPhase::First;

        let mut current_label_offset = 0;

        for instruction in program.instructions.iter() {
            if let Some(name) = instruction.label_name() {
                // We have a label
                ensure!(
                    self.current_section.is_some(),
                    LabelOutsideOfSection {
                        label: name.clone()
                    }
                );
                ensure!(
                    !self.symbols.has_symbol(name),
                    SymbolAlreadyDefined { name: name.clone() }
                );
                let symbol =
                    Symbol::new(String::from(name), SymbolType::Label, current_label_offset);
                self.symbols.add(symbol);
            }

            if let Some(name) = instruction.directive.as_ref() {
                // We have a directive.
                if instruction.has_operands() {
                    // Match which directive it is.
                    match name.as_ref() {
                        "asciiz" => {
                            // Null-terminated ascii string.
                            self.process_asciiz_directive(instruction)?;
                        }
                        "word" => {
                            self.process_word_directive(instruction)?;
                        }
                        _ => {
                            return Err(AssemblerError::UnknownDirective { name: name.clone() });
                        }
                    }
                } else {
                    // No operands => section header.
                    self.process_section_header(name)?;
                }
            }

            if let Some(op) = instruction.opcode.as_ref() {
                current_label_offset += op.width();
            }
        }
        Ok(())
    }

    fn write_header(&self, program_vector: &mut Vec<u8>) -> Result<()> {
        // 4 bytes for magic number
        program_vector.extend_from_slice(&ELIS_HEADER_PREFIX);

        // 4 other bytes for the length of the data block.
        program_vector
            .write_u32::<LittleEndian>(self.readonly_block.len() as u32)
            .context(InvalidReadonlyBlockLength)?;

        // Padding the remaining header length.
        for _i in 8..ELIS_HEADER_LENGTH {
            program_vector.push(0 as u8);
        }

        Ok(())
    }

    fn phase_two(&mut self, program: &Program) -> Result<Vec<u8>> {
        self.current_phase = AssemblerPhase::Second;
        let mut compiled_prg = Vec::with_capacity(
            ELIS_HEADER_LENGTH + self.readonly_block.len() + program.instructions.len(),
        );

        self.write_header(&mut compiled_prg)?;

        // Write the ro block to the program.
        compiled_prg.extend_from_slice(&self.readonly_block);

        // Finally - write the actual program instructions.
        for instruction in program.instructions.iter() {
            if instruction.opcode.is_some() {
                // Only write opcodes to the program.
                instruction.write_bytes(&mut compiled_prg, &self.symbols);
            }
        }

        Ok(compiled_prg)
    }

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>> {
        match parse_program(raw) {
            Ok(prog) => {
                // Actual assembly steps.
                self.phase_one(&prog)?;
                self.phase_two(&prog)
            }
            Err(e) => Err(e),
        }
    }
}
