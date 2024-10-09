use crate::token::{Token, TokenType};

#[derive(Debug, Clone, Copy)]
pub struct Lexer {}

impl Lexer {
    fn is_space(ch: &str) -> bool {
        if ch.len() != 1 {
            return false;
        }

        ch.chars().next().unwrap().is_whitespace()
    }

    pub fn lex(target: &str) -> Vec<Token> {
        if target.len() == 0 {
            return vec![];
        }

        let mut res = vec![];
        let mut cursor: usize = 0;

        while cursor < target.len() {
            let ch = &target[cursor..cursor + 1];
            if Lexer::is_space(ch) {
                cursor += 1;
                continue;
            } else {
                res.push(Token::new(TokenType::Unidentified(ch.to_string())));
                cursor += 1;
                continue;
            }
        }

        res
    }
}
