mod instruction;
mod opcode;
mod operand;
mod program;

pub const INSTRUCTION_LENGTH_BYTES: usize = 4;

pub use instruction::Instruction;
pub use opcode::Opcode;
pub use operand::Operand;
pub use program::Program;

#[cfg(test)]
mod tests {
    use super::{Instruction, Opcode};

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }
}
