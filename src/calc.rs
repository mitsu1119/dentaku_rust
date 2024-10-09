use crate::lexer::Lexer;

#[derive(Debug, Clone, Copy)]
pub struct Calc {}

impl Calc {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, expr: &str) -> i64 {
        let lexed = Lexer::lex(expr);
        println!("lexer: {:?}", lexed);
        100
    }
}
