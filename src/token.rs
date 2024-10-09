#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Num(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lbrace,
    Rbrace,
    Unidentified(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenType,
}

impl Token {
    pub fn new(kind: TokenType) -> Self {
        Self { kind }
    }
}
