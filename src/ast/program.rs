use crate::token::TokenKind;

use super::parser::{Parse, Parser, ParserError};
use super::statement;
use super::Node;

pub struct Program {
    pub statements: Vec<statement::Statement>,
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

impl Parse<'_> for Program {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        let mut program = Self::new();

        while parser.token.kind != TokenKind::EndOfFile {
            let result = match parser.token.kind {
                TokenKind::Let => statement::Let::parse(parser),
                TokenKind::Return => statement::Return::parse(parser),
                _ => statement::Expression::parse(parser),
            };

            match result {
                Ok(statement) => program.statements.push(statement.into()),
                Err(err) => program.errors.push(err.to_string()),
            }

            parser.advance();
        }

        eprintln!("AST (Abstract Syntax Tree): {:#?}", program.statements);

        Ok(program.into())
    }
}
