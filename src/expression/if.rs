use crate::ast::Node;
use crate::expression::Expression;
use crate::parser::{Parse, ParsePrefix, Parser, ParserError};
use crate::precedence::Precedence;
use crate::statement::{Block, Statement};
use crate::token::{Token, TokenKind};

#[derive(Debug)]
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
        // TODO: Factor out the parsing logic for each of the statements/expressions, and have the
        // trait implementation just wrap the type in the expected Node/Statement type. That way we
        // can still call `Block::parse`'s logic without having to untangle the inner value...
        let consequence = match Block::parse(parser)? {
            Node::Statement(Statement::Block(inner)) => inner,
            _ => unreachable!("expected Block::parse to return a Block"),
        };

        let alternative = if parser
            .peek()
            .is_some_and(|token| token.kind() == TokenKind::Else)
        {
            parser.advance();
            _ = parser.expect_token_with_kind(TokenKind::LBrace)?;

            Some(match Block::parse(parser)? {
                Node::Statement(Statement::Block(inner)) => inner,
                _ => unreachable!("expected Block::parse to return a Block"),
            })
        } else {
            None
        };

        let expression = Self::new(token, condition, consequence, alternative);
        Ok(expression.into())
    }
}
