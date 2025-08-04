use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::object::{NULL, Object};
use crate::parser::{Parse, Parser, ParserError};
use crate::statement::Statement;

#[derive(Debug, Default)]
pub struct Program {
    statements: Vec<Statement>,
    errors: Vec<String>,
}

impl Program {
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    pub fn parse(parser: &mut Parser<'_>) -> Result<Self, ParserError> {
        let mut program = Self::default();

        while parser.token().is_some() {
            match Statement::parse(parser) {
                Ok(statement) => program.statements.push(statement),
                Err(err) => program.errors.push(err.to_string()),
            }

            parser.advance();
        }

        Ok(program)
    }
}

impl Evaluate for Program {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let mut result = NULL;

        for statement in &self.statements {
            result = statement.evaluate(env);

            match result {
                Object::Return(inner) => return inner.value().to_owned(),
                Object::Error(_) => return result,
                _ => continue,
            }
        }

        result
    }
}
