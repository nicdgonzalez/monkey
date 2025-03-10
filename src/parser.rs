use std::collections::HashMap;

use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

type PrefixParseFn<'a> = fn(&mut Parser<'a>) -> Expression;
type InfixParseFn<'a> = fn(&mut Parser<'a>, Expression) -> Expression;

pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer<'a>,
    pub errors: Vec<String>,
    pub current: Token,
    pub next: Token,

    pub prefix: HashMap<TokenKind, PrefixParseFn<'a>>,
    pub infix: HashMap<TokenKind, InfixParseFn<'a>>,
}

#[derive(Debug)]
pub enum ParserError {
    WrongToken(TokenKind, TokenKind),
    MissingPrefixFn(TokenKind),
}

impl std::error::Error for ParserError {}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongToken(expected, actual) => {
                write!(f, "expected next token to be {}, got {}", expected, actual)
            }
            Self::MissingPrefixFn(kind) => write!(
                f,
                "expected token {} to have a prefix parsing function",
                kind
            ),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        // Start the parser in a working state.
        let current = lexer.next_token();
        let next = lexer.next_token();
        let errors = Vec::<String>::new();

        let prefix = HashMap::<TokenKind, PrefixParseFn>::new();

        let infix = HashMap::<TokenKind, InfixParseFn>::new();

        let mut parser = Self {
            lexer,
            current,
            next,
            errors,
            prefix,
            infix,
        };

        parser.insert_prefix_fn(TokenKind::Identifier, Parser::parse_identifier);
        parser.insert_prefix_fn(TokenKind::Integer, Parser::parse_integer_literal);

        parser
    }

    fn insert_prefix_fn(&mut self, kind: TokenKind, func: PrefixParseFn<'a>) -> () {
        if let Some(_) = self.prefix.insert(kind, func) {
            panic!("expected keys to be unique");
        }
    }

    fn next_token(&mut self) -> () {
        self.current = self.next.to_owned();
        self.next = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current != Token::from('\0') {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(err) => self.errors.push(err.to_string()),
            }
            self.next_token();
        }

        println!("errors: {:#?}", self.errors);

        program
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current.kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        assert!(self.current.kind == TokenKind::Let);
        let token = self.current.clone();

        let token_next = self.next.clone(); // To avoid borrow issues.
        let name = match token_next.kind {
            TokenKind::Identifier => {
                self.next_token();
                Ok(Expression::Identifier(token_next))
            }
            _ => Err(ParserError::WrongToken(
                TokenKind::Identifier,
                token_next.kind,
            )),
        }?;

        let token_next = self.next.clone(); // To avoid borrow issues.
        if let TokenKind::Assign = &self.next.kind {
            self.next_token();
        } else {
            return Err(ParserError::WrongToken(TokenKind::Assign, token_next.kind));
        }

        // TODO: Handle parsing expressions later.
        while self.current.kind != TokenKind::Semicolon {
            self.next_token();
        }

        let value = "".to_string();

        Ok(Statement::Let(
            token,
            name,
            Expression::Identifier(Token {
                kind: TokenKind::Identifier,
                literal: value,
            }),
        ))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        assert!(self.current.kind == TokenKind::Return);
        self.next_token();

        // TODO: Handle parsing expressions later.
        while self.current.kind != TokenKind::Semicolon {
            self.next_token();
        }

        let _value = "".to_string();

        todo!();
        // Statement::Return(Token::Return, value)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let token = self.current.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.next.kind == TokenKind::Semicolon {
            self.next_token();
        }

        let statement = Statement::Expression(token, expression);
        Ok(statement)
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Result<Expression, ParserError> {
        let left = match self.prefix.get(&self.current.kind) {
            Some(func) => func(self),
            None => return Err(ParserError::MissingPrefixFn(self.current.kind.clone())),
        };

        Ok(left)
    }

    fn parse_identifier(&mut self) -> Expression {
        if let TokenKind::Identifier = &self.current.kind {
            let expression = Expression::Identifier(self.current.clone());
            expression
        } else {
            panic!("expected Token::Identifier, got {}", self.current);
        }
    }

    fn parse_integer_literal(&mut self) -> Expression {
        if let TokenKind::Integer = &self.current.kind {
            Expression::IntegerLiteral(
                self.current.clone(),
                self.current.to_string().parse::<i64>().unwrap(),
            )
        } else {
            panic!("expected Token::Integer, got {}", self.current);
        }
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
                Token::from("let"),
                Expression::Identifier(Token::from("x")),
                Expression::IntegerLiteral(Token::from("5"), 5),
            ),
            Statement::Let(
                Token::from("let"),
                Expression::Identifier(Token::from("y")),
                Expression::IntegerLiteral(Token::from("10"), 10),
            ),
            Statement::Let(
                Token::from("let"),
                Expression::Identifier(Token::from("foo")),
                Expression::IntegerLiteral(Token::from("69"), 69),
            ),
        ];

        for (i, expected_token) in expected_tokens.iter().enumerate() {
            assert_eq!(program.statements.iter().nth(i), Some(expected_token));
        }
    }

    #[test]
    fn test_return_statements() -> () {
        let input = r#"
            return 5;
            return 10;
            return 69;
        "#;

        let mut lexer = Lexer::new(&input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 3);

        let expected_tokens: &[Statement] = &[
            Statement::Return(
                Token::from("return"),
                Expression::IntegerLiteral(Token::from("5"), 5),
            ),
            Statement::Return(
                Token::from("return"),
                Expression::IntegerLiteral(Token::from("10"), 10),
            ),
            Statement::Return(
                Token::from("return"),
                Expression::IntegerLiteral(Token::from("69"), 69),
            ),
        ];

        for (i, expected_token) in expected_tokens.iter().enumerate() {
            assert_eq!(program.statements.iter().nth(i), Some(expected_token));
        }
    }

    #[test]
    fn test_identifier_expressions() -> () {
        let input = r#"
            foo;
        "#;

        let mut lexer = Lexer::new(&input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 1);

        let expected_tokens: &[Statement] = &[Statement::Expression(
            Token::from("foo"),
            Expression::Identifier(Token::from("foo")),
        )];

        for (i, expected_token) in expected_tokens.iter().enumerate() {
            assert_eq!(program.statements.iter().nth(i), Some(expected_token));
        }
    }
}
