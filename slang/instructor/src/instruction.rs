use crate::{LabelConverter, Opcode, Operand};

/// A single Slang instruction.
#[derive(Debug, PartialEq)]
pub struct Instruction {
    /// The opcode of this instruction.
    ///
    /// Can be `None` if the current instruction is a label declaration
    /// or a directive.
    pub opcode: Option<Opcode>,

    /// The directive of this instruction.
    ///
    /// Can be `None` if the current instruction is an opcode.
    pub directive: Option<String>, // TODO: Parse directives as enums also.

    /// The label of this instruction.
    ///
    /// `None` if no label specified.
    pub label: Option<String>,

    /// The optional first operand of the instruction.
    pub operand_1: Option<Operand>,

    /// The optional second operand of the instruction.
    pub operand_2: Option<Operand>,

    /// The optional third operand of the instruction.
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

    /// Returns whether the instruction has at least one operand.
    ///
    /// # Examples
    /// ```
    /// use instructor::{Instruction, Operand};
    ///
    /// let instruction_b = Instruction{
    ///     operand_1: Some(Operand::Register(1)),
    ///     ..Default::default()
    /// };
    /// assert!(instruction_b.has_operands());
    /// ```
    pub fn has_operands(&self) -> bool {
        self.operand_1.is_some() || self.operand_2.is_some() || self.operand_3.is_some()
    }

    /// Returns whether or not the instruction has a label.
    ///
    /// # Examples
    /// ```
    /// use instructor::Instruction;
    ///
    /// let instruction = Instruction{
    ///     label: Some(String::from("hello")),
    ///     ..Default::default()
    /// };
    /// assert!(instruction.is_label());
    /// ```
    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    /// Returns the label name.
    pub fn label_name(&self) -> Option<&String> {
        self.label.as_ref()
    }

    /// Represent the instruction as a sequence of bytes.
    pub fn write_bytes<T: LabelConverter>(&self, w: &mut Vec<u8>, converter: &T) {
        assert!(self.opcode.is_some()); // Cannot fail from user input. If this assert trips, it means the assembler has a bug somewhere.

        w.push(self.opcode.unwrap() as u8);

        // Write all instructions to the stream & gather byte count.
        let cur_size = &[&self.operand_1, &self.operand_2, &self.operand_3]
            .iter()
            .map(|op| Instruction::write_operand(*op, w, converter))
            .sum::<usize>();
        assert!(cur_size <= &3);

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
            opcode: None,
            operand_1: None,
            operand_2: None,
            operand_3: None,
            label: None,
            directive: None,
        }
    }
}
