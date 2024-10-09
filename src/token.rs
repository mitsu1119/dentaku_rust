#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Num(i64),
    Plus,
    Minus,
    Unidentified(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    kind: TokenType,
}

impl Token {
    pub fn new(kind: TokenType) -> Self {
        Self { kind }
    }
}
