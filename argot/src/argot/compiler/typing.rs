use std::convert::TryFrom;
use std::fmt;
use std::mem;

use snafu::ensure;

use crate::compiler::{error::*, operator::Operator};

#[derive(Debug)]
pub struct UnknownType {
    type_name: String,
}

impl fmt::Display for UnknownType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown type: {}", self.type_name)
    }
}

impl std::error::Error for UnknownType {}

pub enum BuiltInType {
    Integer,
    Boolean,
}

impl BuiltInType {
    pub fn alloc_size(&self) -> usize {
        match self {
            BuiltInType::Integer => mem::size_of::<i32>(),
            BuiltInType::Boolean => 1,
        }
    }
}

impl TryFrom<String> for BuiltInType {
    type Error = UnknownType;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_ref() {
            "int" => Ok(BuiltInType::Integer),
            "bool" => Ok(BuiltInType::Boolean),
            _ => Err(UnknownType { type_name: value }),
        }
    }
}

pub fn typecheck_unary_operator<T: Operator>(t: &str) -> Result<()> {
    ensure!(T::defined_for(t), InvalidOperator { t });
    Ok(())
}

pub fn typecheck_binary_operator<T: Operator>(t1: &str, t2: &str) -> Result<()> {
    ensure!(t1 == t2, TypeMismatch { t1, t2 });
    ensure!(T::defined_for(t1), InvalidOperator { t: t1 });
    Ok(())
}