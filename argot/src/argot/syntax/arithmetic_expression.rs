use nom::{
    combinator::map,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::syntax::{
    common::whitespace,
    operator::{term_operator, TermOperator},
    term::{term, Term},
};
use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub struct ArithmeticExpression {
    pub root_term: Term,
    pub trail: Vec<(TermOperator, Term)>,
}

impl Visitable for ArithmeticExpression {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_arithmetic_expression(self)
    }
}

pub fn arithmetic_expression(i: &str) -> IResult<&str, ArithmeticExpression> {
    let t = delimited(
        whitespace,
        tuple((term, many0(tuple((term_operator, term))))),
        whitespace,
    );
    map(t, |(root_trm, lst)| ArithmeticExpression {
        root_term: root_trm,
        trail: lst,
    })(i)
}

#[cfg(test)]
mod tests {

    use super::{arithmetic_expression, ArithmeticExpression, Term, TermOperator};
    use crate::syntax::{Atom, Expression, Factor, FactorOperator, UnaryOperator};

    #[test]
    fn arithm_expression() {
        let expected_expression = ArithmeticExpression {
            root_term: Term {
                root_factor: Factor::Atom(Atom::Integer(18)),
                trail: Vec::new(),
            },
            trail: vec![(
                TermOperator::Plus,
                Term {
                    root_factor: Factor::Atom(Atom::Integer(15)),
                    trail: vec![(
                        FactorOperator::Mult,
                        Factor::Unary(
                            UnaryOperator::Minus,
                            Box::new(Factor::Atom(Atom::Integer(4))),
                        ),
                    )],
                },
            )],
        };

        let (rest, expr) = arithmetic_expression("18 + 15 * -4").unwrap();
        assert_eq!(rest, "");
        assert_eq!(expr, expected_expression);
    }

    #[test]
    fn nested_expression() {
        let expected_expression = ArithmeticExpression {
            root_term: Term {
                root_factor: Factor::Expression(Box::new(Expression::Arithmetic(
                    ArithmeticExpression {
                        root_term: Term {
                            root_factor: Factor::Atom(Atom::Integer(2)),
                            trail: vec![(FactorOperator::Mult, Factor::Atom(Atom::Integer(3)))],
                        },
                        trail: Vec::new(),
                    },
                ))),
                trail: Vec::new(),
            },
            trail: vec![(
                TermOperator::Plus,
                Term {
                    root_factor: Factor::Atom(Atom::Integer(4)),
                    trail: Vec::new(),
                },
            )],
        };

        let (rest, expr) = arithmetic_expression("(2 * 3) + 4").unwrap();
        assert_eq!(rest, "");
        assert_eq!(expr, expected_expression);
    }
}
