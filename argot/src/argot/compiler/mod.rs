mod compiler_visitor;
mod root;
mod scope;
mod types;

pub use root::{CompileError, Compiler};
use scope::Scope;
