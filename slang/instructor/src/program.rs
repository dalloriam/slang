use crate::{Instruction, INSTRUCTION_LENGTH_BYTES};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, Opcode, Operand, Program};

    fn program_to_bytecode() {
        let prog = Program {
            instructions: vec![
                Instruction {
                    opcode: Some(Opcode::LOAD),
                    operand_1: Some(Operand::Register(0)),
                    operand_2: Some(Operand::Integer(100)),
                    operand_3: None,
                    ..Default::default()
                },
                Instruction {
                    opcode: Some(Opcode::LOAD),
                    operand_1: Some(Operand::Register(1)),
                    operand_2: Some(Operand::Integer(25)),
                    operand_3: None,
                    ..Default::default()
                },
            ],
        };

        let expected_bytecode: Vec<u8> = vec![01, 00, 00, 64, 01, 00, 00, 19];
        let actual_bytecode = prog.get_bytecode();
        assert_eq!(expected_bytecode, actual_bytecode);
    }
}
