use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::object::Object;
use crate::parser::{Parse, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::TokenKind;
use crate::{expression, object};

#[derive(Debug, Clone)]
pub struct Return {
    value: expression::Expression,
}

impl Return {
    pub fn new(value: expression::Expression) -> Self {
        Self { value }
    }
}

impl Parse for Return {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, ParserError> {
        _ = parser.expect_token_with_kind(TokenKind::Return)?;
        let value = expression::Expression::parse(parser, Precedence::Lowest)?;

        if parser
            .token()
            .is_some_and(|token| token.kind() == TokenKind::Semicolon)
        {
            parser.advance();
        }

        Ok(Self::new(value))
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
