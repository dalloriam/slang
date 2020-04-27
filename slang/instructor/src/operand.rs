#[derive(Debug, PartialEq)]
pub enum Operand {
    Integer(i32),
    Register(u8),
}

impl Operand {
    pub fn write_bytes(&self, w: &mut Vec<u8>) {
        match self {
            Operand::Register(reg_byte) => w.push(*reg_byte),
            Operand::Integer(op_int) => {
                let converted = *op_int as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                w.push(byte2 as u8);
                w.push(byte1 as u8);
            }
        }
    }
}
