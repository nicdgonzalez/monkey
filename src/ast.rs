use crate::Token;

pub enum Statement {
    Let(Token, Expression),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Let(token, ..) => write!(f, "{}", token),
        }
    }
}

pub enum Expression {
    Identifier(Token, String),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(token, ..) => write!(f, "{}", token),
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
