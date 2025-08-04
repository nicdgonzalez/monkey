use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::object::{NULL, Object};
use crate::parser::{Parse, Parser, ParserError};
use crate::statement::Statement;
use crate::token::TokenKind;

#[derive(Debug, Clone)]
pub struct Block {
    statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

impl Parse for Block {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, ParserError> {
        _ = parser.expect_token_with_kind(TokenKind::LBrace)?;

        let mut statements = Vec::new();

        while parser
            .token()
            .is_some_and(|token| token.kind() != TokenKind::RBrace)
        {
            match Statement::parse(parser) {
                Ok(statement) => statements.push(statement),
                Err(err) => tracing::error!("{err}"), // TODO: The book doesn't handle errors here.
            }

            // parser.advance();
        }

        // TODO: Better error message would be nice. Forgetting to close the curly braces should
        // result in a nice syntax error.
        _ = parser.expect_token_with_kind(TokenKind::RBrace)?;

        Ok(Self::new(statements))
    }
}

impl Evaluate for Block {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let mut value = NULL;

        for statement in &self.statements {
            value = statement.evaluate(env);

            if matches!(value, Object::Return(_) | Object::Error(_)) {
                break;
            }
        }

        value
    }
}
