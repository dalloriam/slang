use instructor::{Instruction, Operand};

use nom::{combinator::map, sequence::tuple, IResult};

use crate::{opcode_parser as opcode, operand_parser as operand, register_parser as register};

/// Parses instructions of the following form:
/// HLT
pub fn instruction_simple(i: &str) -> IResult<&str, Instruction> {
    map(opcode::opcode, |op| Instruction {
        opcode: op,
        operand_1: None,
        operand_2: None,
        operand_3: None,
    })(i)
}

/// Parses instruction of the following form:
/// LOAD $0 #100
pub fn instruction_reg_int(i: &str) -> IResult<&str, Instruction> {
    let combined_tuple = tuple((opcode::opcode, register::register, operand::integer));
    map(combined_tuple, |(op, reg, dig)| Instruction {
        opcode: op,
        operand_1: Some(Operand::Register(reg)),
        operand_2: Some(Operand::Integer(dig)),
        operand_3: None,
    })(i)
}

pub fn instruction_reg_reg_reg(i: &str) -> IResult<&str, Instruction> {
    let combined_tuple = tuple((
        opcode::opcode,
        register::register,
        register::register,
        register::register,
    ));
    map(combined_tuple, |(op, src1, src2, dst)| Instruction {
        opcode: op,
        operand_1: Some(Operand::Register(src1)),
        operand_2: Some(Operand::Register(src2)),
        operand_3: Some(Operand::Register(dst)),
    })(i)
}

#[cfg(test)]
mod tests {
    use instructor::{Instruction, Opcode, Operand};

    use super::{instruction_reg_int, instruction_reg_reg_reg, instruction_simple};

    #[test]
    fn parse_instruction_simple() {
        let (rest, instr) = instruction_simple("hlt\n").unwrap();
        let expected_instruction = Instruction {
            opcode: Opcode::HLT,
            operand_1: None,
            operand_2: None,
            operand_3: None,
        };
        assert_eq!(rest, "");
        assert_eq!(expected_instruction, instr);
    }

    #[test]
    fn parse_instruction_reg_int() {
        let (rest, instr) = instruction_reg_int("load $0 #100\n").unwrap();

        let expected_instruction = Instruction {
            opcode: Opcode::LOAD,
            operand_1: Some(Operand::Register(0)),
            operand_2: Some(Operand::Integer(100)),
            operand_3: None,
        };
        assert_eq!(expected_instruction, instr);
        assert_eq!(rest, "");
    }

    #[test]
    fn parse_instruction_reg_reg_reg() {
        let (rest, instr) = instruction_reg_reg_reg("add $0 $1 $2\n").unwrap();
        assert_eq!(rest, "");

        let expected_instruction = Instruction {
            opcode: Opcode::ADD,
            operand_1: Some(Operand::Register(0)),
            operand_2: Some(Operand::Register(1)),
            operand_3: Some(Operand::Register(2)),
        };
        assert_eq!(expected_instruction, instr);
    }
}
