use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();
        return lexer;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }
    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while Self::is_letter(&self.ch) {
            self.read_char();
        }
        return self.input[position..self.position].as_ref();
    }

    fn is_letter(&ch: &u8) -> bool {
        return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_';
    }
    fn read_number(&mut self) -> &str {
        let position = self.position;
        while Self::is_digit(&self.ch) {
            self.read_char();
        }

        return self.input[position..self.position].as_ref();
    }

    fn is_digit(&ch: &u8) -> bool {
        return b'0' <= ch && ch <= b'9';
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char()
        }
    }
    fn new_token(token_type: TokenType, ch: u8) -> Token {
        Token {
            token_type,
            literal: String::from_utf8(vec![ch]).unwrap(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token: Token = match self.ch {
            b'=' if self.peek_char() == b'=' => Token {
                token_type: TokenType::EQ,
                literal: (|| -> String {
                    let mut ch = String::from_utf8(vec![self.ch]).unwrap();
                    self.read_char();
                    ch.push_str(String::from_utf8(vec![self.ch]).unwrap().as_str());
                    return ch;
                })(),
            },
            b'=' if self.peek_char() != b'=' => Self::new_token(TokenType::ASSIGN, self.ch),
            b'(' => Self::new_token(TokenType::LPAREN, self.ch),
            b')' => Self::new_token(TokenType::RPAREN, self.ch),
            b',' => Self::new_token(TokenType::COMMA, self.ch),
            b'+' => Self::new_token(TokenType::PLUS, self.ch),
            b'-' => Self::new_token(TokenType::MINUS, self.ch),
            b'!' if self.peek_char() == b'=' => Token {
                token_type: TokenType::NotEq,
                literal: (|| -> String {
                    let mut ch = String::from_utf8(vec![self.ch]).unwrap();
                    self.read_char();
                    ch.push_str(String::from_utf8(vec![self.ch]).unwrap().as_str());
                    return ch;
                })(),
            },
            b'!' if self.peek_char() != b'=' => Self::new_token(TokenType::BANG, self.ch),
            b'/' => Self::new_token(TokenType::SLASH, self.ch),
            b'*' => Self::new_token(TokenType::ASTERISK, self.ch),
            b'<' => Self::new_token(TokenType::LT, self.ch),
            b'>' => Self::new_token(TokenType::GT, self.ch),
            b';' => Self::new_token(TokenType::SEMICOLON, self.ch),
            b'{' => Self::new_token(TokenType::LBRACE, self.ch),
            b'}' => Self::new_token(TokenType::RBRACE, self.ch),
            0 => Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },
            _ => {
                if Self::is_letter(&self.ch) {
                    let literal = self.read_identifier().to_string();
                    // lookup_ident() calls read_char(), this should return early.
                    return Token {
                        token_type: lookup_ident(&literal),
                        literal,
                    };
                } else if Self::is_digit(&self.ch) {
                    return Token {
                        token_type: TokenType::INT,
                        literal: self.read_number().to_string(),
                    };
                } else {
                    Token {
                        token_type: TokenType::ILLEGAL,
                        literal: self.ch.to_string(),
                    }
                }
            }
        };

        self.read_char();
        return token;
    }
}

#[test]
fn test_is_letter() {
    assert_eq!(Lexer::is_letter(&b'a'), true);
    assert_eq!(Lexer::is_letter(&b'z'), true);
    assert_eq!(Lexer::is_letter(&b'A'), true);
    assert_eq!(Lexer::is_letter(&b'Z'), true);
    assert_eq!(Lexer::is_letter(&b'_'), true);
    assert_eq!(Lexer::is_letter(&b' '), false);
}
#[test]
fn test_is_digit() {
    assert_eq!(Lexer::is_digit(&b'0'), true);
    assert_eq!(Lexer::is_digit(&b'1'), true);
    assert_eq!(Lexer::is_digit(&b'2'), true);
    assert_eq!(Lexer::is_digit(&b'3'), true);
    assert_eq!(Lexer::is_digit(&b'4'), true);
    assert_eq!(Lexer::is_digit(&b'5'), true);
    assert_eq!(Lexer::is_digit(&b'6'), true);
    assert_eq!(Lexer::is_digit(&b'7'), true);
    assert_eq!(Lexer::is_digit(&b'8'), true);
    assert_eq!(Lexer::is_digit(&b'9'), true);
    assert_eq!(Lexer::is_digit(&b' '), false);
}

#[test]
fn test_next_token() {
    let _input = r#"let five = 5;
let ten = 10;

let add = fn(x,y) {
  x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
  return true;
} else {
  return false;
}

10 == 10;
10 != 9;
    "#;

    let _tests: Vec<(TokenType, String)> = vec![
        (TokenType::LET, String::from("let")),
        (TokenType::IDENT, String::from("five")),
        (TokenType::ASSIGN, String::from("=")),
        (TokenType::INT, String::from("5")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::LET, String::from("let")),
        (TokenType::IDENT, String::from("ten")),
        (TokenType::ASSIGN, String::from("=")),
        (TokenType::INT, String::from("10")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::LET, String::from("let")),
        (TokenType::IDENT, String::from("add")),
        (TokenType::ASSIGN, String::from("=")),
        (TokenType::FUNCTION, String::from("fn")),
        (TokenType::LPAREN, String::from("(")),
        (TokenType::IDENT, String::from("x")),
        (TokenType::COMMA, String::from(",")),
        (TokenType::IDENT, String::from("y")),
        (TokenType::RPAREN, String::from(")")),
        (TokenType::LBRACE, String::from("{")),
        (TokenType::IDENT, String::from("x")),
        (TokenType::PLUS, String::from("+")),
        (TokenType::IDENT, String::from("y")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::RBRACE, String::from("}")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::LET, String::from("let")),
        (TokenType::IDENT, String::from("result")),
        (TokenType::ASSIGN, String::from("=")),
        (TokenType::IDENT, String::from("add")),
        (TokenType::LPAREN, String::from("(")),
        (TokenType::IDENT, String::from("five")),
        (TokenType::COMMA, String::from(",")),
        (TokenType::IDENT, String::from("ten")),
        (TokenType::RPAREN, String::from(")")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::BANG, String::from("!")),
        (TokenType::MINUS, String::from("-")),
        (TokenType::SLASH, String::from("/")),
        (TokenType::ASTERISK, String::from("*")),
        (TokenType::INT, String::from("5")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::INT, String::from("5")),
        (TokenType::LT, String::from("<")),
        (TokenType::INT, String::from("10")),
        (TokenType::GT, String::from(">")),
        (TokenType::INT, String::from("5")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::IF, String::from("if")),
        (TokenType::LPAREN, String::from("(")),
        (TokenType::INT, String::from("5")),
        (TokenType::LT, String::from("<")),
        (TokenType::INT, String::from("10")),
        (TokenType::RPAREN, String::from(")")),
        (TokenType::LBRACE, String::from("{")),
        (TokenType::RETURN, String::from("return")),
        (TokenType::TRUE, String::from("true")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::RBRACE, String::from("}")),
        (TokenType::ELSE, String::from("else")),
        (TokenType::LBRACE, String::from("{")),
        (TokenType::RETURN, String::from("return")),
        (TokenType::FALSE, String::from("false")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::RBRACE, String::from("}")),
        (TokenType::INT, String::from("10")),
        (TokenType::EQ, String::from("==")),
        (TokenType::INT, String::from("10")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::INT, String::from("10")),
        (TokenType::NotEq, String::from("!=")),
        (TokenType::INT, String::from("9")),
        (TokenType::SEMICOLON, String::from(";")),
        (TokenType::EOF, String::from("")),
    ];

    let mut lexer = Lexer::new(_input.to_string());
    for (i, tt) in _tests.iter().enumerate() {
        let token = lexer.next_token();
        assert_eq!(
            token.token_type,
            tt.0,
            "{}",
            format!("tests[{}] - tokentype wrong.", i)
        );
        assert_eq!(
            token.literal,
            tt.1,
            "{}",
            format!("tests[{}] - literal wrong.", i)
        );
    }
}
