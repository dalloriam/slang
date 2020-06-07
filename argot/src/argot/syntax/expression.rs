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
pub struct Expression {
    pub root_term: Term,
    pub trail: Vec<(TermOperator, Term)>,
}

impl Visitable for Expression {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(self)
    }
}

pub fn expression(i: &str) -> IResult<&str, Expression> {
    let t = delimited(
        whitespace,
        tuple((term, many0(tuple((term_operator, term))))),
        whitespace,
    );
    map(t, |(root_trm, lst)| Expression {
        root_term: root_trm,
        trail: lst,
    })(i)
}

#[cfg(test)]
mod tests {

    use super::{expression, Expression, Term, TermOperator};
    use crate::syntax::types::{Atom, AtomicExpression, Factor, FactorOperator, UnaryOperator};

    #[test]
    fn arithm_expression() {
        let expected_expression = Expression {
            root_term: Term {
                root_factor: Factor::Atomic(AtomicExpression {
                    atom: Atom::Integer(18),
                    trailers: Vec::new(),
                }),
                trail: Vec::new(),
            },
            trail: vec![(
                TermOperator::Plus,
                Term {
                    root_factor: Factor::Atomic(AtomicExpression {
                        atom: Atom::Integer(15),
                        trailers: Vec::new(),
                    }),
                    trail: vec![(
                        FactorOperator::Mult,
                        Factor::Unary(
                            UnaryOperator::Minus,
                            Box::new(Factor::Atomic(AtomicExpression {
                                atom: Atom::Integer(4),
                                trailers: Vec::new(),
                            })),
                        ),
                    )],
                },
            )],
        };

        let (rest, expr) = expression("18 + 15 * -4").unwrap();
        assert_eq!(rest, "");
        assert_eq!(expr, expected_expression);
    }

    #[test]
    fn nested_expression() {
        let expected_expression = Expression {
            root_term: Term {
                root_factor: Factor::Expression(Box::new(Expression {
                    root_term: Term {
                        root_factor: Factor::Atomic(AtomicExpression {
                            atom: Atom::Integer(2),
                            trailers: Vec::new(),
                        }),
                        trail: vec![(
                            FactorOperator::Mult,
                            Factor::Atomic(AtomicExpression {
                                atom: Atom::Integer(3),
                                trailers: Vec::new(),
                            }),
                        )],
                    },
                    trail: Vec::new(),
                })),
                trail: Vec::new(),
            },
            trail: vec![(
                TermOperator::Plus,
                Term {
                    root_factor: Factor::Atomic(AtomicExpression {
                        atom: Atom::Integer(4),
                        trailers: Vec::new(),
                    }),
                    trail: Vec::new(),
                },
            )],
        };

        let (rest, expr) = expression("(2 * 3) + 4").unwrap();
        assert_eq!(rest, "");
        assert_eq!(expr, expected_expression);
    }
}
