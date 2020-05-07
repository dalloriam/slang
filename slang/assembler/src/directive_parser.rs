use instructor::Instruction;

use nom::{
    character::complete::{alpha1, char},
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{common::whitespace, label_parser::label_declaration, operand_parser::operand};

fn directive_declaration(i: &str) -> IResult<&str, &str> {
    delimited(whitespace, preceded(char('.'), alpha1), whitespace)(i)
}

pub fn directive(i: &str) -> IResult<&str, Instruction> {
    let comb_tuple = tuple((
        opt(label_declaration),
        directive_declaration,
        opt(operand),
        opt(operand),
        opt(operand),
    ));

    map(comb_tuple, |(lbl, dir_name, op1, op2, op3)| Instruction {
        directive: Some(String::from(dir_name)),
        operand_1: op1,
        operand_2: op2,
        operand_3: op3,
        label: lbl,
        ..Default::default()
    })(i)
}

#[cfg(test)]
mod tests {
    use instructor::{Instruction, Operand};

    use super::directive;

    #[test]
    fn test_string_directive() {
        let (rest, actual) = directive("test: .asciiz \"Hello\"").unwrap();
        assert_eq!(rest, "");

        let expected = Instruction {
            label: Some(String::from("test")),
            directive: Some(String::from("asciiz")),
            operand_1: Some(Operand::Str(String::from("Hello"))),
            ..Default::default()
        };

        assert_eq!(expected, actual);
    }
}
