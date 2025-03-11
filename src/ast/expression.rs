use crate::token::{Token, TokenKind};

use super::parser::{Parse, Parser, ParserError};
use super::statement::Block;
use super::Node;

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Prefix(Prefix),
    Infix(Infix),
    Boolean(Boolean),
    If(If),
    FunctionLiteral(FunctionLiteral),
    Call(Call),
}

impl From<Node> for Expression {
    fn from(value: Node) -> Self {
        match value {
            Node::Expression(expression) => expression,
            _ => panic!("expected node to be an expression"),
        }
    }
}

// ┌────────────────────────────────┐
// │ Implementations for Identifier │
// └────────────────────────────────┘

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
}

impl From<Identifier> for Expression {
    fn from(value: Identifier) -> Self {
        Expression::Identifier(value)
    }
}

impl From<Identifier> for Node {
    fn from(value: Identifier) -> Self {
        Expression::Identifier(value).into()
    }
}

impl Parse<'_> for Identifier {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        let node = parser
            .expect_token(TokenKind::Identifier)
            .map(|token| Self { token })?
            .into();

        Ok(node)
    }
}

// ┌─────────────────────────────────────┐
// │ Implementations for Integer Literal │
// └─────────────────────────────────────┘

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl From<IntegerLiteral> for Expression {
    fn from(value: IntegerLiteral) -> Self {
        Expression::IntegerLiteral(value)
    }
}

impl From<IntegerLiteral> for Node {
    fn from(value: IntegerLiteral) -> Self {
        Expression::IntegerLiteral(value).into()
    }
}

impl Parse<'_> for IntegerLiteral {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        let node = parser
            .expect_token(TokenKind::Integer)
            .map(|token| {
                let value = token.literal.clone().parse::<i64>().unwrap();
                Self { token, value }
            })?
            .into();

        Ok(node)
    }
}

// ┌────────────────────────────┐
// │ Implementations for Prefix │
// └────────────────────────────┘

#[derive(Debug)]
pub struct Prefix {
    pub token: Token,
    pub right: Box<Expression>,
}

impl From<Prefix> for Expression {
    fn from(value: Prefix) -> Self {
        Expression::Prefix(value)
    }
}
impl From<Prefix> for Node {
    fn from(value: Prefix) -> Self {
        Expression::Prefix(value).into()
    }
}

impl Parse<'_> for Prefix {
    fn parse(_parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        todo!()
    }
}

// ┌───────────────────────────┐
// │ Implementations for Infix │
// └───────────────────────────┘

#[derive(Debug)]
pub struct Infix {
    pub token: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl From<Infix> for Expression {
    fn from(value: Infix) -> Self {
        Expression::Infix(value)
    }
}

impl From<Infix> for Node {
    fn from(value: Infix) -> Self {
        Expression::Infix(value).into()
    }
}

impl Parse<'_> for Infix {
    fn parse(_parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        todo!()
    }
}

// ┌─────────────────────────────┐
// │ Implementations for Boolean │
// └─────────────────────────────┘

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl From<Boolean> for Expression {
    fn from(value: Boolean) -> Self {
        Expression::Boolean(value)
    }
}

impl From<Boolean> for Node {
    fn from(value: Boolean) -> Self {
        Expression::Boolean(value).into()
    }
}

impl Parse<'_> for Boolean {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        let token = parser.token.clone();
        let value = token.kind == TokenKind::True;
        let node = Self { token, value }.into();
        Ok(node)
    }
}

// ┌────────────────────────┐
// │ Implementations for If │
// └────────────────────────┘

#[derive(Debug)]
pub struct If {
    pub token: Token,
    pub condition: Box<Expression>,
    pub statement: Box<Expression>,
    pub alternative: Option<Box<Expression>>,
}

impl From<If> for Expression {
    fn from(value: If) -> Self {
        Expression::If(value)
    }
}

impl From<If> for Node {
    fn from(value: If) -> Self {
        Expression::If(value).into()
    }
}

impl Parse<'_> for If {
    fn parse(_parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        todo!()
    }
}

// ┌──────────────────────────────────────┐
// │ Implementations for Function Literal │
// └──────────────────────────────────────┘

#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Block,
}

impl From<FunctionLiteral> for Expression {
    fn from(value: FunctionLiteral) -> Self {
        Expression::FunctionLiteral(value)
    }
}

impl From<FunctionLiteral> for Node {
    fn from(value: FunctionLiteral) -> Self {
        Expression::FunctionLiteral(value).into()
    }
}

impl Parse<'_> for FunctionLiteral {
    fn parse(_parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        todo!()
    }
}

// ┌──────────────────────────┐
// │ Implementations for Call │
// └──────────────────────────┘

#[derive(Debug)]
pub struct Call {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
}

impl From<Call> for Expression {
    fn from(value: Call) -> Self {
        Expression::Call(value)
    }
}

impl From<Call> for Node {
    fn from(value: Call) -> Self {
        Expression::Call(value).into()
    }
}

impl Parse<'_> for Call {
    fn parse(_parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        todo!()
    }
}
