use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use crate::syntax::{
    atom::{atom, Atom},
    common::whitespace,
    expression::{expression, Expression},
    operator::{unary_operator, UnaryOperator},
};
use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub enum Factor {
    Atom(Atom),
    Unary(UnaryOperator, Box<Factor>),
    Expression(Box<Expression>),
}

impl Visitable for Factor {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_factor(self)
    }
}

pub fn factor(i: &str) -> IResult<&str, Factor> {
    delimited(
        whitespace,
        alt((unary_factor, expr_factor, atom_factor)),
        whitespace,
    )(i)
}

fn atom_factor(i: &str) -> IResult<&str, Factor> {
    map(atom, Factor::Atom)(i)
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
            delimited(char('('), expression, char(')')),
            whitespace,
        ),
        |e| Factor::Expression(Box::new(e)),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{factor, Atom, Factor, UnaryOperator};

    #[test]
    fn test_atom_factor() {
        let (rest, f) = factor(" 48").unwrap();
        assert_eq!(rest, "");
        assert_eq!(f, Factor::Atom(Atom::Integer(48)));
    }

    #[test]
    fn test_unary_factor() {
        let (rest, f) = factor("-42").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            f,
            Factor::Unary(
                UnaryOperator::Minus,
                Box::new(Factor::Atom(Atom::Integer(42)))
            )
        );
    }
}
