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

pub mod arithmetic_expression;
pub mod atom;
pub mod block;
pub mod common;
pub mod expression;
pub mod factor;
pub mod function;
pub mod number;
pub mod operator;
pub mod program;
pub mod statement;
pub mod term;
pub mod var_decl;

pub use arithmetic_expression::ArithmeticExpression;
pub use atom::Atom;
pub use block::Block;
pub use expression::Expression;
pub use factor::Factor;
pub use function::FunctionDeclaration;
pub use operator::{FactorOperator, TermOperator, UnaryOperator};
pub use program::Program;
pub use statement::Statement;
pub use term::Term;
pub use var_decl::{VariableAssignment, VariableDeclaration};
