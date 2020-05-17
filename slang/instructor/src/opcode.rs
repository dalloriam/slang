/// All available Opcodes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    /// Halt - Stops the VM immediately.
    #[deprecated(note = "Use an exit syscall instead")]
    HLT,

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

    /// Allocation - Resizes the heap to the value of the first register.
    ALOC,

    /// Increment - Increments the value of the specified register by one.
    INC,

    /// Decrement - Decrements the value of the specified register by one.
    DEC,

    /// Long absolute jump - Jumps to the offset stored in the specified register.
    RJMP,

    /// Syscall - Executes the syscall currently stored in the $v0 virtual register.
    SYSC,

    /// Stack push - Pushes the value of the first register on the stack.
    PUSH,

    /// Stack pop - Pops the first value off the stack and writes it to the first register.
    POP,

    /// Illegal Instruction.
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
            "push" => Opcode::PUSH,
            "pop" => Opcode::POP,
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
            21 => Opcode::PUSH,
            22 => Opcode::POP,
            _ => Opcode::IGL,
        }
    }
}
