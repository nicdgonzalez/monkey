use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::Expression;
use crate::object::{self, Object};
use crate::parser::{ParsePrefix, Parser, ParserError};
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Boolean {
    token: Token,
    value: bool,
}

impl Boolean {
    pub fn new(token: Token, value: bool) -> Self {
        Self { token, value }
    }

    pub const fn token(&self) -> &Token {
        &self.token
    }

    pub const fn value(&self) -> bool {
        self.value
    }
}

impl ParsePrefix for Boolean {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<Expression, ParserError> {
        assert!(matches!(
            parser.token().map(Token::kind),
            Some(TokenKind::True) | Some(TokenKind::False)
        ));
        let token = parser.token().take().unwrap().to_owned();
        parser.advance();

        let value = token.kind() == TokenKind::True;

        let expression = Self::new(token, value);
        Ok(expression.into())
    }
}

impl Evaluate for Boolean {
    fn evaluate(&self, _: &mut Environment) -> Object {
        object::Boolean::new(self.value).into()
    }
}
