/// The parser chapter guides me to write the code in OOP,
/// but it is easier to write and understand the code in Rust's ways,
/// enumerating the data structure and matching expression.

/// Adhering to the parser chapter requires a token field.
/// Still, I ignored creating it because Rust's enum can represent the Let statement,
/// Return statement, or Expression statement, even without the token field.
#[derive(Debug)]
pub enum Statement {
    LET { name: Expression, value: Expression },
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
