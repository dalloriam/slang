#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    // Short label jump.
    JMP,

    // Short relative jump forwards.
    JMPF,

    // Short relative jump backwards
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
    ALOC,
    INC,
    DEC,
    // Long absolute jump
    RJMP,
    SYSC,
    IGL,
}

impl<'a> From<&'a str> for Opcode {
    fn from(v: &'a str) -> Self {
        match v {
            "hlt" => Opcode::HLT,
            "ld" => Opcode::LOAD,
            "add" => Opcode::ADD,
            "sub" => Opcode::SUB,
            "mul" => Opcode::MUL,
            "div" => Opcode::DIV,
            "jmp" => Opcode::JMP,
            "jmpf" => Opcode::JMPF,
            "jmpb" => Opcode::JMPB,
            "eq" => Opcode::EQ,
            "neq" => Opcode::NEQ,
            "gt" => Opcode::GT,
            "lt" => Opcode::LT,
            "gtq" => Opcode::GTQ,
            "ltq" => Opcode::LTQ,
            "jeq" => Opcode::JEQ,
            "aloc" => Opcode::ALOC,
            "inc" => Opcode::INC,
            "dec" => Opcode::DEC,
            "rjmp" => Opcode::RJMP,
            "syscall" => Opcode::SYSC,
            _ => Opcode::IGL,
        }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTQ,
            14 => Opcode::LTQ,
            15 => Opcode::JEQ,
            16 => Opcode::ALOC,
            17 => Opcode::INC,
            18 => Opcode::DEC,
            19 => Opcode::RJMP,
            20 => Opcode::SYSC,
            _ => Opcode::IGL,
        }
    }
}
