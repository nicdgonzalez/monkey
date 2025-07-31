use crate::ast::Node;
use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::{self, Identifier};
use crate::object::{Error, Object};
use crate::parser::{Parse, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Let {
    token: Token,
    name: Identifier,
    value: expression::Expression,
}

impl Let {
    pub fn new(token: Token, name: Identifier, value: expression::Expression) -> Self {
        Self { token, name, value }
    }

    pub const fn token(&self) -> &Token {
        &self.token
    }

    pub const fn name(&self) -> &Identifier {
        &self.name
    }

    pub const fn value(&self) -> &expression::Expression {
        &self.value
    }
}

impl Parse for Let {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        let token = parser.expect_token_with_kind(TokenKind::Let)?;

        let name = parser
            .expect_token_with_kind(TokenKind::Identifier)
            .map(Identifier::new)?;

        _ = parser.expect_token_with_kind(TokenKind::Assign)?;

        let value = expression::Expression::parse(parser, Precedence::Lowest)?;

        let statement = Self::new(token, name, value);
        Ok(Node::Statement(statement.into()))
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
            None => value,
        }
    }
}
