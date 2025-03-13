//! Implementation for the Abstract Syntax Tree (AST).

use crate::expression::Expression;
use crate::parser::{parse_statement, Parse, Parser, ParserError};
use crate::statement::Statement;
use crate::token::TokenKind;

pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

impl From<Program> for Node {
    fn from(value: Program) -> Self {
        Self::Program(value)
    }
}

impl From<Statement> for Node {
    fn from(value: Statement) -> Self {
        Self::Statement(value)
    }
}

impl From<Expression> for Node {
    fn from(value: Expression) -> Self {
        Self::Expression(value)
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
    pub errors: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            errors: Vec::new(),
        }
    }
}

impl From<Node> for Program {
    fn from(value: Node) -> Self {
        match value {
            Node::Program(program) => program,
            _ => panic!("expected node to be a program"),
        }
    }
}

impl Parse for Program {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        let mut program = Self::new();

        while parser.token.kind != TokenKind::EndOfFile {
            match parse_statement(parser) {
                Ok(statement) => program.statements.push(statement.into()),
                Err(err) => program.errors.push(err.to_string()),
            }

            parser.advance();
        }

        // eprintln!("AST (Abstract Syntax Tree): {:#?}", program.statements);
        Ok(program.into())
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statement = if self.statements.len() > 0 {
            self.statements.iter().nth(0).unwrap().to_string()
        } else {
            "".to_string()
        };

        write!(f, "{}", statement)
    }
}
