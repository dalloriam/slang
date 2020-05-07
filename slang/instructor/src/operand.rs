use byteorder::{LittleEndian, WriteBytesExt};

use crate::LabelConverter;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Integer(i32),
    Register(u8),
    Label(String),
    Str(String),
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
                wtr.write_u32::<LittleEndian>(offset).unwrap(); // TODO: Handle.
                w.push(wtr[1]);
                w.push(wtr[0]);
                2
            }
            Operand::Str(s) => unimplemented!(),
        }
    }
}
