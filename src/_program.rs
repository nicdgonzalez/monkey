use crate::parser::parse_statement;
use crate::token::TokenKind;

use crate::ast::Node;
use crate::parser::{Parse, ParserContext, ParserError};
use crate::statement;

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

impl Parse for Program {
    fn parse(parser: &mut ParserContext<'_>) -> Result<Node, ParserError> {
        let mut program = Self::new();

        while parser.token.kind != TokenKind::EndOfFile {
            match parse_statement(parser) {
                Ok(statement) => program.statements.push(statement.into()),
                Err(err) => program.errors.push(err.to_string()),
            }

            parser.advance();
        }

        eprintln!("AST (Abstract Syntax Tree): {:#?}", program.statements);

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
