use nom::{
    character::complete::{alpha1, char},
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::syntax::{common::whitespace, expression::expression, Expression};

use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub struct VariableDeclaration {
    pub var_type: String,
    pub name: String,
    pub expression: Option<Expression>,
}

impl Visitable for VariableDeclaration {
    fn accept<V: Visitor>(&mut self, v: &mut V) -> V::Result {
        v.visit_variable_declaration(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableAssignment {
    pub name: String,
    pub expression: Expression,
}

fn assign(i: &str) -> IResult<&str, Expression> {
    preceded(delimited(whitespace, char('='), whitespace), expression)(i)
}

pub fn identifier(i: &str) -> IResult<&str, &str> {
    delimited(whitespace, alpha1, whitespace)(i)
}

pub fn variable_declaration(i: &str) -> IResult<&str, VariableDeclaration> {
    map(
        tuple((identifier, identifier, opt(assign))),
        |(var_type, name, ass)| VariableDeclaration {
            var_type: String::from(var_type),
            name: String::from(name),
            expression: ass,
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::variable_declaration;
    use crate::syntax::{
        ArithmeticExpression, Atom, Expression, Factor, Term, VariableDeclaration,
    };

    #[test]
    fn int_decl() {
        let (rest, decl) = variable_declaration("int a ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            decl,
            VariableDeclaration {
                var_type: String::from("int"),
                name: String::from("a"),
                expression: None
            }
        );
    }

    #[test]
    fn int_decl_assign() {
        let (rest, decl) = variable_declaration("int bing = 14").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            decl,
            VariableDeclaration {
                var_type: String::from("int"),
                name: String::from("bing"),
                expression: Some(Expression::Arithmetic(ArithmeticExpression {
                    root_term: Term {
                        root_factor: Factor::Atom(Atom::Integer(14)),
                        trail: Vec::new()
                    },
                    trail: Vec::new(),
                }))
            }
        )
    }
}
