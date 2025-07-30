use crate::expression::Expression;
use crate::parser::{ParsePrefix, Parser, ParserError};
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct IntegerLiteral {
    token: Token,
    value: i64,
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i64) -> Self {
        Self { token, value }
    }

    pub const fn token(&self) -> &Token {
        &self.token
    }

    pub const fn value(&self) -> i64 {
        self.value
    }
}

impl ParsePrefix for IntegerLiteral {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<Expression, ParserError> {
        parser
            .expect_token_with_kind(TokenKind::Integer)
            .map(|token| {
                let value = token.literal().parse::<i64>().expect("expected valid i64");
                Self::new(token, value)
            })
            .map(Expression::from)
    }
}
