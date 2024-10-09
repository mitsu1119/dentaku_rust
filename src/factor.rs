use crate::{
    ast::Ast,
    expr::Expr,
    token::{Token, TokenType},
    token_vec::TokenVec,
};

#[derive(Debug)]
enum FactorValue<'a> {
    Num(&'a Token),
    Exp(Expr<'a>),
}

#[derive(Debug)]
pub struct Factor<'a> {
    value: FactorValue<'a>,
    token_len: usize,
}

impl<'a> Ast<'a> for Factor<'a> {
    fn parse(lexed: &'a [Token]) -> Result<Self, String> {
        if lexed.consume_kind(TokenType::Lbrace).is_some() {
            let mut cursor = 1;
            let expr = Expr::parse(&lexed[cursor..])?;
            cursor += expr.token_len();
            let _ = &lexed[cursor..].expect_kind(TokenType::Rbrace)?;
            cursor += 1;

            Ok(Self {
                value: FactorValue::Exp(expr),
                token_len: cursor,
            })
        } else {
            Ok(Self {
                value: FactorValue::Num(lexed.expect_kind(TokenType::Num(0))?),
                token_len: 1,
            })
        }
    }

    fn token_len(&self) -> usize {
        self.token_len
    }

    fn eval(&self) -> i64 {
        match self.value {
            FactorValue::Num(tk) => {
                if let TokenType::Num(res) = tk.kind {
                    res
                } else {
                    0
                }
            }
            FactorValue::Exp(ref expr) => expr.eval(),
        }
    }
}
