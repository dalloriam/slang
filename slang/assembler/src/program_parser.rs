use instructor::Program;

use nom::{combinator::map, multi::many1, IResult};

use crate::instruction_parser as instruction;

pub fn program(i: &str) -> IResult<&str, Program> {
    map(many1(instruction::instruction_reg_int), |instructions| {
        Program { instructions }
    })(i)
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
                    operand_3: None,
                },
                Instruction {
                    opcode: Opcode::LOAD,
                    operand_1: Some(Operand::Register(1)),
                    operand_2: Some(Operand::Integer(25)),
                    operand_3: None,
                },
            ],
        };
        let (rest, actual_program) = program("load $0 #100\nload $1 #25").unwrap();
        assert_eq!(rest, "");
        assert_eq!(expected_program, actual_program);
    }
}
