/*
 * <term> = Num ((MUL | DIV) Num)*
 */
use crate::{
    token::{Token, TokenType},
    token_vec::TokenVec,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MulOrDiv {
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Term<'a> {
    value: Vec<&'a Token>,
    ops: Vec<MulOrDiv>,
}

impl<'a> Term<'a> {
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
            if (&lexed[cursor..])
                .consume_kind(TokenType::Asterisk)
                .is_some()
            {
                ops.push(MulOrDiv::Mul);
                let tk = (&lexed[cursor + 1..]).expect_kind(TokenType::Num(0))?;
                value.push(tk);
                cursor += 2;
            } else if (&lexed[cursor..]).consume_kind(TokenType::Slash).is_some() {
                ops.push(MulOrDiv::Div);
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
