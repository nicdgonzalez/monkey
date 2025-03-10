use std::rc::Rc;

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(Token, Expression, Expression),
    Return(Token, Expression),
    Expression(Token, Expression),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Let(token, ..) => write!(f, "{}", token),
            Self::Return(token, ..) => write!(f, "{}", token),
            Self::Expression(token, ..) => write!(f, "{}", token),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Token),
    IntegerLiteral(Token, i64),
    Prefix(Token, String, Rc<Expression>),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(token, ..) => write!(f, "{}", token),
            Self::IntegerLiteral(token, ..) => write!(f, "{}", token),
            Self::Prefix(token, ..) => write!(f, "{}", token),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements.iter().nth(0).unwrap().to_string()
        } else {
            "".to_string()
        }
    }
}
