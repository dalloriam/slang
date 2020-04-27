use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::{delimited, preceded},
    IResult,
};

use crate::common::whitespace;

pub fn register(i: &str) -> IResult<&str, u8> {
    map_res(
        delimited(whitespace, preceded(char('$'), digit1), whitespace),
        |byte_val: &str| byte_val.parse::<u8>(),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::register;

    #[test]
    fn parse_register() {
        {
            let (rest, reg) = register("$18").unwrap();
            assert_eq!(reg, 18);
            assert_eq!(rest, "");
        }
        {
            let (_rest, reg) = register(" $18").unwrap();
            assert_eq!(reg, 18);
        }

        {
            assert!(register("$400").is_err());
        }
    }
}
