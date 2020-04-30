use instructor::Operand;

use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::{delimited, preceded},
    IResult,
};

use crate::common::whitespace;

pub fn operand(i: &str) -> IResult<&str, Operand> {
    alt((integer, register))(i)
}

fn integer(i: &str) -> IResult<&str, Operand> {
    map(
        map_res(
            delimited(whitespace, preceded(char('#'), digit1), whitespace),
            |int_val: &str| int_val.parse::<i32>(),
        ),
        |i_val| Operand::Integer(i_val),
    )(i)
}

fn register(i: &str) -> IResult<&str, Operand> {
    map(
        map_res(
            delimited(whitespace, preceded(char('$'), digit1), whitespace),
            |byte_val: &str| byte_val.parse::<u8>(),
        ),
        |i_val| Operand::Register(i_val),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{integer, register, Operand};

    #[test]
    fn parse_integer_operand() {
        {
            let (rest, reg) = integer("#10 ").unwrap();
            assert_eq!(reg, Operand::Integer(10));
            assert_eq!(rest, "");
        }

        {
            let (_rest, reg) = integer("#400").unwrap();
            assert_eq!(reg, Operand::Integer(400));
        }

        {
            assert!(integer("#asdf").is_err());
        }
    }

    #[test]
    fn parse_register() {
        {
            let (rest, reg) = register("$18").unwrap();
            assert_eq!(reg, Operand::Register(18));
            assert_eq!(rest, "");
        }
        {
            let (_rest, reg) = register(" $18").unwrap();
            assert_eq!(reg, Operand::Register(18));
        }

        {
            assert!(register("$400").is_err());
        }
    }
}
