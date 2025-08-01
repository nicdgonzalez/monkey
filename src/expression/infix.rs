use std::mem;

use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::Expression;
use crate::object::{Boolean, Error, Integer, NULL, Object};
use crate::parser::{ParseInfix, Parser, ParserError};
use crate::precedence::{PRECEDENCES, Precedence};
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Infix {
    token: Token,
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Infix {
    pub fn new(token: Token, left: Box<Expression>, right: Box<Expression>) -> Self {
        Self { token, left, right }
    }

    pub const fn token(&self) -> &Token {
        &self.token
    }

    pub fn left(&self) -> &Expression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &Expression {
        self.right.as_ref()
    }
}

impl ParseInfix for Infix {
    fn parse_infix(parser: &mut Parser<'_>, left: Expression) -> Result<Expression, ParserError> {
        let token = parser.token().take().unwrap().to_owned();
        let precedence = PRECEDENCES
            .get(&token.kind())
            .unwrap_or(&Precedence::Lowest);
        parser.advance();

        let right = Expression::parse(parser, precedence.to_owned())?;

        let expression = Self::new(token, Box::new(left), Box::new(right));
        Ok(expression.into())
    }
}

impl Evaluate for Infix {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let left = self.left.evaluate(env);

        if matches!(left, Object::Error(_)) {
            return left;
        }

        let right = self.right.evaluate(env);

        if matches!(right, Object::Error(_)) {
            return right;
        }

        let operator = self.token.kind();

        if mem::discriminant(&left) != mem::discriminant(&right) {
            let message = format!("type mismatch: {left:?} and {right:?}");
            return Error::new(message).into();
        }

        match (operator, &left, &right) {
            (_, Object::Integer(inner_left), Object::Integer(inner_right)) => match operator {
                TokenKind::Plus => Integer::new(inner_left.value() + inner_right.value()).into(),
                TokenKind::Minus => Integer::new(inner_left.value() - inner_right.value()).into(),
                TokenKind::Asterisk => {
                    Integer::new(inner_left.value() * inner_right.value()).into()
                }
                TokenKind::Slash => Integer::new(inner_left.value() / inner_right.value()).into(),
                TokenKind::LessThan => {
                    Boolean::new(inner_left.value() < inner_right.value()).into()
                }
                TokenKind::GreaterThan => {
                    Boolean::new(inner_left.value() > inner_right.value()).into()
                }
                TokenKind::Equal => Boolean::new(inner_left.value() == inner_right.value()).into(),
                TokenKind::NotEqual => {
                    Boolean::new(inner_left.value() != inner_right.value()).into()
                }
                _ => NULL,
            },
            (TokenKind::Equal, _, _) => Boolean::new(left == right).into(),
            (TokenKind::NotEqual, _, _) => Boolean::new(left != right).into(),
            _ => {
                let message = format!("unknown operator: {left:?} {operator:?} {right:?}");
                Error::new(message).into()
            }
        }
    }
}
