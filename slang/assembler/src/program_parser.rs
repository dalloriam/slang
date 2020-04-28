use instructor::Program;

use nom::{branch::alt, combinator::map, multi::many1, IResult};

use crate::instruction_parser as instruction;

pub fn program(i: &str) -> IResult<&str, Program> {
    map(
        many1(alt((
            instruction::instruction_reg_reg_reg,
            instruction::instruction_reg_int,
            instruction::instruction_simple,
        ))),
        |instructions| Program { instructions },
    )(i)
}

#[cfg(test)]
mod tests {
    use instructor::{Instruction, Opcode, Operand, Program};

    use super::program;

    #[test]
    fn parse_program() {
        let expected_program = Program {
            instructions: vec![
                Instruction {
                    opcode: Opcode::LOAD,
                    operand_1: Some(Operand::Register(0)),
                    operand_2: Some(Operand::Integer(100)),
                    ..Default::default()
                },
                Instruction {
                    opcode: Opcode::LOAD,
                    operand_1: Some(Operand::Register(1)),
                    operand_2: Some(Operand::Integer(25)),
                    ..Default::default()
                },
                Instruction {
                    opcode: Opcode::ADD,
                    operand_1: Some(Operand::Register(0)),
                    operand_2: Some(Operand::Register(1)),
                    operand_3: Some(Operand::Register(2)),
                },
                Instruction {
                    opcode: Opcode::HLT,
                    ..Default::default()
                },
            ],
        };
        let (rest, actual_program) =
            program("load $0 #100\nload $1 #25\nadd $0 $1 $2\nhlt").unwrap();
        assert_eq!(rest, "");
        assert_eq!(expected_program, actual_program);
    }
}
