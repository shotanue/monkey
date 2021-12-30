use std::io::stdin;

mod lexer;
mod repl;
mod token;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");

    repl::start(stdin());
}
