use byteorder::{LittleEndian, WriteBytesExt};

use crate::LabelConverter;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MemorySection {
    Stack,
    Heap,
}

impl From<MemorySection> for u8 {
    fn from(s: MemorySection) -> u8 {
        match s {
            MemorySection::Stack => 0,
            MemorySection::Heap => 1,
        }
    }
}

impl From<u8> for MemorySection {
    fn from(u: u8) -> MemorySection {
        match u {
            0 => MemorySection::Stack,
            1 => MemorySection::Heap,
            _ => panic!("Invalid section"), // TODO: Handle error properly with TryFrom.
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Address {
    pub offset: u8,
    pub register: u8,
    pub section: MemorySection,
}

impl Address {
    pub fn new_heap(register: u8, offset: u8) -> Address {
        Address {
            register,
            offset,
            section: MemorySection::Heap,
        }
    }

    pub fn new_stack(register: u8, offset: u8) -> Address {
        Address {
            register,
            offset,
            section: MemorySection::Stack,
        }
    }
}

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
    Address(Address),
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
            Operand::Address(addr) => {
                w.push(addr.register);
                w.push(addr.offset);
                w.push(addr.section.into());
                3
            }
            Operand::Str(_s) => panic!(
                "String operands should never be written. They should be stripped beforehand."
            ),
        }
    }
}
