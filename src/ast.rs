use crate::token::Token;

pub trait Ast<'a>: Sized {
    fn parse(lexed: &'a [Token]) -> Result<Self, &str>;
    fn token_len(&self) -> usize;
}
