use crate::syntax::types::{FactorOperator, TermOperator, UnaryOperator};

pub trait Operator {
    fn defined_for(t: &str) -> bool;
}

impl Operator for FactorOperator {
    fn defined_for(t: &str) -> bool {
        t == "int"
    }
}

impl Operator for TermOperator {
    fn defined_for(t: &str) -> bool {
        t == "int"
    }
}

impl Operator for UnaryOperator {
    fn defined_for(t: &str) -> bool {
        t == "int"
    }
}
