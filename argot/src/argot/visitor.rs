use crate::syntax::{
    ArithmeticExpression, Factor, FactorOperator, Term, TermOperator, UnaryOperator,
};

pub trait Visitable {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result;
}

pub trait Visitor {
    type Result;

    fn visit_arithmetic_expression(&mut self, v: &mut ArithmeticExpression) -> Self::Result;
    fn visit_factor(&mut self, v: &mut Factor) -> Self::Result;
    fn visit_factor_operator(&mut self, v: &mut FactorOperator) -> Self::Result;
    fn visit_term(&mut self, v: &mut Term) -> Self::Result;
    fn visit_term_operator(&mut self, v: &mut TermOperator) -> Self::Result;
    fn visit_unary_operator(&mut self, v: &mut UnaryOperator) -> Self::Result;
}
