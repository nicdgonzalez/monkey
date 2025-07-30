use crate::expression::Expression;
use crate::parser::{ParsePrefix, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::TokenKind;

#[derive(Default)]
pub struct Grouped;

impl Grouped {
    pub fn new() -> Self {
        Self {}
    }
}

impl ParsePrefix for Grouped {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<Expression, ParserError> {
        _ = parser.expect_token_with_kind(TokenKind::LParenthesis)?;
        let expression = Expression::parse(parser, Precedence::Lowest)?;
        _ = parser.expect_token_with_kind(TokenKind::RParenthesis)?;
        Ok(expression)
    }
}
