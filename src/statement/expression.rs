use crate::ast::Node;
use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression;
use crate::object::Object;
use crate::parser::{Parse, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Expression {
    token: Token,
    expression: expression::Expression,
}

impl Expression {
    pub fn new(token: Token, expression: expression::Expression) -> Self {
        Self { token, expression }
    }

    pub const fn token(&self) -> &Token {
        &self.token
    }

    pub const fn expression(&self) -> &expression::Expression {
        &self.expression
    }
}

impl Parse for Expression {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        assert_ne!(parser.token(), None);
        let token = parser.token().unwrap().to_owned();

        let expression = expression::Expression::parse(parser, Precedence::Lowest)?;

        if parser
            .peek()
            .is_some_and(|token| token.kind() == TokenKind::Semicolon)
        {
            parser.advance();
        }

        let statement = Self::new(token, expression);
        Ok(Node::Statement(statement.into()))
    }
}

impl Evaluate for Expression {
    fn evaluate(&self, env: &mut Environment) -> Object {
        self.expression.evaluate(env)
    }
}
