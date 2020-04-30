use instructor::Instruction;

use nom::{combinator::map, combinator::opt, sequence::tuple, IResult};

use crate::{opcode_parser as opcode, operand_parser as operand};

pub fn instruction(i: &str) -> IResult<&str, Instruction> {
    let comb_tuple = tuple((
        opcode::opcode,
        opt(operand::operand),
        opt(operand::operand),
        opt(operand::operand),
    ));

    map(comb_tuple, |(opc, op1, op2, op3)| Instruction {
        opcode: Some(opc),
        directive: None,
        label: None,
        operand_1: op1,
        operand_2: op2,
        operand_3: op3,
    })(i)
}

#[cfg(test)]
mod tests {
    use instructor::{Instruction, Opcode, Operand};

    use super::instruction;

    #[test]
    fn parse_instruction_simple() {
        let (rest, instr) = instruction("hlt\n").unwrap();
        let expected_instruction = Instruction {
            opcode: Some(Opcode::HLT),
            ..Default::default()
        };
        assert_eq!(rest, "");
        assert_eq!(expected_instruction, instr);
    }

    #[test]
    fn parse_instruction_reg_int() {
        let (rest, instr) = instruction("ld $0 #100\n").unwrap();

        let expected_instruction = Instruction {
            opcode: Some(Opcode::LOAD),
            operand_1: Some(Operand::Register(0)),
            operand_2: Some(Operand::Integer(100)),
            ..Default::default()
        };
        assert_eq!(expected_instruction, instr);
        assert_eq!(rest, "");
    }

    #[test]
    fn parse_instruction_reg_reg_reg() {
        let (rest, instr) = instruction("add $0 $1 $2\n").unwrap();
        assert_eq!(rest, "");

        let expected_instruction = Instruction {
            opcode: Some(Opcode::ADD),
            operand_1: Some(Operand::Register(0)),
            operand_2: Some(Operand::Register(1)),
            operand_3: Some(Operand::Register(2)),
            ..Default::default()
        };
        assert_eq!(expected_instruction, instr);
    }
}
