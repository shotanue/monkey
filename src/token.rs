#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

#[derive(Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
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

    // keyword
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    EQ,
    NotEq,
}

pub fn lookup_ident(ident: &String) -> TokenType {
    match ident.as_str() {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        "true" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        _ => TokenType::IDENT,
    }
}
