use super::Statement;

pub struct Function {
    return_type: String,
    name: String,
    body: Vec<Statement>,
}
