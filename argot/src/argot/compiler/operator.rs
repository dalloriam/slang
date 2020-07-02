use crate::syntax::types::{FactorOperator, TermOperator, UnaryOperator};

pub trait Operator {
    fn defined_for(&self, t: &str) -> bool;
}

impl Operator for FactorOperator {
    fn defined_for(&self, t: &str) -> bool {
        t == "int"
    }
}

impl Operator for TermOperator {
    fn defined_for(&self, t: &str) -> bool {
        t == "int"
    }
}

impl Operator for UnaryOperator {
    fn defined_for(&self, t: &str) -> bool {
        match self {
            UnaryOperator::Plus | UnaryOperator::Minus => t == "int",
            UnaryOperator::Not => t == "bool",
            UnaryOperator::Unknown => false,
        }
    }
}
