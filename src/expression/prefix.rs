use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::Expression;
use crate::object::{Error, Integer, Object};
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

impl Evaluate for Prefix {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let right = self.right.evaluate(env);

        if matches!(right, Object::Error(_)) {
            return right;
        }

        match self.token.kind() {
            TokenKind::Bang => (!right.as_boolean()).into(),
            TokenKind::Minus => match right {
                Object::Integer(inner) => Integer::new(-inner.value()).into(),
                _ => Error::new(format!("unknown operator: -{right:?}")).into(),
            },
            kind => Error::new(format!("unknown operator: {kind:?}")).into(),
        }
    }
}
