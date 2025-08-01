use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::Expression;
use crate::object::{NULL, Object};
use crate::parser::{Parse, ParsePrefix, Parser, ParserError};
use crate::precedence::Precedence;
use crate::statement::{Block, Statement};
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct If {
    token: Token,
    condition: Box<Expression>,
    consequence: Block,
    alternative: Option<Block>,
}

impl If {
    pub fn new(
        token: Token,
        condition: Box<Expression>,
        consequence: Block,
        alternative: Option<Block>,
    ) -> Self {
        Self {
            token,
            condition,
            consequence,
            alternative,
        }
    }

    pub fn condition(&self) -> &Expression {
        self.condition.as_ref()
    }

    pub const fn consequence(&self) -> &Block {
        &self.consequence
    }

    pub fn alternative(&self) -> Option<&Block> {
        self.alternative.as_ref()
    }
}

impl ParsePrefix for If {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<Expression, ParserError> {
        let token = parser.expect_token_with_kind(TokenKind::If)?;
        _ = parser.expect_token_with_kind(TokenKind::LParenthesis)?;
        let condition = Box::new(Expression::parse(parser, Precedence::Lowest)?);
        _ = parser.expect_token_with_kind(TokenKind::RParenthesis)?;
        let consequence = Block::parse(parser)?;

        let alternative = if parser
            .peek()
            .is_some_and(|token| token.kind() == TokenKind::Else)
        {
            parser.advance();
            _ = parser.expect_token_with_kind(TokenKind::LBrace)?;

            Some(Block::parse(parser)?)
        } else {
            None
        };

        let expression = Self::new(token, condition, consequence, alternative);
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
            self.condition.evaluate(env)
        } else if self.alternative.is_some() {
            self.alternative.as_ref().unwrap().evaluate(env)
        } else {
            NULL
        }
    }
}
