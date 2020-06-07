use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::syntax::{number::integer, var_decl::identifier};

use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Boolean(bool),
    Identifier(String),
    Integer(i32),
}

impl Visitable for Atom {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_atom(self)
    }
}

pub fn atom(i: &str) -> IResult<&str, Atom> {
    alt((bool_atom, identifier_atom, int_atom))(i)
}

fn bool_atom(i: &str) -> IResult<&str, Atom> {
    alt((
        map(tag("true"), |_| Atom::Boolean(true)),
        map(tag("false"), |_| Atom::Boolean(false)),
    ))(i)
}

fn identifier_atom(i: &str) -> IResult<&str, Atom> {
    map(identifier, |id_str| Atom::Identifier(String::from(id_str)))(i)
}

fn int_atom(i: &str) -> IResult<&str, Atom> {
    map(integer, Atom::Integer)(i)
}
