#[derive(Copy, Clone, Debug)]
pub enum SysCall {
    NOP,
    PRINT,
    EXIT,
    IGL,
}

impl From<i32> for SysCall {
    fn from(i: i32) -> SysCall {
        match i {
            0 => SysCall::NOP,
            1 => SysCall::PRINT,
            2 => SysCall::EXIT,
            _ => SysCall::IGL,
        }
    }
}
