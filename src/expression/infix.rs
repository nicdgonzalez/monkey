use crate::expression::Expression;
use crate::parser::{ParseInfix, Parser, ParserError};
use crate::precedence::{PRECEDENCES, Precedence};
use crate::token::Token;

#[derive(Debug)]
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
