mod compiler_visitor;
mod root;
mod scope;

pub use root::Compiler;
use scope::Scope;

#[cfg(test)]
mod tests;
