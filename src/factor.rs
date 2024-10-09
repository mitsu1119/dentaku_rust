use crate::{
    ast::Ast,
    token::{Token, TokenType},
    token_vec::TokenVec,
};

#[derive(Debug)]
pub struct Factor<'a> {
    value: &'a Token,
}

impl<'a> Ast<'a> for Factor<'a> {
    fn parse(lexed: &'a [Token]) -> Result<Self, String> {
        Ok(Self {
            value: lexed.expect_kind(TokenType::Num(0))?,
        })
    }

    fn token_len(&self) -> usize {
        1
    }

    fn eval(&self) -> i64 {
        if let TokenType::Num(res) = self.value.kind {
            res
        } else {
            0
        }
    }
}
