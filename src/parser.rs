/*
 * <expr> = <term> ((Plus | Minus) <term>)*
 * <term> = <factor> ((Asterisk | Slash) <factor>)*
 * <factor> = Num
 */

use crate::{ast::Ast, expr::Expr, token::Token};

pub struct Parser {}

impl Parser {
    pub fn parse(lexed: &Vec<Token>) -> Result<Expr, String> {
        let ast = Expr::parse(&lexed[..])?;
        Ok(ast)
    }
}
