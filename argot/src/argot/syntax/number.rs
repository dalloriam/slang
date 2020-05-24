use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1},
    combinator::map_res,
    sequence::{delimited, preceded},
    IResult,
};

use crate::syntax::common::whitespace;

pub fn integer(i: &str) -> IResult<&str, i32> {
    alt((hex_int, decimal_int))(i)
}

fn decimal_int(i: &str) -> IResult<&str, i32> {
    map_res(
        delimited(whitespace, digit1, whitespace),
        |int_val: &str| int_val.parse::<i32>(),
    )(i)
}

fn hex_int(i: &str) -> IResult<&str, i32> {
    // TODO: Detect hex int overflows, this doesn't catch it.
    map_res(
        delimited(whitespace, preceded(tag("0x"), hex_digit1), whitespace),
        |rs| i32::from_str_radix(rs, 16),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::integer;

    #[test]
    fn decimal_integer() {
        let (rest, i) = integer(" 192\n").unwrap();
        assert_eq!(rest, "");
        assert_eq!(i, 192);
    }

    #[test]
    fn decimal_integer_overflow() {
        assert!(integer("99999999999999999999999999999999999999").is_err());
    }

    #[test]
    fn hexadecimal_integer() {
        let (rest, i) = integer("0x002A").unwrap();
        assert_eq!(rest, "");
        assert_eq!(i, 42);
    }

    #[test]
    fn hexadecimal_integer_overflow() {
        let (rest, i) = integer("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
        assert_eq!(i, 0);
    }
}
