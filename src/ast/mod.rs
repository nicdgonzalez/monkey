//! Implementation for the Abstract Syntax Tree (AST).

pub mod expression;
pub mod parser;
pub mod program;
pub mod statement;

use expression::Expression;
use program::Program;
use statement::Statement;

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
