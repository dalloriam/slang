use crate::syntax::types::*;

pub trait Visitable {
    fn accept<V: Visitor>(&mut self, visitor: &mut V) -> V::Result;
}

pub trait Visitor {
    type Result;

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
    fn visit_atomic_expression(&mut self, v: &mut AtomicExpression) -> Self::Result;
    fn visit_atom(&mut self, v: &mut Atom) -> Self::Result;
    fn visit_block(&mut self, v: &mut Block) -> Self::Result;
    fn visit_variable_assignment(&mut self, v: &mut VariableAssignment) -> Self::Result;
    fn visit_function_call(&mut self, v: &mut FunctionCall) -> Self::Result;
}
