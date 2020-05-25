use super::{VariableAssignment, VariableDeclaration};

pub enum Statement {
    VarDecl(VariableDeclaration),
    VarAssign(VariableAssignment),
}
