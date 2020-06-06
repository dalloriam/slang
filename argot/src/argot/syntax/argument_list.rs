use nom::{character::complete::char, combinator::map, sequence::delimited, IResult};

use crate::syntax::common::whitespace;

#[derive(Clone, Debug, PartialEq)]
pub struct ArgumentList {}

pub fn argument_list(i: &str) -> IResult<&str, ArgumentList> {
    map(delimited(char('('), whitespace, char(')')), |_| {
        ArgumentList {}
    })(i)
}
