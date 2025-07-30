use std::collections::HashMap;
use std::{error, fmt};

use crate::ast::Node;
use crate::expression::{
    Boolean, Call, Expression, FunctionLiteral, Grouped, Identifier, If, Infix, IntegerLiteral,
    Prefix,
};
use crate::lexer::Tokens;
use crate::token::{Token, TokenKind};

pub trait Parse {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError>;
}

pub trait ParsePrefix {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<Expression, ParserError>;
}

pub trait ParseInfix {
    fn parse_infix(parser: &mut Parser<'_>, left: Expression) -> Result<Expression, ParserError>;
}

type ParsePrefixFn = fn(&mut Parser<'_>) -> Result<Expression, ParserError>;
type ParseInfixFn = fn(&mut Parser<'_>, Expression) -> Result<Expression, ParserError>;

lazy_static::lazy_static! {
    pub static ref PREFIX: HashMap<TokenKind, ParsePrefixFn> = HashMap::from([
        (TokenKind::Identifier, Identifier::parse_prefix as ParsePrefixFn),
        (TokenKind::Integer, IntegerLiteral::parse_prefix as ParsePrefixFn),
        (TokenKind::Bang, Prefix::parse_prefix as ParsePrefixFn),
        (TokenKind::Minus, Prefix::parse_prefix as ParsePrefixFn),
        (TokenKind::True, Boolean::parse_prefix as ParsePrefixFn),
        (TokenKind::False, Boolean::parse_prefix as ParsePrefixFn),
        (TokenKind::LParenthesis, Grouped::parse_prefix as ParsePrefixFn),
        (TokenKind::If, If::parse_prefix as ParsePrefixFn),
        (TokenKind::Function, FunctionLiteral::parse_prefix as ParsePrefixFn),
    ]);

    pub static ref INFIX: HashMap<TokenKind, ParseInfixFn> = HashMap::from([
        (TokenKind::Plus, Infix::parse_infix as ParseInfixFn),
        (TokenKind::Minus, Infix::parse_infix as ParseInfixFn),
        (TokenKind::Slash, Infix::parse_infix as ParseInfixFn),
        (TokenKind::Asterisk, Infix::parse_infix as ParseInfixFn),
        (TokenKind::Equal, Infix::parse_infix as ParseInfixFn),
        (TokenKind::NotEqual, Infix::parse_infix as ParseInfixFn),
        (TokenKind::LessThan, Infix::parse_infix as ParseInfixFn),
        (TokenKind::GreaterThan, Infix::parse_infix as ParseInfixFn),
        (TokenKind::LParenthesis, Call::parse_infix as ParseInfixFn),
    ]);
}

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Tokens<'a>,
    token: Option<Token>,
    peek: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(mut tokens: Tokens<'a>) -> Self {
        let token = tokens.next();
        let peek = tokens.next();

        Self {
            tokens,
            token,
            peek,
        }
    }

    pub fn tokens(&self) -> &Tokens<'a> {
        &self.tokens
    }
}

impl Parser<'_> {
    pub const fn token(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    pub const fn peek(&self) -> Option<&Token> {
        self.peek.as_ref()
    }

    pub fn advance(&mut self) {
        self.token = self.peek.take();
        self.peek = self.tokens.next();
    }

    pub fn expect_token_with_kind(&mut self, expected: TokenKind) -> Result<Token, ParserError> {
        let actual = self.token.as_ref().map(|token| token.kind());

        if actual == Some(expected) {
            let token = self.token.take().unwrap();
            self.advance();
            Ok(token)
        } else {
            Err(ParserError::WrongTokenKind { expected, actual })
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    WrongTokenKind {
        expected: TokenKind,
        actual: Option<TokenKind>,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::WrongTokenKind { expected, actual } => {
                write!(f, "expected token kind {expected:?}, got {actual:?}")
            }
        }
    }
}

impl error::Error for ParserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::WrongTokenKind { .. } => None,
        }
    }
}
