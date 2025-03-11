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
            Self::Let(token, name, value) => write!(f, "{} {} = {};", token, name, value),
            Self::Return(token, value) => write!(f, "{} {}", token, value),
            Self::Expression(_, expression) => write!(f, "{}", expression),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Token),
    IntegerLiteral(Token, i64),
    Prefix(Token, Rc<Expression>),
    Infix(Token, Rc<Expression>, Rc<Expression>),
    Boolean(Token, bool),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(token, ..) => write!(f, "{}", token),
            Self::IntegerLiteral(token, ..) => write!(f, "{}", token),
            Self::Prefix(token, right) => write!(f, "({}{})", token, right),
            Self::Infix(token, left, right) => write!(f, "({} {} {})", left, token, right),
            Self::Boolean(_, value) => write!(f, "{}", value),
        }
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

    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements.iter().nth(0).unwrap().to_string()
        } else {
            "".to_string()
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statement = self
            .statements
            .iter()
            .nth(0)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "".to_string());

        write!(f, "{}", statement)
    }
}
