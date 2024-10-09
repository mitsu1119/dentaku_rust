use crate::{ast::Ast, lexer::Lexer, parser::Parser};

#[derive(Debug, Clone, Copy)]
pub struct Calc {}

impl Calc {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, expr: &str) -> Result<i64, String> {
        let lexed = Lexer::lex(expr);
        let ast = Parser::parse(&lexed)?;
        Ok(ast.eval())
    }
}
