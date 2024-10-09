/*
 * <expr> = <term> ((Plus | Minus) <term>)*
 * <term> = Num ((Asterisk | Slash) Num)*
 */

use crate::{ast::Ast, expr::Expr, token::Token};

pub struct Parser {}

impl Parser {
    pub fn parse(lexed: &Vec<Token>) -> Result<Expr, &str> {
        let ast = Expr::parse(&lexed[..])?;
        Ok(ast)
    }
}
