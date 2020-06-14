pub enum BooleanOperator {
    Equal,
    NotEqual,

    And, // TODO: Might want to split And and Or in separate phases so as to give priority to one.
    Or,
}
