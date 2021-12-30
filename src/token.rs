use crate::lexer::Lexer;

pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

#[derive(Debug,PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

#[test]
fn test_next_token() {
    let _input = "=+(){},;";

    let _tests: Vec<Token> = vec![
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::PLUS,
            literal: String::from("+"),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: String::from("{"),
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: String::from("}"),
        },
        Token {
            token_type: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::EOF,
            literal: String::from(""),
        },
    ];

    let mut lexer = Lexer::new(_input.to_string());
    for (i, tt) in _tests.iter().enumerate() {
        let token = lexer.next_token();
        assert_eq!(token.token_type, tt.token_type, "{}", format!("tests[{}] - tokentype wrong.", i));
        assert_eq!(token.token_type, tt.token_type, "{}", format!("tests[{}] - literal wrong.", i));
    }
}
