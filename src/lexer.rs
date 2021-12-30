use crate::token::{Token, TokenType};

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
    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token = match self.ch {
            b'=' => Token { token_type: TokenType::ASSIGN, literal: self.ch.to_string() },
            b';' => Token { token_type: TokenType::SEMICOLON, literal: self.ch.to_string() },
            b'(' => Token { token_type: TokenType::LPAREN, literal: self.ch.to_string() },
            b')' => Token { token_type: TokenType::RPAREN, literal: self.ch.to_string() },
            b',' => Token { token_type: TokenType::COMMA, literal: self.ch.to_string() },
            b'+' => Token { token_type: TokenType::PLUS, literal: self.ch.to_string() },
            b'{' => Token { token_type: TokenType::LBRACE, literal: self.ch.to_string() },
            b'}' => Token { token_type: TokenType::RBRACE, literal: self.ch.to_string() },
            0 => Token { token_type: TokenType::EOF, literal: "".to_string() },
            _ => unimplemented!()
        };

        self.read_char();

        return token
    }
}
