use crate::token::Token;

pub trait Node {}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(Token, Expression, Expression),
    Return(Token, Expression),
    Expression(Token, Expression),
    Block(Token, Vec<Box<Statement>>),
}

impl Node for Statement {}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Let(token, name, value) => write!(f, "{} {} = {};", token, name, value),
            Self::Return(token, value) => write!(f, "{} {}", token, value),
            Self::Expression(_, expression) => write!(f, "{}", expression),
            Self::Block(_, block) => {
                let statements = block
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                write!(f, "{}", statements)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Token),
    IntegerLiteral(Token, i64),
    Prefix(Token, Box<Expression>),
    Infix(Token, Box<Expression>, Box<Expression>),
    Boolean(Token, bool),
    If(
        Token,                  // TokenKind::If
        Box<Expression>,        // Condition
        Box<Statement>,         // Consequence
        Box<Option<Statement>>, // Alternative
    ),
    FunctionLiteral(Token, Vec<Box<Expression>>, Box<Statement>),
    Call(Token, Box<Expression>, Vec<Box<Expression>>),
}

impl Node for Expression {}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(token, ..) => write!(f, "{}", token),
            Self::IntegerLiteral(token, ..) => write!(f, "{}", token),
            Self::Prefix(token, right) => write!(f, "({}{})", token, right),
            Self::Infix(token, left, right) => write!(f, "({} {} {})", left, token, right),
            Self::Boolean(_, value) => write!(f, "{}", value),
            Self::If(token, condition, consequence, alternative) => {
                let alt = if let Some(alternative) = alternative.as_ref() {
                    format!(" else {}", alternative)
                } else {
                    "".to_string()
                };

                write!(f, "{} {} {}{}", token, condition, consequence, alt)
            }
            Self::FunctionLiteral(token, parameters, body) => {
                let parameters = parameters
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(f, "{}({}) {{ {} }}", token, parameters, body)
            }
            Self::Call(_, function, arguments) => {
                let arguments = arguments
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(f, "{}({})", function, arguments)
            }
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
