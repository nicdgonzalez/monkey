use crate::expression::Expression;
use crate::parser::{ParsePrefix, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Prefix {
    token: Token,
    right: Box<Expression>,
}

impl Prefix {
    pub fn new(token: Token, right: Box<Expression>) -> Self {
        Self { token, right }
    }

    pub const fn token(&self) -> &Token {
        &self.token
    }

    pub fn right(&self) -> &Expression {
        self.right.as_ref()
    }
}

impl ParsePrefix for Prefix {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<Expression, ParserError> {
        assert!(matches!(
            parser.token().map(|token| token.kind()),
            Some(TokenKind::Minus) | Some(TokenKind::Bang)
        ));
        let token = parser.token().take().unwrap().to_owned();
        parser.advance();

        let right = Box::new(Expression::parse(parser, Precedence::Prefix)?);

        let expression = Self::new(token, right);
        Ok(expression.into())
    }
}
