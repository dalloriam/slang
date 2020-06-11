use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::delimited, IResult};

use crate::syntax::{common::whitespace, number::integer, var_decl::identifier};
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
    delimited(
        whitespace,
        alt((
            map(tag("true"), |_| Atom::Boolean(true)),
            map(tag("false"), |_| Atom::Boolean(false)),
        )),
        whitespace,
    )(i)
}

fn identifier_atom(i: &str) -> IResult<&str, Atom> {
    map(identifier, |id_str| Atom::Identifier(String::from(id_str)))(i)
}

fn int_atom(i: &str) -> IResult<&str, Atom> {
    map(integer, Atom::Integer)(i)
}

#[cfg(test)]
mod tests {
    use super::{atom, Atom};

    #[test]
    fn bool_atom() {
        let (rest, atm) = atom("  true").unwrap();
        assert_eq!(rest, "");
        assert_eq!(atm, Atom::Boolean(true));
    }

    #[test]
    fn identifier_atom() {
        let (rest, atm) = atom(" bing  ").unwrap();
        assert_eq!(rest, "");
        assert_eq!(atm, Atom::Identifier(String::from("bing")));
    }

    #[test]
    fn integer_atom() {
        let (rest, atm) = atom("    83712").unwrap();
        assert_eq!(rest, "");
        assert_eq!(atm, Atom::Integer(83712));
    }
}
