use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::{delimited, preceded},
    IResult,
};

use crate::common::whitespace;

pub fn integer(i: &str) -> IResult<&str, i32> {
    map_res(
        delimited(whitespace, preceded(char('#'), digit1), whitespace),
        |int_val: &str| int_val.parse::<i32>(),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::integer;

    #[test]
    fn parse_integer_operand() {
        {
            let (rest, reg) = integer("#10 ").unwrap();
            assert_eq!(reg, 10);
            assert_eq!(rest, "");
        }

        {
            let (_rest, reg) = integer("#400").unwrap();
            assert_eq!(reg, 400);
        }

        {
            assert!(integer("#asdf").is_err());
        }
    }
}
