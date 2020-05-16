use instructor::Operand;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::{delimited, preceded},
    IResult,
};

use crate::{common::whitespace, label_parser as label};

pub fn operand(i: &str) -> IResult<&str, Operand> {
    alt((
        integer,
        register,
        map(label::label_usage, |lbl| Operand::Label(lbl)),
        string,
    ))(i)
}

fn string(i: &str) -> IResult<&str, Operand> {
    map(
        delimited(
            whitespace,
            delimited(char('"'), take_till(|c| c == '"'), char('"')),
            whitespace,
        ),
        |s| Operand::Str(String::from(s)),
    )(i)
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
        alt((
            map_res(
                delimited(whitespace, preceded(char('$'), digit1), whitespace),
                |byte_val: &str| byte_val.parse::<u8>(), // TODO: Validate that the specified register is 0-31
            ),
            map(
                delimited(whitespace, preceded(char('$'), tag("v0")), whitespace),
                |_val| 32 as u8,
            ),
        )),
        |i_val| Operand::Register(i_val),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{integer, operand, register, string, Operand};

    #[test]
    fn parse_integer() {
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

    #[test]
    fn parse_string() {
        let (rest, s) = string("\"hello\"").unwrap();
        assert_eq!(s, Operand::Str(String::from("hello")));
        assert_eq!(rest, "");
    }

    #[test]
    fn parse_operand() {
        // TODO: Convert this test to a macro test.
        {
            let (rest, op) = operand("\"hello world\"").unwrap();
            assert_eq!(rest, "");
            assert_eq!(op, Operand::Str(String::from("hello world")));
        }
    }
}
