use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer<'a>,
    pub current: Token,
    pub next: Token,
    pub errors: Vec<String>,
}

#[derive(Debug)]
pub enum ParserError {
    WrongToken(Token, Token),
}

impl std::error::Error for ParserError {}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongToken(expected, actual) => {
                write!(f, "expected next token to be {expected}, got {actual}")
            }
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        // Start the parser in a working state.
        let current = lexer.next_token();
        let next = lexer.next_token();

        Self {
            lexer,
            current,
            next,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) -> () {
        self.current = self.next.to_owned();
        self.next = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current != Token::EndOfFile {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(err) => self.errors.push(err.to_string()),
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current {
            Token::Let => self.parse_let_statement(),
            _ => todo!(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let token = self.current.clone();

        let token_next = self.next.clone(); // To avoid borrow issues.

        let name = match token_next {
            Token::Identifier(n) => {
                self.next_token();
                Ok(n)
            }
            _ => Err(ParserError::WrongToken(
                Token::Identifier(String::new()),
                token_next,
            )),
        }?;

        let token_next = self.next.clone(); // To avoid borrow issues.

        if let Token::Assign = &self.next {
            self.next_token();
        } else {
            return Err(ParserError::WrongToken(Token::Assign, token_next));
        }

        // TODO: We will handle parsing expressions later
        while self.current != Token::Semicolon {
            self.next_token();
        }

        let value = "".to_string();

        Ok(Statement::Let(
            token,
            Expression::Identifier(Token::Identifier(name), value),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Statement};

    use super::*;

    #[test]
    fn test_let_statements() -> () {
        let input = r#"
            let x = 5;
            let y = 10;
            let foo = 69;
        "#;

        let mut lexer = Lexer::new(&input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 3);

        let expected_tokens: &[Statement] = &[
            Statement::Let(
                Token::Let,
                Expression::Identifier(Token::Identifier("x".to_string()), "5".to_string()),
            ),
            Statement::Let(
                Token::Let,
                Expression::Identifier(Token::Identifier("y".to_string()), "10".to_string()),
            ),
            Statement::Let(
                Token::Let,
                Expression::Identifier(Token::Identifier("foo".to_string()), "69".to_string()),
            ),
        ];

        for (i, expected_token) in expected_tokens.iter().enumerate() {
            assert_eq!(program.statements.iter().nth(i), Some(expected_token));
        }
    }
}
