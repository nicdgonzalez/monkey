use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::Expression;
use crate::object::{NULL, Object};
use crate::parser::{Parse, ParsePrefix, Parser, ParserError};
use crate::precedence::Precedence;
use crate::statement::Block;
use crate::token::TokenKind;

#[derive(Debug, Clone)]
pub struct If {
    condition: Box<Expression>,
    consequence: Block,
    alternative: Option<Block>,
}

impl If {
    pub fn new(condition: Box<Expression>, consequence: Block, alternative: Option<Block>) -> Self {
        Self {
            condition,
            consequence,
            alternative,
        }
    }
}

impl ParsePrefix for If {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<Expression, ParserError> {
        _ = parser.expect_token_with_kind(TokenKind::If)?;
        let condition = Box::new(Expression::parse(parser, Precedence::Lowest)?);
        let consequence = Block::parse(parser)?;

        let alternative = if parser
            .token()
            .is_some_and(|token| token.kind() == TokenKind::Else)
        {
            _ = parser.expect_token_with_kind(TokenKind::Else)?;
            Some(Block::parse(parser)?)
        } else {
            None
        };

        let expression = Self::new(condition, consequence, alternative);
        Ok(expression.into())
    }
}

impl Evaluate for If {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let condition = self.condition.evaluate(env);

        if matches!(condition, Object::Error(_)) {
            return condition;
        }

        if condition.as_boolean().value() {
            self.consequence.evaluate(env)
        } else if self.alternative.is_some() {
            self.alternative.as_ref().unwrap().evaluate(env)
        } else {
            NULL
        }
    }
}
