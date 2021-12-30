use crate::lexer::Lexer;

#[derive(Debug)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    // operator
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,

    //
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

pub fn lookup_ident(ident: &String) -> TokenType {
    match ident.as_str() {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        _ => TokenType::IDENT,
    }
}
