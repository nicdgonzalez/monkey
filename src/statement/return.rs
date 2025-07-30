use crate::ast::Node;
use crate::expression;
use crate::parser::{Parse, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Return {
    token: Token,
    value: expression::Expression,
}

impl Return {
    pub fn new(token: Token, value: expression::Expression) -> Self {
        Self { token, value }
    }

    pub const fn token(&self) -> &Token {
        &self.token
    }

    pub const fn value(&self) -> &expression::Expression {
        &self.value
    }
}

impl Parse for Return {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        let token = parser.expect_token_with_kind(TokenKind::Return)?;

        let value = expression::Expression::parse(parser, Precedence::Lowest)?.into();
        parser.advance();

        if parser
            .peek()
            .is_some_and(|token| token.kind() == TokenKind::Semicolon)
        {
            parser.advance();
        }

        let statement = Self::new(token, value);
        Ok(Node::Statement(statement.into()))
    }
}
