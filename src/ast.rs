use std::mem;

use crate::token::{Token, TokenType};

trait TokenVec {
    fn consume_kind(&self, kind: TokenType) -> Option<&Token>;
    fn expect_kind(&self, kind: TokenType) -> Result<&Token, &str>;
}

impl TokenVec for [Token] {
    fn consume_kind(&self, kind: TokenType) -> Option<&Token> {
        if let Some(tk) = self.first() {
            if mem::discriminant(&tk.kind) == mem::discriminant(&kind) {
                Some(&tk)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn expect_kind(&self, kind: TokenType) -> Result<&Token, &str> {
        if let Some(tk) = self.consume_kind(kind) {
            Ok(tk)
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
    pub fn parse(lexed: &'a [Token]) -> Result<Self, &str> {
        let mut cursor = 0;
        let mut value = vec![];
        let mut ops = vec![];

        {
            let tk = lexed.expect_kind(TokenType::Num(0))?;
            value.push(tk);
            cursor += 1;
        }

        loop {
            if (&lexed[cursor..]).consume_kind(TokenType::Plus).is_some() {
                ops.push(AddOrSub::Add);
                let tk = (&lexed[cursor + 1..]).expect_kind(TokenType::Num(0))?;
                value.push(tk);
                cursor += 2;
            } else if (&lexed[cursor..]).consume_kind(TokenType::Minus).is_some() {
                ops.push(AddOrSub::Sub);
                let tk = (&lexed[cursor + 1..]).expect_kind(TokenType::Num(0))?;
                value.push(tk);
                cursor += 2;
            } else {
                break;
            }
        }

        Ok(Self { value, ops })
    }
}
