use nom::{
    combinator::map,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::syntax::{
    common::whitespace,
    factor::{factor, Factor},
    operator::{factor_operator, FactorOperator},
};
use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub struct Term {
    pub root_factor: Factor,
    pub trail: Vec<(FactorOperator, Factor)>,
}

impl Visitable for Term {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_term(self)
    }
}

pub fn term(i: &str) -> IResult<&str, Term> {
    let t = delimited(
        whitespace,
        tuple((factor, many0(tuple((factor_operator, factor))))),
        whitespace,
    );
    map(t, |(root_fct, lst)| Term {
        root_factor: root_fct,
        trail: lst,
    })(i)
}

#[cfg(test)]
mod tests {

    use super::{term, Factor, FactorOperator, Term};
    use crate::syntax::operator::UnaryOperator;

    #[test]
    fn term_single_factor() {
        let (rest, t) = term(" -26 ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            t,
            Term {
                root_factor: Factor::Unary(UnaryOperator::Minus, Box::new(Factor::Integer(26))),
                trail: Vec::new()
            }
        )
    }

    #[test]
    fn term_multi_factor() {
        let (rest, t) = term(" -26  * 42  /  -3").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            t,
            Term {
                root_factor: Factor::Unary(UnaryOperator::Minus, Box::new(Factor::Integer(26))),
                trail: vec![
                    (FactorOperator::Mult, Factor::Integer(42)),
                    (
                        FactorOperator::Div,
                        Factor::Unary(UnaryOperator::Minus, Box::new(Factor::Integer(3)))
                    ),
                ]
            }
        )
    }
}
