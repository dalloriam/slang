use nom::{bytes::complete::tag, combinator::map, sequence::terminated, IResult};

use crate::syntax::var_decl::identifier;
use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub name: String,
}

impl Visitable for FunctionCall {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_function_call(self)
    }
}

pub fn function_call(i: &str) -> IResult<&str, FunctionCall> {
    map(terminated(identifier, tag("()")), |fn_name| FunctionCall {
        name: String::from(fn_name),
    })(i)
}
