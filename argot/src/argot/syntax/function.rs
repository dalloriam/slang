use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    syntax::{
        block::{block, Block},
        common::whitespace,
    },
    visitor::{Visitable, Visitor},
};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub return_type: String,
    pub name: String,
    pub block: Block,
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
            block,
        )),
        |(_f, name, _x, block)| FunctionDeclaration {
            return_type: String::from("int"),
            name: String::from(name),
            block,
        },
    )(i)
}

#[cfg(test)]
mod tests {

    use super::function_declaration;

    use crate::syntax::{Block, FunctionDeclaration, Statement, VariableDeclaration};

    #[test]
    fn fn_decl_no_return_type_no_arg_no_body() {
        let (rest, decl) = function_declaration("fn hello() {}").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            decl,
            FunctionDeclaration {
                return_type: String::from("int"),
                name: String::from("hello"),
                block: Block::new(),
            }
        )
    }

    #[test]
    fn fn_decl_no_return_type_no_arg() {
        let (rest, decl) = function_declaration("fn hello() { int a; }").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            decl,
            FunctionDeclaration {
                return_type: String::from("int"),
                name: String::from("hello"),
                block: Block {
                    body: vec![Statement::VarDecl(VariableDeclaration {
                        name: String::from("a"),
                        var_type: String::from("int"),
                        expression: None
                    })]
                }
            }
        )
    }
}
