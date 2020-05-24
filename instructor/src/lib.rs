mod instruction;
mod label_converter;
mod opcode;
mod operand;
mod program;
mod syscall;

/// The magic number for ELIS executables.
pub const ELIS_HEADER_PREFIX: [u8; 4] = [69, 76, 73, 83];

/// The length of the ELIS header.
pub const ELIS_HEADER_LENGTH: usize = 64;

/// Number of VM registers, excluding special ones.
pub const REGULAR_REGISTER_COUNT: usize = 32;

/// Number of registers of the VM.
pub const REGISTER_COUNT: usize = 35;

/// Which register is used for syscalls.
pub const SYSCALL_REGISTER: usize = 32;
pub const STACK_POINTER_REGISTER: usize = 33;
pub const STACK_BASE_REGISTER: usize = 34;

pub use instruction::Instruction;
pub use label_converter::LabelConverter;
pub use opcode::Opcode;
pub use operand::{Address, MemorySection, Operand};
pub use program::Program;
pub use syscall::SysCall;
