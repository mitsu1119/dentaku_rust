use std::mem;

use crate::token::{Token, TokenType};

trait TokenVec {
    fn expect_kind(&self, kind: TokenType) -> Result<&Token, &str>;
}

impl TokenVec for Vec<Token> {
    fn expect_kind(&self, kind: TokenType) -> Result<&Token, &str> {
        if let Some(tk) = self.first() {
            if mem::discriminant(&tk.kind) == mem::discriminant(&kind) {
                Ok(&tk)
            } else {
                Err("expect_kind")
            }
        } else {
            Err("expect_kind")
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AddOrSub {
    Add,
    Sub,
}

#[derive(Debug)]
pub struct Expr<'a> {
    value: Vec<&'a Token>,
    ops: Vec<AddOrSub>,
}

impl<'a> Expr<'a> {
    pub fn parse(lexed: &'a Vec<Token>) -> Result<Self, &str> {
        let mut value = vec![];
        value.push(lexed.expect_kind(TokenType::Num(0))?);

        Ok(Self { value, ops: vec![] })
    }
}
