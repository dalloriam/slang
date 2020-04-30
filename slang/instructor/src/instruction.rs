use crate::{LabelConverter, Opcode, Operand};

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Option<Opcode>,
    pub directive: Option<String>,
    pub label: Option<String>,

    pub operand_1: Option<Operand>,
    pub operand_2: Option<Operand>,
    pub operand_3: Option<Operand>, // Each instruction is max 4 bytes.
}

impl Instruction {
    fn write_operand<T: LabelConverter>(
        op: &Option<Operand>,
        w: &mut Vec<u8>,
        converter: &T,
    ) -> usize {
        if let Some(oper) = op {
            oper.write_bytes(w, converter)
        } else {
            0
        }
    }

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn label_name(&self) -> Option<&String> {
        self.label.as_ref()
    }

    /// Represent the instruction as a sequence of bytes.
    pub fn write_bytes<T: LabelConverter>(&self, w: &mut Vec<u8>, converter: &T) {
        w.push(self.opcode.unwrap() as u8); // TODO HACK IMPORTANT: REMOVE UNWRAP HERE BEFORE USING DIRECTIVES.

        // Write all instructions to the stream & gather byte count.
        let cur_size = &[&self.operand_1, &self.operand_2, &self.operand_3]
            .iter()
            .map(|op| Instruction::write_operand(*op, w, converter))
            .sum::<usize>();

        // Padding to ensure instructions always 4 bytes wide.
        // We loop to 3 instead of 4 because the opcode is a guaranteed write, so we can write 1-4 bytes, inclusively.
        for _i in *cur_size..3 {
            w.push(0);
        }
    }
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            opcode: Some(Opcode::IGL),
            operand_1: None,
            operand_2: None,
            operand_3: None,
            label: None,
            directive: None,
        }
    }
}
