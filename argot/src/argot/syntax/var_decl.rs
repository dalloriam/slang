use nom::{
    character::complete::{alpha1, char},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::syntax::{arithmetic_expression::arithmetic_expression, common::whitespace};

use super::ArithmeticExpression;
use nom::sequence::delimitedc;

#[derive(Clone, Debug, PartialEq)]
pub struct VariableDeclaration {
    pub var_type: String,
    pub name: String,
    pub expression: Option<ArithmeticExpression>,
}

pub struct VariableAssignment {
    pub name: String,
    pub expression: ArithmeticExpression,
}

fn assign(i: &str) -> IResult<&str, ArithmeticExpression> {
    preceded(
        delimited(whitespace, char('='), whitespace),
        arithmetic_expression,
    )(i)
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
    use super::{variable_declaration, ArithmeticExpression, VariableDeclaration};
    use crate::syntax::{Factor, Term};

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
                expression: Some(ArithmeticExpression {
                    root_term: Term {
                        root_factor: Factor::Integer(14),
                        trail: Vec::new()
                    },
                    trail: Vec::new(),
                })
            }
        )
    }
}
