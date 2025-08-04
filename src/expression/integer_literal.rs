use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::Expression;
use crate::object::{Integer, Object};
use crate::parser::{ParsePrefix, Parser, ParserError};
use crate::token::TokenKind;

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    value: i64,
}

impl IntegerLiteral {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> i64 {
        self.value
    }
}

impl ParsePrefix for IntegerLiteral {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<Expression, ParserError> {
        parser
            .expect_token_with_kind(TokenKind::Integer)
            .map(|token| {
                let value = token.literal().parse::<i64>().expect("expected valid i64");
                Self::new(value)
            })
            .map(Expression::from)
    }
}

impl Evaluate for IntegerLiteral {
    fn evaluate(&self, _: &mut Environment) -> Object {
        Integer::new(self.value).into()
    }
}
