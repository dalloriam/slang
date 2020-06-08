use nom::{character::complete::char, combinator::map, multi::many0, sequence::delimited, IResult};

use crate::{
    syntax::{
        common::whitespace,
        statement::{statement, Statement},
    },
    visitor::{Visitable, Visitor},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub body: Vec<Statement>,
}

impl Block {
    pub fn new() -> Block {
        Block { body: Vec::new() }
    }
}

impl Default for Block {
    fn default() -> Block {
        Block::new()
    }
}

impl Visitable for Block {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_block(self)
    }
}

pub fn block(i: &str) -> IResult<&str, Block> {
    map(
        delimited(
            whitespace,
            delimited(
                char('{'),
                delimited(whitespace, many0(statement), whitespace),
                char('}'),
            ),
            whitespace,
        ),
        |v| Block { body: v },
    )(i)
}
