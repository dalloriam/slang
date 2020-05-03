use crate::{Instruction, INSTRUCTION_LENGTH_BYTES};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}
