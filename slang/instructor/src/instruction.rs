use crate::{Opcode, Operand};

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand_1: Option<Operand>,
    pub operand_2: Option<Operand>,
    pub operand_3: Option<Operand>, // Each instruction is max 4 bytes.
}

impl Instruction {
    fn write_operand(op: &Option<Operand>, w: &mut Vec<u8>) {
        if let Some(oper) = op {
            oper.write_bytes(w)
        }
    }

    pub fn write_bytes(&self, w: &mut Vec<u8>) {
        w.push(self.opcode as u8);

        Instruction::write_operand(&self.operand_1, w);
        Instruction::write_operand(&self.operand_2, w);
        Instruction::write_operand(&self.operand_3, w);
    }
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            opcode: Opcode::IGL,
            operand_1: None,
            operand_2: None,
            operand_3: None,
        }
    }
}
