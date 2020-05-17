use crate::Instruction;

#[derive(Debug, PartialEq)]
/// Represents a compiled program.
pub struct Program {
    /// The vector of instructions composing the program.
    pub instructions: Vec<Instruction>,
}
