use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use crate::syntax::{
    arithmetic_expression::{arithmetic_expression, ArithmeticExpression},
    common::whitespace,
    number::integer,
    operator::{unary_operator, UnaryOperator},
};
use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub enum Factor {
    Integer(i32),
    Unary(UnaryOperator, Box<Factor>),
    Expression(Box<ArithmeticExpression>),
}

impl Visitable for Factor {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_factor(self)
    }
}

pub fn factor(i: &str) -> IResult<&str, Factor> {
    delimited(
        whitespace,
        alt((unary_factor, int_factor, expr_factor)),
        whitespace,
    )(i)
}

fn int_factor(i: &str) -> IResult<&str, Factor> {
    map(integer, Factor::Integer)(i)
}

fn unary_factor(i: &str) -> IResult<&str, Factor> {
    map(tuple((unary_operator, factor)), |(op, sub)| {
        Factor::Unary(op, Box::new(sub))
    })(i)
}

fn expr_factor(i: &str) -> IResult<&str, Factor> {
    map(
        delimited(
            whitespace,
            delimited(char('('), arithmetic_expression, char(')')),
            whitespace,
        ),
        |e| Factor::Expression(Box::new(e)),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{factor, Factor, UnaryOperator};

    #[test]
    fn test_int_factor() {
        let (rest, f) = factor(" 48").unwrap();
        assert_eq!(rest, "");
        assert_eq!(f, Factor::Integer(48));
    }

    #[test]
    fn test_unary_factor() {
        let (rest, f) = factor("-42").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            f,
            Factor::Unary(UnaryOperator::Minus, Box::new(Factor::Integer(42)))
        );
    }
}
