use instructor::Program;

use nom::{branch::alt, combinator::map, multi::many1, IResult};

use crate::directive_parser::directive;
use crate::instruction_parser as instruction;

pub fn program(i: &str) -> IResult<&str, Program> {
    map(
        many1(alt((instruction::instruction, directive))),
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
                    opcode: Some(Opcode::LOAD),
                    operand_1: Some(Operand::Register(0)),
                    operand_2: Some(Operand::Integer(100)),
                    ..Default::default()
                },
                Instruction {
                    opcode: Some(Opcode::LOAD),
                    operand_1: Some(Operand::Register(1)),
                    operand_2: Some(Operand::Integer(25)),
                    ..Default::default()
                },
                Instruction {
                    opcode: Some(Opcode::ADD),
                    operand_1: Some(Operand::Register(0)),
                    operand_2: Some(Operand::Register(1)),
                    operand_3: Some(Operand::Register(2)),
                    ..Default::default()
                },
                Instruction {
                    opcode: Some(Opcode::HLT),
                    ..Default::default()
                },
            ],
        };
        let (rest, actual_program) = program("ld $0 100\nld $1 25\nadd $0 $1 $2\nhlt").unwrap();
        assert_eq!(rest, "");
        assert_eq!(expected_program, actual_program);
    }

    #[test]
    fn parse_program_with_directive() {
        let source = ".data\nhello: .asciiz \"Hello, world\"\n.code\nhlt";

        let expected_program = Program {
            instructions: vec![
                Instruction {
                    directive: Some(String::from("data")),
                    ..Default::default()
                },
                Instruction {
                    label: Some(String::from("hello")),
                    directive: Some(String::from("asciiz")),
                    operand_1: Some(Operand::Str(String::from("Hello, world"))),
                    ..Default::default()
                },
                Instruction {
                    directive: Some(String::from("code")),
                    ..Default::default()
                },
                Instruction {
                    opcode: Some(Opcode::HLT),
                    ..Default::default()
                },
            ],
        };

        let (rest, actual_program) = program(source).unwrap();
        assert_eq!(rest, "");
        assert_eq!(expected_program, actual_program);
    }
}
