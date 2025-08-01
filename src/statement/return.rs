use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::object::Object;
use crate::parser::{Parse, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::{Token, TokenKind};
use crate::{expression, object};

#[derive(Debug, Clone)]
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
    fn parse(parser: &mut Parser<'_>) -> Result<Self, ParserError> {
        let token = parser.expect_token_with_kind(TokenKind::Return)?;

        let value = expression::Expression::parse(parser, Precedence::Lowest)?.into();
        parser.advance();

        if parser
            .peek()
            .is_some_and(|token| token.kind() == TokenKind::Semicolon)
        {
            parser.advance();
        }

        Ok(Self::new(token, value))
    }
}

impl Evaluate for Return {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let value = self.value.evaluate(env);

        if matches!(value, Object::Error(_)) {
            return value;
        }

        object::Return::new(Box::new(value)).into()
    }
}
