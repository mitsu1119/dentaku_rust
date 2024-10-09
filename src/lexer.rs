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

    fn is_numeric(ch: &str) -> bool {
        if ch.len() != 1 {
            return false;
        }

        ch.chars().next().unwrap().is_numeric()
    }

    fn is_not_numeric(ch: char) -> bool {
        !ch.is_numeric()
    }

    fn read_decimal(start: &str) -> &str {
        if let Some(ind) = start.find(Self::is_not_numeric) {
            &start[..ind]
        } else {
            start
        }
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
            } else if Lexer::is_numeric(ch) {
                let num = Self::read_decimal(&target[cursor..]);
                let tk = Token::new(TokenType::Unidentified(num.to_string()));
                res.push(tk);
                cursor += num.len();
            } else {
                let tk = Token::new(TokenType::Unidentified(ch.to_string()));
                res.push(tk);
                cursor += 1;
            }
        }

        res
    }
}
