use core::panic;

use crate::token::Token;

use super::expression;
use super::parser::{Parse, Parser, ParserError};
use super::Node;

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Return(Return),
    Expression(Expression),
    Block(Block),
}

impl From<Node> for Statement {
    fn from(value: Node) -> Self {
        match value {
            Node::Statement(statement) => statement,
            _ => panic!("expected node to be a statement"),
        }
    }
}

// ┌─────────────────────────┐
// │ Implementations for Let │
// └─────────────────────────┘

#[derive(Debug)]
pub struct Let {
    pub token: Token,
    pub name: expression::Identifier,
    pub value: Box<expression::Expression>,
}

impl From<Let> for Statement {
    fn from(value: Let) -> Self {
        Statement::Let(value)
    }
}

impl From<Let> for Node {
    fn from(value: Let) -> Self {
        Statement::Let(value).into()
    }
}

impl Parse<'_> for Let {
    fn parse(_parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        todo!()
    }
}

// ┌────────────────────────────┐
// │ Implementations for Return │
// └────────────────────────────┘

#[derive(Debug)]
pub struct Return {
    pub token: Token,
    pub value: Box<expression::Expression>,
}

impl From<Return> for Statement {
    fn from(value: Return) -> Self {
        Statement::Return(value)
    }
}

impl From<Return> for Node {
    fn from(value: Return) -> Self {
        Statement::Return(value).into()
    }
}

impl Parse<'_> for Return {
    fn parse(_parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        todo!()
    }
}

// ┌────────────────────────────────┐
// │ Implementations for Expression │
// └────────────────────────────────┘

#[derive(Debug)]
pub struct Expression {
    pub token: Token,
    pub expression: Box<expression::Expression>,
}

impl From<Expression> for Statement {
    fn from(value: Expression) -> Self {
        Statement::Expression(value)
    }
}

impl From<Expression> for Node {
    fn from(value: Expression) -> Self {
        Statement::Expression(value).into()
    }
}

impl Parse<'_> for Expression {
    fn parse(_parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        todo!()
    }
}

// ┌───────────────────────────┐
// │ Implementations for Block │
// └───────────────────────────┘

#[derive(Debug)]
pub struct Block {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl From<Block> for Statement {
    fn from(value: Block) -> Self {
        Statement::Block(value)
    }
}

impl From<Block> for Node {
    fn from(value: Block) -> Self {
        Statement::Block(value).into()
    }
}

impl Parse<'_> for Block {
    fn parse(_parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        todo!()
    }
}
