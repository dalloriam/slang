mod emit;
mod error;
mod first_pass;
mod operator;
mod root;
mod scope;
mod second_pass;
mod typing;

pub use error::CompileError;
pub use root::Compiler;
use scope::Scope;
