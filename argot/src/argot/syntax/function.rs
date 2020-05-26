use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char},
    combinator::map,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use super::Statement;
use crate::{
    syntax::{common::whitespace, statement::statement},
    visitor::{Visitable, Visitor},
};

pub struct FunctionDeclaration {
    pub return_type: String,
    pub name: String,
    pub body: Vec<Statement>,
}

impl Visitable for FunctionDeclaration {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_function_declaration(self)
    }
}

pub fn function_declaration(i: &str) -> IResult<&str, FunctionDeclaration> {
    map(
        tuple((
            tag("fn"),
            delimited(whitespace, alpha1, whitespace),
            tag("()"),
            function_body,
        )),
        |(_f, name, _x, body)| FunctionDeclaration {
            return_type: String::from("int"),
            name: String::from(name),
            body,
        },
    )(i)
}

fn function_body(i: &str) -> IResult<&str, Vec<Statement>> {
    delimited(
        whitespace,
        delimited(char('{'), many0(statement), char('}')),
        whitespace,
    )(i)
}
