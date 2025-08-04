use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::{self, Identifier};
use crate::object::{Error, Object};
use crate::parser::{Parse, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::TokenKind;

#[derive(Debug, Clone)]
pub struct Let {
    name: Identifier,
    value: expression::Expression,
}

impl Let {
    pub fn new(name: Identifier, value: expression::Expression) -> Self {
        Self { name, value }
    }

    pub const fn name(&self) -> &Identifier {
        &self.name
    }
}

impl Parse for Let {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, ParserError> {
        _ = parser.expect_token_with_kind(TokenKind::Let)?;

        let name = parser
            .expect_token_with_kind(TokenKind::Identifier)
            .map(Identifier::new)?;

        _ = parser.expect_token_with_kind(TokenKind::Assign)?;

        let value = expression::Expression::parse(parser, Precedence::Lowest)?;

        Ok(Self::new(name, value))
    }
}

impl Evaluate for Let {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let value = self.value.evaluate(env);

        if matches!(value, Object::Error(_)) {
            return value;
        }

        let identifier = self.name().token().literal();
        match env.store_mut().insert(identifier.to_owned(), value.clone()) {
            Some(_) => {
                let message = format!("variable named {identifier:?} already exists");
                Error::new(message).into()
            }
            None => match value {
                Object::Return(inner) => inner.value().to_owned(),
                _ => value,
            },
        }
    }
}
