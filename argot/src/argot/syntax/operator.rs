use nom::{branch::alt, character::complete::char, combinator::map, sequence::delimited, IResult};

use crate::syntax::common::whitespace;
use crate::visitor::{Visitable, Visitor};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,

    Unknown,
}

impl Visitable for UnaryOperator {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_unary_operator(self)
    }
}

impl From<String> for UnaryOperator {
    fn from(c: String) -> UnaryOperator {
        match c.as_ref() {
            "+" => UnaryOperator::Plus,
            "-" => UnaryOperator::Minus,
            _ => UnaryOperator::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TermOperator {
    Plus,
    Minus,

    Unknown,
}

impl Visitable for TermOperator {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_term_operator(self)
    }
}

impl From<String> for TermOperator {
    fn from(c: String) -> TermOperator {
        match c.as_ref() {
            "+" => TermOperator::Plus,
            "-" => TermOperator::Minus,
            _ => TermOperator::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FactorOperator {
    Mult,
    Div,

    Unknown, // TODO: Get more detail.
}

impl Visitable for FactorOperator {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_factor_operator(self)
    }
}

impl From<String> for FactorOperator {
    fn from(c: String) -> FactorOperator {
        match c.as_ref() {
            "*" => FactorOperator::Mult,
            "/" => FactorOperator::Div,
            _ => FactorOperator::Unknown,
        }
    }
}

pub fn unary_operator(i: &str) -> IResult<&str, UnaryOperator> {
    map(
        delimited(whitespace, alt((char('+'), char('-'))), whitespace),
        |c| UnaryOperator::from(c.to_string()),
    )(i)
}

pub fn term_operator(i: &str) -> IResult<&str, TermOperator> {
    map(
        delimited(whitespace, alt((char('+'), char('-'))), whitespace),
        |c| TermOperator::from(c.to_string()),
    )(i)
}

pub fn factor_operator(i: &str) -> IResult<&str, FactorOperator> {
    map(
        delimited(whitespace, alt((char('*'), char('/'))), whitespace),
        |c| FactorOperator::from(c.to_string()),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{
        factor_operator, term_operator, unary_operator, FactorOperator, TermOperator, UnaryOperator,
    };

    #[test]
    fn unary_op_valid() {
        let (rest, op) = unary_operator("  - ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(op, UnaryOperator::Minus);
    }

    #[test]
    fn unary_op_invalid() {
        assert!(unary_operator("/").is_err());
    }

    #[test]
    fn term_op_plus() {
        let (rest, op) = term_operator("  + ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(op, TermOperator::Plus);
    }

    #[test]
    fn term_op_minus() {
        let (rest, op) = term_operator("  - ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(op, TermOperator::Minus);
    }

    #[test]
    fn factor_op_mult() {
        let (rest, op) = factor_operator("  * ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(op, FactorOperator::Mult);
    }

    #[test]
    fn factor_op_div() {
        let (rest, op) = factor_operator("  / ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(op, FactorOperator::Div);
    }

    #[test]
    fn term_op_invalid() {
        assert!(term_operator("  ? ").is_err());
    }

    #[test]
    fn factor_op_invalid() {
        assert!(factor_operator("  ? ").is_err());
    }
}