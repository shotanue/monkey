use crate::ast::Expression::Identifier;
use crate::ast::{Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        return Parser {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
        };
    }

    fn peek_error(&mut self, token: &TokenType) {
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?} instead",
            token, token,
        ))
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program { statements: vec![] };

        while self.current_token.token_type != TokenType::EOF {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }
        return Ok(program);
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            _ => None,
        }
    }
    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let name = Identifier(self.current_token.literal.clone());
        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }
        while self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        return Some(Statement::LET {
            name,
            value: Identifier(String::from("")),
        });
    }
    fn current_token_is(&self, token: TokenType) -> bool {
        return self.current_token.token_type == token;
    }
    fn peek_token_is(&self, token: &TokenType) -> bool {
        return self.peek_token.token_type == *token;
    }

    pub fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(&token) {
            self.next_token();
            return true;
        }
        self.peek_error(&token);
        return false;
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{Expression, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn check_parser_errors(parser: &Parser) {
        let errors = &parser.errors;
        if errors.is_empty() {
            return;
        }
        eprintln!("parser has {} errors", errors.len());
        for error in &parser.errors {
            eprintln!("{}", error)
        }
        panic!()
    }

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(String::from(input));
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements"
        );
        let tests = vec!["x", "y", "foobar"];
        for (i, expected_identifier) in tests.iter().enumerate() {
            let statement = &program.statements[i];
            test_let_statement(statement, &expected_identifier);
        }
    }

    fn test_let_statement(statement: &Statement, expected_identifier: &str) {
        match statement {
            Statement::LET { name, .. } => match name {
                Expression::Identifier(identifier_name) => {
                    assert_eq!(
                        identifier_name, expected_identifier,
                        "check let statement name"
                    );
                }
            },
            #[allow(unreachable_patterns)]
            x => {
                panic!("statement is not Statement::Let. got={:?}", x);
            }
        };
    }
}
