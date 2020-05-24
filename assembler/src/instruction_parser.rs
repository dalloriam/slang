use instructor::{Instruction, Opcode};

use nom::{
    combinator::map, combinator::opt, error::ErrorKind, sequence::tuple, Err as NErr, IResult,
};

use crate::{label_parser as label, opcode_parser as opcode, operand_parser as operand};

fn opcode_instr(opcode: Opcode, lbl: Option<String>, rest: &str) -> IResult<&str, Instruction> {
    let (r, ins) = match opcode {
        Opcode::LOAD | Opcode::LCW => {
            map(tuple((operand::register, operand::offset)), |(reg, i)| {
                Instruction {
                    label: lbl.clone(),
                    opcode: Some(opcode),
                    operand_1: Some(reg),
                    operand_2: Some(i),
                    ..Default::default()
                }
            })(rest)?
        }
        Opcode::ADD | Opcode::SUB | Opcode::MUL | Opcode::DIV => map(
            tuple((operand::register, operand::register, operand::register)),
            |(a, b, c)| Instruction {
                label: lbl.clone(),
                opcode: Some(opcode),
                operand_1: Some(a),
                operand_2: Some(b),
                operand_3: Some(c),
                ..Default::default()
            },
        )(rest)?,
        Opcode::JMP | Opcode::JMPF | Opcode::JMPB | Opcode::JEQ | Opcode::CALL => {
            map(operand::offset, |i| Instruction {
                label: lbl.clone(),
                opcode: Some(opcode),
                operand_1: Some(i),
                ..Default::default()
            })(rest)?
        }
        Opcode::EQ
        | Opcode::NEQ
        | Opcode::GT
        | Opcode::GTQ
        | Opcode::LT
        | Opcode::LTQ
        | Opcode::MOV => map(tuple((operand::register, operand::register)), |(a, b)| {
            Instruction {
                label: lbl.clone(),
                opcode: Some(opcode),
                operand_1: Some(a),
                operand_2: Some(b),
                ..Default::default()
            }
        })(rest)?,
        Opcode::INC | Opcode::DEC | Opcode::RJMP | Opcode::PUSH | Opcode::POP => {
            map(operand::register, |r| Instruction {
                label: lbl.clone(),
                opcode: Some(opcode),
                operand_1: Some(r),
                ..Default::default()
            })(rest)?
        }
        Opcode::SYSC | Opcode::IGL | Opcode::RET => Ok((
            rest,
            Instruction {
                label: lbl.clone(),
                opcode: Some(opcode),
                ..Default::default()
            },
        ))?,
        Opcode::SW | Opcode::LW | Opcode::SB | Opcode::LB => map(
            tuple((operand::register, operand::address)),
            |(reg, add)| Instruction {
                label: lbl.clone(),
                opcode: Some(opcode),
                operand_1: Some(reg),
                operand_2: Some(add),
                ..Default::default()
            },
        )(rest)?,
    };

    Ok((r, ins))
}

pub fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (rest, (lbl, opcode)) = tuple((opt(label::label_declaration), opt(opcode::opcode)))(i)?;

    match opcode {
        Some(opc) => opcode_instr(opc, lbl, rest),
        None => {
            if lbl.is_none() {
                return Err(NErr::Error((i, ErrorKind::Verify)));
            }
            Ok((
                rest,
                Instruction {
                    label: lbl,
                    ..Default::default()
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use instructor::{Instruction, Opcode, Operand};

    use super::instruction;

    #[test]
    fn parse_instruction_simple() {
        let (rest, instr) = instruction("ld $0 10\n").unwrap();
        let expected_instruction = Instruction {
            opcode: Some(Opcode::LOAD),
            operand_1: Some(Operand::Register(0)),
            operand_2: Some(Operand::Integer(10)),
            ..Default::default()
        };
        assert_eq!(rest, "");
        assert_eq!(expected_instruction, instr);
    }

    #[test]
    fn parse_instruction_reg_int() {
        let (rest, instr) = instruction("ld $0 100\n").unwrap();

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

    #[test]
    fn parse_instruction_invalid_semantics() {
        // Tests that an instruction that is syntactically valid but that has wrong
        // typing will be rejected.
        assert!(instruction("ld 100 $0").is_err());
    }
}
