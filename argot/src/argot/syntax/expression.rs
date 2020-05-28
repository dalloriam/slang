use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::tuple, IResult};

use crate::syntax::{
    arithmetic_expression::arithmetic_expression, var_decl::identifier, ArithmeticExpression,
};
use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Arithmetic(ArithmeticExpression),
    Identifier(String),
    FunctionCall(String),
}

impl Visitable for Expression {
    fn accept<V: Visitor>(&mut self, v: &mut V) -> V::Result {
        v.visit_expression(self)
    }
}

pub fn expression(i: &str) -> IResult<&str, Expression> {
    alt((
        map(tuple((identifier, tag("()"))), |(id, _tag)| {
            Expression::FunctionCall(String::from(id))
        }),
        map(arithmetic_expression, |expr| Expression::Arithmetic(expr)),
        map(identifier, |ident| {
            Expression::Identifier(String::from(ident))
        }),
    ))(i)
}
