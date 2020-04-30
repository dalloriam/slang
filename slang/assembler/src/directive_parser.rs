use instructor::Instruction;

use nom::{
    character::complete::{alpha1, char},
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{common::whitespace, operand_parser::operand};

fn directive_declaration(i: &str) -> IResult<&str, &str> {
    delimited(whitespace, preceded(char('.'), alpha1), whitespace)(i)
}

pub fn directive(i: &str) -> IResult<&str, Instruction> {
    let comb_tuple = tuple((
        directive_declaration,
        opt(operand),
        opt(operand),
        opt(operand),
    ));

    map(comb_tuple, |(dir_name, op1, op2, op3)| Instruction {
        directive: Some(String::from(dir_name)),
        operand_1: op1,
        operand_2: op2,
        operand_3: op3,
        ..Default::default()
    })(i)
}
