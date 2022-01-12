use std::fmt;

use assembler::AssemblerError;

use snafu::Snafu;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum CompileError {
    AssemblyError {
        source: AssemblerError,
    },
    DuplicateFunction {
        name: String,
    },
    IncompleteParse {
        source: ParseError,
    },
    InvalidArguments, // TODO: Details
    InvalidOperator {
        t: String,
    },
    InvalidRegisterState,
    MissingEntryPoint,
    MissingScope,
    MissingType,
    NotAllPathsReturnAValue,
    NoUsedRegisters,
    TypeMismatch {
        t1: String,
        t2: String,
    },
    UnknownFunction {
        name: String,
    },
    UnknownIdentifier {
        name: String,
    },
    UnknownType {
        name: String,
        source: super::typing::UnknownType,
    },
    VariableAlreadyDefined {
        name: String,
    },
}

pub type Result<T> = std::result::Result<T, CompileError>;
