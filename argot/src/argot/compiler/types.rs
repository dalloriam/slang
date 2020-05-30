use std::convert::TryFrom;
use std::fmt;
use std::mem;

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

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "int" => Ok(BuiltInType::Integer),
            "bool" => Ok(BuiltInType::Boolean),
            _ => Err(UnknownType {
                type_name: String::from(value),
            }),
        }
    }
}
