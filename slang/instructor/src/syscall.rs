/// Defined system calls.
#[derive(Copy, Clone, Debug)]
pub enum SysCall {
    /// No-op. Default call in `$v0`.
    NOP,

    /// Constant print. Prints a string from the constant section of the read-only block.
    CPRINT,

    /// Exit. Terminates the process.
    EXIT,

    /// Illegal syscall. Panics.
    IGL,
}

impl From<i32> for SysCall {
    fn from(i: i32) -> SysCall {
        match i {
            0 => SysCall::NOP,
            1 => SysCall::CPRINT,
            2 => SysCall::EXIT,
            _ => SysCall::IGL,
        }
    }
}
