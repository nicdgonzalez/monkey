use crate::expression::Expression;
use crate::parser::{ParsePrefix, Parser, ParserError};
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Identifier {
    token: Token,
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ParsePrefix for Identifier {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<Expression, ParserError> {
        parser
            .expect_token_with_kind(TokenKind::Identifier)
            .map(Self::new)
            .map(Expression::from)
    }
}
