use crate::{
    ast::Ast,
    factor::Factor,
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
    value: Vec<Factor<'a>>,
    ops: Vec<MulOrDiv>,
    token_len: usize,
}

impl<'a> Ast<'a> for Term<'a> {
    fn parse(lexed: &'a [Token]) -> Result<Self, &str> {
        let mut cursor = 0;
        let mut value = vec![];
        let mut ops = vec![];

        {
            let factor = Factor::parse(&lexed[cursor..])?;
            cursor += factor.token_len();
            value.push(factor);
        }

        loop {
            if (&lexed[cursor..])
                .consume_kind(TokenType::Asterisk)
                .is_some()
            {
                ops.push(MulOrDiv::Mul);
                let factor = Factor::parse(&lexed[cursor + 1..])?;
                cursor += 1 + factor.token_len();
                value.push(factor);
            } else if (&lexed[cursor..]).consume_kind(TokenType::Slash).is_some() {
                ops.push(MulOrDiv::Div);
                let factor = Factor::parse(&lexed[cursor + 1..])?;
                cursor += 1 + factor.token_len();
                value.push(factor);
            } else {
                break;
            }
        }

        Ok(Self {
            value,
            ops,
            token_len: cursor,
        })
    }

    fn token_len(&self) -> usize {
        self.token_len
    }
}
