use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::Expression;
use crate::object::{Error, Object};
use crate::parser::{ParsePrefix, Parser, ParserError};
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Identifier {
    token: Token,
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        Self { token }
    }

    pub const fn token(&self) -> &Token {
        &self.token
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

impl Evaluate for Identifier {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let identifier = self.token.literal();

        match env.get(identifier) {
            Some(value) => value.to_owned(),
            None => Error::new(format!("identifier {identifier:?} is not defined")).into(),
        }
    }
}
