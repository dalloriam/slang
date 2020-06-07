use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::syntax::{
    block::{block, Block},
    common::whitespace,
    expression::{expression, Expression},
};

#[derive(Clone, Debug, PartialEq)]
pub struct IfExpression {
    condition: Expression,
    if_block: Block,
    else_block: Option<Block>,
}

pub fn if_expression(i: &str) -> IResult<&str, IfExpression> {
    map(
        tuple((
            preceded(
                tag("if"),
                delimited(
                    whitespace,
                    delimited(char('('), expression, char(')')),
                    whitespace,
                ),
            ),
            block,
            opt(preceded(tag("else"), block)),
        )),
        |(condition, if_block, else_block)| IfExpression {
            condition,
            if_block,
            else_block,
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{if_expression, IfExpression};
    use crate::syntax::types::{
        Atom, AtomicExpression, Block, Expression, Factor, Statement, Term, VariableDeclaration,
    };

    #[test]
    fn if_empty_condition() {
        assert!(if_expression("if () {}").is_err());
    }

    #[test]
    fn if_empty_body() {
        let (rest, if_expr) = if_expression("if (18) {}").unwrap();
        assert_eq!(rest, "");

        assert_eq!(
            if_expr,
            IfExpression {
                condition: Expression {
                    root_term: Term {
                        root_factor: Factor::Atomic(AtomicExpression {
                            atom: Atom::Integer(18),
                            trailers: Vec::new()
                        }),
                        trail: Vec::new()
                    },
                    trail: Vec::new()
                },
                if_block: Block::new(),
                else_block: None,
            }
        )
    }

    #[test]
    fn if_block_body() {
        let (rest, if_expr) = if_expression("if (18) {int a = 3;}").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            if_expr,
            IfExpression {
                condition: Expression {
                    root_term: Term {
                        root_factor: Factor::Atomic(AtomicExpression {
                            atom: Atom::Integer(18),
                            trailers: Vec::new()
                        }),
                        trail: Vec::new()
                    },
                    trail: Vec::new()
                },
                if_block: Block {
                    body: vec![Statement::VarDecl(VariableDeclaration {
                        var_type: String::from("int"),
                        name: String::from("a"),
                        expression: Some(Expression {
                            root_term: Term {
                                root_factor: Factor::Atomic(AtomicExpression {
                                    atom: Atom::Integer(3),
                                    trailers: Vec::new()
                                }),
                                trail: Vec::new()
                            },
                            trail: Vec::new()
                        })
                    })]
                },
                else_block: None,
            }
        )
    }

    #[test]
    fn if_else_empty_else() {
        let (rest, if_expr) = if_expression(
            "if (18) {int a = 3;}
        else {}",
        )
        .unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            if_expr,
            IfExpression {
                condition: Expression {
                    root_term: Term {
                        root_factor: Factor::Atomic(AtomicExpression {
                            atom: Atom::Integer(18),
                            trailers: Vec::new()
                        }),
                        trail: Vec::new()
                    },
                    trail: Vec::new()
                },
                if_block: Block {
                    body: vec![Statement::VarDecl(VariableDeclaration {
                        var_type: String::from("int"),
                        name: String::from("a"),
                        expression: Some(Expression {
                            root_term: Term {
                                root_factor: Factor::Atomic(AtomicExpression {
                                    atom: Atom::Integer(3),
                                    trailers: Vec::new()
                                }),
                                trail: Vec::new()
                            },
                            trail: Vec::new()
                        })
                    })]
                },
                else_block: Some(Block::new()),
            }
        )
    }

    #[test]
    fn if_else_else_body() {
        let (rest, if_expr) = if_expression(
            "if (18) {int a = 3;}
        else {int a = 5; }",
        )
        .unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            if_expr,
            IfExpression {
                condition: Expression {
                    root_term: Term {
                        root_factor: Factor::Atomic(AtomicExpression {
                            atom: Atom::Integer(18),
                            trailers: Vec::new()
                        }),
                        trail: Vec::new()
                    },
                    trail: Vec::new()
                },
                if_block: Block {
                    body: vec![Statement::VarDecl(VariableDeclaration {
                        var_type: String::from("int"),
                        name: String::from("a"),
                        expression: Some(Expression {
                            root_term: Term {
                                root_factor: Factor::Atomic(AtomicExpression {
                                    atom: Atom::Integer(3),
                                    trailers: Vec::new()
                                }),
                                trail: Vec::new()
                            },
                            trail: Vec::new()
                        })
                    })]
                },
                else_block: Some(Block {
                    body: vec![Statement::VarDecl(VariableDeclaration {
                        var_type: String::from("int"),
                        name: String::from("a"),
                        expression: Some(Expression {
                            root_term: Term {
                                root_factor: Factor::Atomic(AtomicExpression {
                                    atom: Atom::Integer(5),
                                    trailers: Vec::new()
                                }),
                                trail: Vec::new()
                            },
                            trail: Vec::new()
                        })
                    })]
                }),
            }
        )
    }
}
