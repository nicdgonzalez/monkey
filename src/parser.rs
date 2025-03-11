use std::collections::HashMap;
use std::rc::Rc;

use lazy_static::lazy_static;

use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

type PrefixParseFn<'a> = fn(&mut Parser<'a>) -> Result<Expression, ParserError>;
type InfixParseFn<'a> = fn(&mut Parser<'a>, Expression) -> Result<Expression, ParserError>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

lazy_static! {
    static ref PRECEDENCES: HashMap<TokenKind, Precedence> = HashMap::from([
        (TokenKind::Equal, Precedence::Equals),
        (TokenKind::NotEqual, Precedence::Equals),
        (TokenKind::LessThan, Precedence::LessGreater),
        (TokenKind::GreaterThan, Precedence::LessGreater),
        (TokenKind::Plus, Precedence::Sum),
        (TokenKind::Minus, Precedence::Sum),
        (TokenKind::Slash, Precedence::Product),
        (TokenKind::Asterisk, Precedence::Product),
    ]);
}

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer<'a>,
    pub current: Token,
    pub next: Token,

    pub prefix: HashMap<TokenKind, PrefixParseFn<'a>>,
    pub infix: HashMap<TokenKind, InfixParseFn<'a>>,
}

#[derive(Debug)]
pub enum ParserError {
    WrongToken(TokenKind, TokenKind),
    MissingPrefixFn(TokenKind),
    MissingInfixFn(TokenKind),
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
            Self::MissingInfixFn(kind) => write!(
                f,
                "expected token {} to have an infix parsing function",
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

        let mut parser = Self {
            lexer,
            current,
            next,
            prefix: HashMap::new(),
            infix: HashMap::new(),
        };

        // Registering prefix parsing functions.
        parser.register_prefix_fn(TokenKind::Identifier, Parser::parse_identifier);
        parser.register_prefix_fn(TokenKind::Integer, Parser::parse_integer_literal);
        parser.register_prefix_fn(TokenKind::Bang, Parser::parse_prefix_expression);
        parser.register_prefix_fn(TokenKind::Minus, Parser::parse_prefix_expression);
        parser.register_prefix_fn(TokenKind::True, Parser::parse_boolean);
        parser.register_prefix_fn(TokenKind::False, Parser::parse_boolean);
        parser.register_prefix_fn(TokenKind::LParenthesis, Parser::parse_grouped_expression);

        // Registering infix parsing functions.
        parser.register_infix_fn(TokenKind::Plus, Parser::parse_infix_expression);
        parser.register_infix_fn(TokenKind::Minus, Parser::parse_infix_expression);
        parser.register_infix_fn(TokenKind::Slash, Parser::parse_infix_expression);
        parser.register_infix_fn(TokenKind::Asterisk, Parser::parse_infix_expression);
        parser.register_infix_fn(TokenKind::Equal, Parser::parse_infix_expression);
        parser.register_infix_fn(TokenKind::NotEqual, Parser::parse_infix_expression);
        parser.register_infix_fn(TokenKind::LessThan, Parser::parse_infix_expression);
        parser.register_infix_fn(TokenKind::GreaterThan, Parser::parse_infix_expression);

        parser
    }

    fn register_prefix_fn(&mut self, kind: TokenKind, func: PrefixParseFn<'a>) {
        if let Some(_) = self.prefix.insert(kind, func) {
            panic!("expected keys to be unique");
        }
    }

    fn register_infix_fn(&mut self, kind: TokenKind, func: InfixParseFn<'a>) {
        if let Some(_) = self.infix.insert(kind, func) {
            panic!("expected keys to be unique");
        }
    }

    fn advance_current_token(&mut self) {
        self.current = self.next.to_owned();
        self.next = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current != Token::from('\0') {
            let result = self.parse_statement();
            match result {
                Ok(statement) => program.statements.push(statement),
                Err(err) => program.errors.push(err.to_string()),
            }
            self.advance_current_token();
        }

        // Debugging:
        println!("statements: {:#?}", program.statements);
        println!("errors: {:#?}", program.errors);

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
        let token = self.current.clone();
        self.advance_current_token();

        let name = match self.current.kind {
            TokenKind::Identifier => {
                let expression = Expression::Identifier(self.current.clone());
                self.advance_current_token();
                expression
            }
            _ => {
                return Err(ParserError::WrongToken(
                    TokenKind::Identifier,
                    self.current.kind.clone(),
                ))
            }
        };

        if self.current.kind == TokenKind::Assign {
            self.advance_current_token();
        } else {
            return Err(ParserError::WrongToken(
                TokenKind::Identifier,
                self.current.kind.clone(),
            ));
        }

        let value = self.parse_expression(Precedence::Lowest)?;
        self.advance_current_token();

        // TODO: Annoying bug; be clearer about whose responsibility it is to
        // advance the lexer. I would prefer if the parser function handles it.
        // For now, I'm trusting the book has a reason for doing it this way.
        // if self.current.kind == TokenKind::Semicolon {
        //     self.advance_current_token();
        // }

        let statement = Statement::Let(token, name, value);
        Ok(statement)
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        assert!(self.current.kind == TokenKind::Return);
        let token = self.current.clone();
        self.advance_current_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.next.kind == TokenKind::Semicolon {
            self.advance_current_token();
        }

        let statement = Statement::Return(token, value);
        Ok(statement)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        println!("Parse expression token: {}", self.current);
        let token = self.current.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;
        println!("Parse expression: {}", expression);

        if self.next.kind == TokenKind::Semicolon {
            self.advance_current_token();
        }

        let statement = Statement::Expression(token, expression);
        Ok(statement)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        // Parse the left side of the operator.
        let mut left = match self.prefix.get(&self.current.kind) {
            Some(f) => f(self)?,
            None => return Err(ParserError::MissingPrefixFn(self.current.kind.clone())),
        };
        println!("expression left: {:#?}", left);

        while self.next.kind != TokenKind::Semicolon && self.next.kind != TokenKind::EndOfFile {
            let peek_precedence = match PRECEDENCES.get(&self.next.kind) {
                Some(&ref p) => p,
                None => &Precedence::Lowest,
            };

            println!(
                "{:?} >= {:?}: {}",
                precedence,
                *peek_precedence,
                precedence >= *peek_precedence
            );
            if precedence >= *peek_precedence {
                break;
            }

            if let Some(&infix) = self.infix.get(&self.next.kind) {
                self.advance_current_token();
                left = infix(self, left)?;
                println!("new left: {:#}", left);
            } else {
                return Ok(left);
            }
        }

        Ok(left)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        assert!(self.current.kind == TokenKind::Minus || self.current.kind == TokenKind::Bang);
        let token = self.current.clone();
        self.advance_current_token();

        let right = self.parse_expression(Precedence::Prefix)?;

        let expression = Expression::Prefix(token, Rc::new(right));
        Ok(expression)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParserError> {
        let token = self.current.clone();
        let precedence = PRECEDENCES
            .get(&self.current.kind)
            .ok_or_else(|| ParserError::MissingInfixFn(token.kind.clone()))?;
        println!("infix precedence: {:?}", precedence);
        self.advance_current_token();

        let right = self.parse_expression(precedence.clone())?;
        println!("infix left expression: {:#?}", left);
        println!("infix right expression: {:#?}", right);

        let expression = Expression::Infix(token, Rc::new(left), Rc::new(right));
        println!("infix Expression: {}", expression);
        Ok(expression)
    }

    fn parse_identifier(&mut self) -> Result<Expression, ParserError> {
        let token = self.current.clone();
        if let TokenKind::Identifier = token.kind {
            let expression = Expression::Identifier(token);
            Ok(expression)
        } else {
            return Err(ParserError::WrongToken(TokenKind::Identifier, token.kind));
        }
    }

    fn parse_integer_literal(&mut self) -> Result<Expression, ParserError> {
        let token = self.current.clone();
        if let TokenKind::Integer = token.kind {
            let value = token.to_string().parse::<i64>().unwrap();
            let expression = Expression::IntegerLiteral(token, value);
            Ok(expression)
        } else {
            return Err(ParserError::WrongToken(TokenKind::Integer, token.kind));
        }
    }

    fn parse_boolean(&mut self) -> Result<Expression, ParserError> {
        assert!(self.current.kind == TokenKind::True || self.current.kind == TokenKind::False);
        let value = self.current.kind == TokenKind::True;
        let expression = Expression::Boolean(self.current.clone(), value);
        Ok(expression)
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.advance_current_token();
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.next.kind != TokenKind::RParenthesis {
            return Err(ParserError::WrongToken(
                TokenKind::RParenthesis,
                self.current.kind.clone(),
            ));
        }

        // ... don't know why this worked, but it did.
        self.advance_current_token();

        Ok(expression)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Statement};

    use super::*;

    #[test]
    fn test_let_statements() {
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
    fn test_return_statements() {
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
    fn test_identifier_expressions() {
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

    #[test]
    fn test_operator_precedence() {
        let tests: &[(&str, &str)] = &[
            (("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)")),
            (("(5 + 5) * 2", "((5 + 5) * 2)")),
            (("2 / (5 + 5)", "(2 / (5 + 5))")),
            (("-(5 + 5)", "(-(5 + 5))")),
            (("!(true == true)", "(!(true == true))")),
        ];

        for (input, expected) in tests.iter() {
            let mut lexer = Lexer::new(&input);
            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse_program();
            assert_eq!(program.to_string(), *expected);
        }
    }
}
