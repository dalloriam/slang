mod instruction;
mod label_converter;
mod opcode;
mod operand;
mod program;
mod syscall;

pub const INSTRUCTION_LENGTH_BYTES: usize = 4;

pub const ELIS_HEADER_PREFIX: [u8; 4] = [69, 76, 73, 83];
pub const ELIS_HEADER_LENGTH: usize = 64;

pub use instruction::Instruction;
pub use label_converter::LabelConverter;
pub use opcode::Opcode;
pub use operand::Operand;
pub use program::Program;
pub use syscall::SysCall;

#[cfg(test)]
mod tests {
    use super::Opcode;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }
}
