use nom::{
    character::complete::{alphanumeric1, char},
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use crate::common::whitespace;

pub fn label_declaration(i: &str) -> IResult<&str, String> {
    map(
        delimited(
            whitespace,
            tuple((alphanumeric1, whitespace, char(':'))),
            whitespace,
        ),
        |(name, _ws, _ch)| String::from(name),
    )(i)
}

pub fn label_usage(i: &str) -> IResult<&str, String> {
    map(
        delimited(
            whitespace,
            tuple((char('@'), whitespace, alphanumeric1)),
            whitespace,
        ),
        |(_ch, _ws, name)| String::from(name),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{label_declaration, label_usage};

    #[test]
    fn parse_label_declaration() {
        let (rest, label) = label_declaration(" test: ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(label, "test");
    }

    #[test]
    fn parse_label_usage() {
        let (rest, label) = label_usage("@test ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(label, "test");
    }
}
