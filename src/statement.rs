use core::panic;

use crate::ast::Node;
use crate::expression;
use crate::parser::{
    parse_expression, parse_statement, Parse, ParserContext, ParserError, Precedence,
};
use crate::token::{Token, TokenKind};

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

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Let(inner) => write!(f, "{}", inner),
            Self::Return(inner) => write!(f, "{}", inner),
            Self::Expression(inner) => write!(f, "{}", inner),
            Self::Block(inner) => write!(f, "{}", inner),
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

impl From<Statement> for Let {
    fn from(value: Statement) -> Self {
        value.into()
    }
}

impl From<Let> for Node {
    fn from(value: Let) -> Self {
        Statement::Let(value).into()
    }
}

impl From<Node> for Let {
    fn from(value: Node) -> Self {
        Into::<Statement>::into(value).into()
    }
}

impl Parse for Let {
    fn parse(parser: &mut ParserContext<'_>) -> Result<Node, ParserError> {
        // Parse "let" keyword.
        assert_eq!(parser.token.kind, TokenKind::Let);
        let token = parser.token.clone();
        parser.advance();

        // Parse identifier.
        let name = parser
            .expect_token(TokenKind::Identifier)
            .map(|token| expression::Identifier { token })?;
        parser.advance();

        // Parse assignment operator.
        _ = parser.expect_token(TokenKind::Assign)?;
        parser.advance();

        // Parse value expression.
        let value = super::parser::parse_expression(parser, Precedence::Lowest)?;
        parser.advance();

        // Enforce semicolon.
        _ = parser.expect_token(TokenKind::Semicolon)?;

        let statement = Self {
            token,
            name,
            value: Box::new(value),
        };
        Ok(statement.into())
    }
}

impl std::fmt::Display for Let {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} = {};", self.token, self.name, self.value)
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

impl From<Statement> for Return {
    fn from(value: Statement) -> Self {
        value.into()
    }
}

impl From<Return> for Node {
    fn from(value: Return) -> Self {
        Statement::Return(value).into()
    }
}

impl From<Node> for Return {
    fn from(value: Node) -> Self {
        Into::<Statement>::into(value).into()
    }
}

impl Parse for Return {
    fn parse(parser: &mut ParserContext<'_>) -> Result<Node, ParserError> {
        // Parse "return" keyword.
        debug_assert_eq!(parser.token.kind, TokenKind::Return);
        let token = parser.token.clone();
        parser.advance();

        // Parse value expression.
        let value = parse_expression(parser, Precedence::Lowest)?;
        parser.advance();

        // TODO: Why advance if semicolon here, but not in `Let::parse`?
        if parser.peek.kind == TokenKind::Semicolon {
            parser.advance();
        }

        let statement = Self {
            token,
            value: Box::new(value),
        };
        Ok(statement.into())
    }
}

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {};", self.token, self.value)
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

impl From<Statement> for Expression {
    fn from(value: Statement) -> Self {
        value.into()
    }
}

impl From<Expression> for Node {
    fn from(value: Expression) -> Self {
        Statement::Expression(value).into()
    }
}

impl From<Node> for Expression {
    fn from(value: Node) -> Self {
        Into::<Statement>::into(value).into()
    }
}

impl Parse for Expression {
    fn parse(parser: &mut ParserContext<'_>) -> Result<Node, ParserError> {
        let token = parser.token.clone();
        let expr = parse_expression(parser, Precedence::Lowest)?;

        if parser.peek.kind == TokenKind::Semicolon {
            parser.advance();
        }

        let statement = Self {
            token,
            expression: Box::new(expr),
        };
        Ok(statement.into())
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
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

impl From<Statement> for Block {
    fn from(value: Statement) -> Self {
        match value {
            Statement::Block(inner) => inner,
            _ => panic!("expected statement to be block"),
        }
    }
}

impl From<Block> for Node {
    fn from(value: Block) -> Self {
        Statement::Block(value).into()
    }
}

impl From<Node> for Block {
    fn from(value: Node) -> Self {
        Into::<Statement>::into(value).into()
    }
}

impl Parse for Block {
    fn parse(parser: &mut ParserContext<'_>) -> Result<Node, ParserError> {
        debug_assert_eq!(parser.token.kind, TokenKind::LBrace);
        let token = parser.token.clone();
        parser.advance();

        let mut statements = Vec::<Statement>::new();

        // Loop over statements until we reach the closing brace.
        while parser.token.kind != TokenKind::EndOfFile {
            if parser.token.kind == TokenKind::RBrace {
                break;
            }

            match parse_statement(parser) {
                Ok(statement) => statements.push(statement),
                Err(_) => (), // TODO: The book doesn't handle errors here.
            }

            parser.advance();
        }

        let statement = Self { token, statements };
        Ok(statement.into())
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = self
            .statements
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{}", statements)
    }
}
