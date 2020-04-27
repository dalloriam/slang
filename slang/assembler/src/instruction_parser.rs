use instructor::{Instruction, Operand};

use nom::{combinator::map, sequence::tuple, IResult};

use crate::{opcode_parser as opcode, operand_parser as operand, register_parser as register};

/// Parses instruction of the following form:
/// LOAD $0 #100
pub fn instruction_reg_int(i: &str) -> IResult<&str, Instruction> {
    let combined_tuple = tuple((opcode::load, register::register, operand::integer));
    map(combined_tuple, |(op, reg, dig)| Instruction {
        opcode: op,
        operand_1: Some(Operand::Register(reg)),
        operand_2: Some(Operand::Integer(dig)),
        operand_3: None,
    })(i)
}

#[cfg(test)]
mod tests {
    use instructor::{Instruction, Opcode, Operand};

    use super::instruction_reg_int;

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
}
