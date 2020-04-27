use crate::Opcode;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }

    pub fn opcode(&self) -> &Opcode {
        return &self.opcode;
    }
}
