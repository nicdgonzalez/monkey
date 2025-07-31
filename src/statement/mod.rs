mod block;
mod expression;
mod r#let;
mod r#return;

use crate::ast::Node;
use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::object::Object;
use crate::parser::{Parse, Parser, ParserError};
use crate::token::TokenKind;

pub use block::Block;
pub use expression::Expression;
pub use r#let::Let;
pub use r#return::Return;

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Return(Return),
    Expression(self::Expression),
    Block(Block),
}

impl Parse for Statement {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        assert!(parser.token().is_some(), "Statement::parse after EOF");

        match parser.token().unwrap().kind() {
            TokenKind::Let => Let::parse(parser),
            TokenKind::Return => Return::parse(parser),
            // There also exists a `crate::expression::Expression`; the two are not the same.
            _ => self::Expression::parse(parser),
        }
    }
}

impl Evaluate for Statement {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let inner: &dyn Evaluate = match *self {
            Self::Let(ref inner) => inner,
            Self::Return(ref inner) => inner,
            Self::Expression(ref inner) => inner,
            Self::Block(ref inner) => inner,
        };

        inner.evaluate(env)
    }
}

impl From<Let> for Statement {
    fn from(value: Let) -> Self {
        Statement::Let(value)
    }
}

impl From<Return> for Statement {
    fn from(value: Return) -> Self {
        Statement::Return(value)
    }
}

impl From<Expression> for Statement {
    fn from(value: Expression) -> Self {
        Statement::Expression(value)
    }
}

impl From<Block> for Statement {
    fn from(value: Block) -> Self {
        Statement::Block(value)
    }
}
