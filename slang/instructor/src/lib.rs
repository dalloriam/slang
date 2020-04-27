mod instruction;
mod opcode;

pub use instruction::Instruction;
pub use opcode::Opcode;

#[cfg(test)]
mod tests {
    use super::{Instruction, Opcode};

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode(), &Opcode::HLT);
    }
}
