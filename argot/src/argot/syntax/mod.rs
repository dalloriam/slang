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
pub mod common;
pub mod factor;
pub mod number;
pub mod operator;
pub mod term;

pub use arithmetic_expression::ArithmeticExpression;
pub use factor::Factor;
pub use operator::{FactorOperator, TermOperator, UnaryOperator};
pub use term::Term;
