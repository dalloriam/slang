use crate::syntax::{
    ArithmeticExpression, Expression, Factor, FactorOperator, FunctionDeclaration, Program,
    Statement, Term, TermOperator, UnaryOperator, VariableDeclaration,
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
    fn visit_function_declaration(&mut self, v: &mut FunctionDeclaration) -> Self::Result;
    fn visit_statement(&mut self, v: &mut Statement) -> Self::Result;
    fn visit_variable_declaration(&mut self, v: &mut VariableDeclaration) -> Self::Result;
    fn visit_expression(&mut self, v: &mut Expression) -> Self::Result;
    fn visit_program(&mut self, v: &mut Program) -> Self::Result;
}
