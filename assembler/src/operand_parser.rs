use instructor::Operand;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use snafu::{ResultExt, Snafu};

use crate::{common::whitespace, label_parser as label};

#[derive(Debug, Snafu)]
enum ParseError {
    InvalidRegisterNumber { source: std::num::ParseIntError },
    InvalidRegisterRange,
}

pub fn operand(i: &str) -> IResult<&str, Operand> {
    alt((
        address,
        integer,
        register,
        map(label::label_usage, Operand::Label),
        string,
    ))(i)
}

fn address(i: &str) -> IResult<&str, Operand> {
    let addr = delimited(whitespace, register, whitespace);
    let tup = delimited(
        whitespace,
        tuple((byte, delimited(char('('), addr, char(')')))),
        whitespace,
    );
    map(tup, |(offset, reg)| {
        if let Operand::Register(reg_byte) = reg {
            return Operand::Address((offset, reg_byte));
        }
        panic!("The register parser returned a non-register operand");
    })(i)
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

fn byte(i: &str) -> IResult<&str, u8> {
    map_res(delimited(whitespace, digit1, whitespace), |b_val: &str| {
        b_val.parse::<u8>()
    })(i)
}

fn integer(i: &str) -> IResult<&str, Operand> {
    map(
        map_res(
            delimited(whitespace, digit1, whitespace),
            |int_val: &str| int_val.parse::<i32>(),
        ),
        Operand::Integer,
    )(i)
}

fn register(i: &str) -> IResult<&str, Operand> {
    map(
        alt((
            map_res(
                delimited(whitespace, preceded(char('$'), digit1), whitespace),
                |byte_val: &str| {
                    let v = byte_val.parse::<u8>().context(InvalidRegisterNumber)?;
                    if v > 31 {
                        return Err(ParseError::InvalidRegisterRange);
                    }

                    Ok(v)
                },
            ),
            map(
                delimited(whitespace, preceded(char('$'), tag("v0")), whitespace),
                |_val| 32 as u8,
            ),
        )),
        Operand::Register,
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{address, integer, operand, register, string, Operand};

    #[test]
    fn parse_address() {
        let (rest, addr) = address(" 18($3 ) ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(addr, Operand::Address((18, 3)));
    }

    #[test]
    fn parse_integer() {
        {
            let (rest, reg) = integer("10 ").unwrap();
            assert_eq!(reg, Operand::Integer(10));
            assert_eq!(rest, "");
        }

        {
            let (_rest, reg) = integer("400").unwrap();
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
        {
            assert!(register("$32").is_err());
        }
        {
            // Test parsing of special register $v0 (used for syscalls).
            let (rest, reg) = register("$v0").unwrap();
            assert_eq!(reg, Operand::Register(32));
            assert_eq!(rest, "");
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
