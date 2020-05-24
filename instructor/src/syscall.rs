/// Defined system calls.
#[derive(Copy, Clone, Debug)]
pub enum SysCall {
    /// No-op. Default call in `$v0`.
    NOP,

    /// Constant print. Prints a string from the constant section of the read-only block.
    CPRINT,

    /// Exit. Terminates the process.
    EXIT,

    /// Allocate. Allocates the amount of memory specified in $0.
    /// Writes the start address of the allocated memory in $v0.
    ALLOC,

    /// Free. Frees memory associated to the pointer stored in $0.
    FREE,

    /// Print string. Prints a string from dynamic memory.
    PRINTS,

    /// Illegal syscall. Panics.
    IGL,
}

impl From<i32> for SysCall {
    fn from(i: i32) -> SysCall {
        match i {
            0 => SysCall::NOP,
            1 => SysCall::CPRINT,
            2 => SysCall::EXIT,
            3 => SysCall::ALLOC,
            4 => SysCall::FREE,
            5 => SysCall::PRINTS,
            _ => SysCall::IGL,
        }
    }
}
