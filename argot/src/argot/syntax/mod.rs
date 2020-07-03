/*
Grammar.

ArithmeticExpression: Term ( (+ | -) term)*

Term: Factor ( (* | /) Factor )*

Factor:   UnaryOperator Factor
        | Integer
        | Float

TermOperator:   + | -
FactorOperator: * | /
UnaryOperator:  -
*/

pub mod argument_list;
pub mod atom;
pub mod atom_expr;
pub mod block;
pub mod call;
pub mod common;
pub mod expression;
pub mod factor;
pub mod function;
pub mod if_expr;
pub mod number;
pub mod operator;
pub mod program;
pub mod statement;
pub mod term;
pub mod trailer;
pub mod var_decl;

pub mod types {
    pub use super::argument_list::{Argument, ArgumentList};
    pub use super::atom::Atom;
    pub use super::atom_expr::AtomicExpression;
    pub use super::block::Block;
    pub use super::call::FunctionCall;
    pub use super::expression::Expression;
    pub use super::factor::Factor;
    pub use super::function::FunctionDeclaration;
    pub use super::if_expr::IfExpression;
    pub use super::operator::{FactorOperator, TermOperator, UnaryOperator};
    pub use super::program::Program;
    pub use super::statement::Statement;
    pub use super::term::Term;
    pub use super::trailer::Trailer;
    pub use super::var_decl::{VariableAssignment, VariableDeclaration};
}

pub use atom::Atom;
pub use block::Block;
pub use factor::Factor;
pub use function::FunctionDeclaration;
pub use operator::{FactorOperator, TermOperator, UnaryOperator};
pub use program::Program;
pub use statement::Statement;
pub use term::Term;
pub use var_decl::{VariableAssignment, VariableDeclaration};
