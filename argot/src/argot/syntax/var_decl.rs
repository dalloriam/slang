use super::ArithmeticExpression;

pub struct VariableDeclaration {
    pub name: String,
    pub expression: Option<ArithmeticExpression>,
}

pub struct VariableAssignment {
    pub name: String,
    pub expression: ArithmeticExpression,
}
