use std::fmt;
use std::fmt::Formatter;

/// The parser chapter guides me to write the code in OOP,
/// but it is easier to write and understand the code in Rust's ways,
/// enumerating the data structure and matching expression.

/// Adhering to the parser chapter requires a token field.
/// Still, I ignored creating it because Rust's enum can represent the Let statement,
/// Return statement, or Expression statement, even without the token field.
#[derive(Debug)]
pub enum Statement {
    LET { name: Expression, value: Expression },
    RETURN(Expression),
    EXPRESSION(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
}
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Statement::LET { name, value } => write!(f, "let {} = {};", name, value),
            Statement::RETURN(x) => write!(f, "return {}", x),
            Statement::EXPRESSION(x) => write!(f, "{}", x),
        }
    }
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(x) => write!(f, "{}", x),
        }
    }
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for statement in self.statements.iter() {
            writeln!(f, "{}", statement).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{Expression, Program, Statement};

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::LET {
                name: Expression::Identifier("myVar".to_string()),
                value: Expression::Identifier("anotherVar".to_string()),
            }],
        };
        assert_eq!("let myVar = anotherVar;\n", program.to_string())
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // my_cunction(x){}
    LBRACKET,    // []
}
