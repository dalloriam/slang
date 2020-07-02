/// All available Opcodes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    /// Illegal Instruction.
    IGL,

    /// Load - Loads a 32-bit integer in a register.
    LOAD,

    /// Add - Adds the values of the first two registers, storing the result in the third.
    ADD,

    /// Substract - Subtracts the value of the second register from the first, storing the result in the third.
    SUB,

    /// Multiply - Multiplies the values of the first two registers, storing the result in the third.
    MUL,

    /// Divide - Divide the value of the first register by the value of the second, storing the integer result in the third register,
    /// and the remainder in the `$r0` register.
    DIV,

    /// Short Absolute Jump - Jump to the label or offset specified as an argument.
    JMP,

    /// Short Relative Jump (fwd) - Jump {x} instructions forward.
    JMPF,

    /// Short Relative Jump (bwd) - Jump {x} instructions backwards.
    JMPB,

    /// Equal - Checks the values of the two provided registers for equality.
    EQ,

    /// Not Equal - Complement of `EQ`
    NEQ,

    /// Greater Than - Checks if the value of the first register is greater than the value of the second.
    GT,

    /// Lower Than - Checks if the value of the first register is lower than the value of the second.
    LT,

    /// Greater Than or Equal - Checks if the value of the first register is greater than or equal to the value of the second.
    GTQ,

    /// Lower Than or Equal - Checks if the value of the first register is lower than or equal to the value of the second.
    LTQ,

    /// Short jump if equal - If the result of the previous comparison was true, jump to the offset or label specified.
    JEQ,

    /// Increment - Increments the value of the specified register by one.
    INC,

    /// Decrement - Decrements the value of the specified register by one.
    DEC,

    /// Long absolute jump - Jumps to the offset stored in the specified register.
    RJMP,

    /// Syscall - Executes the syscall currently stored in the $v0 virtual register.
    SYSC,

    /// Stack push word - Pushes the value of the first register on the stack.
    PUSHW,

    /// Stack pop word - Pops the first value off the stack and writes it to the first register.
    POPW,

    /// Move - Copies the value from the first register to the second.
    MOV,

    /// Load constant word - Loads a word (i32) from the readonly section.
    LCW,

    /// Set word - Writes a word (i32) from memory.
    SW,

    /// Load word - Loads a word from memory and writes it to the first register.
    LW,

    // Set byte - Writes a single byte to memory.
    SB,

    // Load byte - Loads a single byte from memory.
    LB,

    // Call - Calls a procedure.
    CALL,

    // Ret - Returns from a procedure.
    RET,

    // Negate - inverts the value of a register.
    NEG,

    // Push Byte - Pushes a byte on the stack.
    PUSHB,

    // Pop byte - Pops a byte from the stack.
    POPB,

    // Bitwise logical negation.
    NOT,

    // Left Bitshift
    SHIFTL,

    // Right Bitshift
    SHIFTR,

    // Bitwise and.
    AND,

    // Bitwise OR.
    OR,

    /// Jump if zero. Jumps to the specified offset if the value of the specified register is zero.
    JEZ,
}

impl Opcode {
    pub fn width(self) -> u16 {
        match self {
            Opcode::LOAD => 4,
            Opcode::ADD => 4,
            Opcode::SUB => 4,
            Opcode::MUL => 4,
            Opcode::DIV => 4,
            Opcode::JMP => 3,
            Opcode::JMPF => 3,
            Opcode::JMPB => 3,
            Opcode::RJMP => 2,
            Opcode::EQ => 3,
            Opcode::NEQ => 3,
            Opcode::GT => 3,
            Opcode::LT => 3,
            Opcode::GTQ => 3,
            Opcode::LTQ => 3,
            Opcode::JEQ => 3,
            Opcode::INC => 2,
            Opcode::DEC => 2,
            Opcode::SYSC => 1,
            Opcode::PUSHW => 2,
            Opcode::POPW => 2,
            Opcode::MOV => 3,
            Opcode::LCW => 4,
            Opcode::SW => 5,
            Opcode::LW => 5,
            Opcode::SB => 5,
            Opcode::LB => 5,
            Opcode::CALL => 3,
            Opcode::RET => 1,
            Opcode::NEG => 2,
            Opcode::PUSHB => 2,
            Opcode::POPB => 2,
            Opcode::NOT => 2,
            Opcode::SHIFTL => 3,
            Opcode::SHIFTR => 3,
            Opcode::AND => 3,
            Opcode::OR => 3,
            Opcode::JEZ => 4,
            Opcode::IGL => 1,
        }
    }
}

impl<'a> From<&'a str> for Opcode {
    fn from(v: &'a str) -> Self {
        match v {
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
            "inc" => Opcode::INC,
            "dec" => Opcode::DEC,
            "rjmp" => Opcode::RJMP,
            "syscall" => Opcode::SYSC,
            "pushw" => Opcode::PUSHW,
            "popw" => Opcode::POPW,
            "move" => Opcode::MOV,
            "lcw" => Opcode::LCW,
            "sw" => Opcode::SW,
            "lw" => Opcode::LW,
            "sb" => Opcode::SB,
            "lb" => Opcode::LB,
            "call" => Opcode::CALL,
            "ret" => Opcode::RET,
            "neg" => Opcode::NEG,
            "pushb" => Opcode::PUSHB,
            "popb" => Opcode::POPB,
            "not" => Opcode::NOT,
            "shl" => Opcode::SHIFTL,
            "shr" => Opcode::SHIFTR,
            "and" => Opcode::AND,
            "or" => Opcode::OR,
            "jez" => Opcode::JEZ,
            _ => Opcode::IGL,
        }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
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
            16 => Opcode::INC,
            17 => Opcode::DEC,
            18 => Opcode::RJMP,
            19 => Opcode::SYSC,
            20 => Opcode::PUSHW,
            21 => Opcode::POPW,
            22 => Opcode::MOV,
            23 => Opcode::LCW,
            24 => Opcode::SW,
            25 => Opcode::LW,
            26 => Opcode::SB,
            27 => Opcode::LB,
            28 => Opcode::CALL,
            29 => Opcode::RET,
            30 => Opcode::NEG,
            31 => Opcode::PUSHB,
            32 => Opcode::POPB,
            33 => Opcode::NOT,
            34 => Opcode::SHIFTL,
            35 => Opcode::SHIFTR,
            36 => Opcode::AND,
            37 => Opcode::OR,
            38 => Opcode::JEZ,
            _ => Opcode::IGL,
        }
    }
}
