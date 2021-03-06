use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    sequence::{delimited, preceded, terminated},
    IResult,
};

use crate::{
    syntax::{
        common::whitespace,
        expression::expression,
        if_expr::{if_expression, IfExpression},
        types::{Expression, VariableAssignment, VariableDeclaration},
        var_decl::{variable_assignment, variable_declaration},
    },
    visitor::{Visitable, Visitor},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    VarDecl(VariableDeclaration),
    VarAssign(VariableAssignment),
    Return(Option<Expression>),
    IfExpression(IfExpression),
    Expr(Expression),
}

impl Visitable for Statement {
    fn accept<V: Visitor>(&mut self, vis: &mut V) -> V::Result {
        vis.visit_statement(self)
    }
}

fn semicolon_statement(i: &str) -> IResult<&str, Statement> {
    delimited(
        whitespace,
        terminated(
            alt((
                map(preceded(tag("return"), opt(expression)), Statement::Return),
                map(variable_declaration, Statement::VarDecl),
                map(variable_assignment, Statement::VarAssign),
                map(expression, Statement::Expr),
            )),
            char(';'),
        ),
        whitespace,
    )(i)
}

pub fn block_statement(i: &str) -> IResult<&str, Statement> {
    delimited(
        whitespace,
        map(if_expression, Statement::IfExpression),
        whitespace,
    )(i)
}

pub fn statement(i: &str) -> IResult<&str, Statement> {
    alt((block_statement, semicolon_statement))(i)
}

#[cfg(test)]
mod tests {

    use super::statement;
    use crate::syntax::types::{
        Atom, AtomicExpression, Expression, Factor, Statement, Term, VariableDeclaration,
    };

    #[test]
    fn statement_decl_noassign() {
        let (rest, stmt) = statement("int i;").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            stmt,
            Statement::VarDecl(VariableDeclaration {
                name: String::from("i"),
                expression: None,
                var_type: String::from("int")
            })
        )
    }

    #[test]
    fn statement_decl_assign() {
        let (rest, stmt) = statement("int i = 3;").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            stmt,
            Statement::VarDecl(VariableDeclaration {
                name: String::from("i"),
                var_type: String::from("int"),
                expression: Some(Expression {
                    root_term: Term {
                        root_factor: Factor::Atomic(AtomicExpression {
                            atom: Atom::Integer(3),
                            trailers: Vec::new()
                        }),
                        trail: Vec::new(),
                    },
                    trail: Vec::new()
                })
            })
        );
    }

    #[test]
    fn bad_statement() {
        assert!(statement("asd askdjaks asd;").is_err());
    }

    #[test]
    fn statement_missing_semicolon() {
        assert!(statement("int a").is_err());
    }
}
