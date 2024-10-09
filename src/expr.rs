use crate::{
    ast::Ast,
    term::Term,
    token::{Token, TokenType},
    token_vec::TokenVec,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AddOrSub {
    Add,
    Sub,
}

#[derive(Debug)]
pub struct Expr<'a> {
    value: Vec<Term<'a>>,
    ops: Vec<AddOrSub>,
    token_len: usize,
}

impl<'a> Ast<'a> for Expr<'a> {
    fn parse(lexed: &'a [Token]) -> Result<Self, &str> {
        let mut cursor = 0;
        let mut value = vec![];
        let mut ops = vec![];

        {
            let term = Term::parse(lexed)?;
            cursor += term.token_len();
            value.push(term);
        }

        loop {
            if (&lexed[cursor..]).consume_kind(TokenType::Plus).is_some() {
                ops.push(AddOrSub::Add);
                let term = Term::parse(&lexed[cursor + 1..])?;
                cursor += 1 + term.token_len();
                value.push(term);
            } else if (&lexed[cursor..]).consume_kind(TokenType::Minus).is_some() {
                ops.push(AddOrSub::Sub);
                let term = Term::parse(&lexed[cursor + 1..])?;
                cursor += 1 + term.token_len();
                value.push(term);
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
