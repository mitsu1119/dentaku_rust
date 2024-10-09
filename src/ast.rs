use crate::token::Token;

pub trait Ast<'a>: Sized {
    fn parse(lexed: &'a [Token]) -> Result<Self, String>;
    fn token_len(&self) -> usize;
    fn eval(&self) -> i64;
}
