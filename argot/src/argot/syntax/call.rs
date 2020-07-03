use nom::{
    character::complete::char,
    combinator::map,
    multi::separated_list,
    sequence::{delimited, tuple},
    IResult,
};

use crate::syntax::{expression::expression, types::Expression, var_decl::identifier};
use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

impl Visitable for FunctionCall {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_function_call(self)
    }
}

pub fn function_call(i: &str) -> IResult<&str, FunctionCall> {
    map(
        tuple((
            identifier,
            delimited(char('('), separated_list(char(','), expression), char(')')),
        )),
        |(fn_name, args)| FunctionCall {
            name: String::from(fn_name),
            arguments: args,
        },
    )(i)
}
