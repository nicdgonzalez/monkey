use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression;
use crate::object::Object;
use crate::parser::{Parse, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::TokenKind;

#[derive(Debug, Clone)]
pub struct Expression {
    expression: expression::Expression,
}

impl Expression {
    pub fn new(expression: expression::Expression) -> Self {
        Self { expression }
    }
}

impl Parse for Expression {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, ParserError> {
        let expression = expression::Expression::parse(parser, Precedence::Lowest)?;

        if parser
            .token()
            .is_some_and(|token| token.kind() == TokenKind::Semicolon)
        {
            parser.advance();
        }

        Ok(Self::new(expression))
    }
}

impl Evaluate for Expression {
    fn evaluate(&self, env: &mut Environment) -> Object {
        self.expression.evaluate(env)
    }
}
