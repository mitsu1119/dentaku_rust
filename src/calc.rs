use crate::{lexer::Lexer, parser::Parser};

#[derive(Debug, Clone, Copy)]
pub struct Calc {}

impl Calc {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, expr: &str) -> i64 {
        let lexed = Lexer::lex(expr);
        let ast = Parser::parse(&lexed);
        println!("lexer: {:?}", lexed);
        println!("ast  : {:?}", ast);
        100
    }
}
