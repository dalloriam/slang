use nom::{combinator::map, IResult};

use crate::syntax::{argument_list::argument_list, types::ArgumentList};

#[derive(Clone, Debug, PartialEq)]
pub enum Trailer {
    ArgumentList(ArgumentList),
}

pub fn trailer(i: &str) -> IResult<&str, Trailer> {
    map(argument_list, Trailer::ArgumentList)(i)
}
