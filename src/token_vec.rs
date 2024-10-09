use std::mem;

use crate::token::{Token, TokenType};

pub trait TokenVec {
    fn consume_kind(&self, kind: TokenType) -> Option<&Token>;
    fn expect_kind(&self, kind: TokenType) -> Result<&Token, &str>;
}

impl TokenVec for [Token] {
    fn consume_kind(&self, kind: TokenType) -> Option<&Token> {
        if let Some(tk) = self.first() {
            if mem::discriminant(&tk.kind) == mem::discriminant(&kind) {
                Some(&tk)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn expect_kind(&self, kind: TokenType) -> Result<&Token, &str> {
        if let Some(tk) = self.consume_kind(kind) {
            Ok(tk)
        } else {
            Err("expect_kind")
        }
    }
}
