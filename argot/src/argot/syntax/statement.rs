use nom::{
    character::complete::char,
    combinator::map,
    sequence::{delimited, terminated},
    IResult,
};

use crate::{
    syntax::{
        common::whitespace, var_decl::variable_declaration, VariableAssignment, VariableDeclaration,
    },
    visitor::{Visitable, Visitor},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    VarDecl(VariableDeclaration),
    VarAssign(VariableAssignment),
}

impl Visitable for Statement {
    fn accept<V: Visitor>(&mut self, vis: &mut V) -> V::Result {
        vis.visit_statement(self)
    }
}

pub fn statement(i: &str) -> IResult<&str, Statement> {
    map(
        delimited(
            whitespace,
            terminated(variable_declaration, char(';')),
            whitespace,
        ),
        |decl| Statement::VarDecl(decl),
    )(i)
}

#[cfg(test)]
mod tests {

    use super::statement;
    use crate::syntax::{Statement, VariableDeclaration};

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
    fn statement_decl_assign() {}

    #[test]
    fn bad_statement() {}

    #[test]
    fn statement_missing_semicolon() {}
}
