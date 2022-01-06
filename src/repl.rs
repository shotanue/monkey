use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io::Stdin;

pub fn start(stdin: Stdin) {
    loop {
        let mut s = String::new();
        stdin.read_line(&mut s).expect("failed to read stdin");

        if s.is_empty() {
            return;
        }
        let mut lexer = Lexer::new(s);
        loop {
            let token = lexer.next_token();
            match token.token_type {
                TokenType::EOF => break,
                _ => println!("{:?}", token),
            }
        }
    }
}
