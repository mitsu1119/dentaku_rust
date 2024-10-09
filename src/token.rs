#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    Num(i64),
    Plus,
    Minus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Token {
    kind: TokenType,
}
