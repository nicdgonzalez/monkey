use crate::ast::Node;
use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::object::{NULL, Object};
use crate::parser::{Parse, Parser, ParserError};
use crate::statement::Statement;
use crate::token::TokenKind;

#[derive(Debug, Default)]
pub struct Program {
    statements: Vec<Statement>,
    errors: Vec<String>,
}

impl Program {
    pub fn new(statements: Vec<Statement>, errors: Vec<String>) -> Self {
        Self { statements, errors }
    }

    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

impl Parse for Program {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError> {
        let mut program = Self::default();

        while parser.token().is_some() {
            match Statement::parse(parser) {
                Ok(node) => program.statements.push(
                    node.into_statement()
                        .expect("expected Statement::parse to return only statements"),
                ),
                Err(err) => program.errors.push(err.to_string()),
            }

            parser.advance();
        }

        Ok(program.into())
    }
}

impl Evaluate for Program {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let mut result = NULL;

        for statement in self.statements {
            result = statement.evaluate(env);

            match result {
                Object::Return(_) | Object::Error(_) => return result,
                _ => continue,
            }
        }

        result
    }
}
