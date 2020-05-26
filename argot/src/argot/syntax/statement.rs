use nom::{
    branch::alt,
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
