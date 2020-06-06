use nom::{combinator::map, multi::many0, sequence::tuple, IResult};

use crate::syntax::types::{Atom, Trailer};
use crate::syntax::{atom::atom, trailer::trailer};
use crate::visitor::{Visitable, Visitor};

#[derive(Clone, Debug, PartialEq)]
pub struct AtomicExpression {
    pub atom: Atom,
    pub trailers: Vec<Trailer>,
}

impl Visitable for AtomicExpression {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit_atomic_expression(self)
    }
}

pub fn atomic_expression(i: &str) -> IResult<&str, AtomicExpression> {
    map(tuple((atom, many0(trailer))), |(atom, trailers)| {
        AtomicExpression { atom, trailers }
    })(i)
}
