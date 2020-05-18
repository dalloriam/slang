use byteorder::{LittleEndian, WriteBytesExt};

use crate::LabelConverter;

/// Operand Types.
#[derive(Debug, PartialEq)]
pub enum Operand {
    /// Integer literal operand.
    Integer(i32),

    /// Register operand.
    Register(u8),

    /// Label operand.
    Label(String),

    /// String operand.
    Str(String),

    /// Address operand. (offset + register)
    Address((u8, u8)),
}

impl Operand {
    pub fn write_bytes<T: LabelConverter>(&self, w: &mut Vec<u8>, converter: &T) -> usize {
        match self {
            Operand::Register(reg_byte) => {
                w.push(*reg_byte);
                1
            }
            Operand::Integer(op_int) => {
                let converted = *op_int as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                w.push(byte2 as u8);
                w.push(byte1 as u8);
                2
            }
            Operand::Label(s) => {
                let mut wtr = Vec::with_capacity(2);
                let offset = converter.offset_of(&s).unwrap(); // TODO: Handle error;
                wtr.write_u16::<LittleEndian>(offset).unwrap(); // TODO: Handle.
                w.push(wtr[1]);
                w.push(wtr[0]);
                2
            }
            Operand::Address((offset, reg)) => {
                // Swap the address tuple to allow the vm to do
                // let ptr = self.registers[self.next_8_bits()] + self.next_8_bits();
                w.push(*reg);
                w.push(*offset);
                2
            }
            Operand::Str(_s) => panic!(
                "String operands should never be written. They should be stripped beforehand."
            ),
        }
    }
}
