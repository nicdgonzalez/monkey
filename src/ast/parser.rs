use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

use super::Node;

pub trait Parse<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Node, ParserError>;
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    pub token: Token,
    pub next_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        // Start the parser in a working state.
        let token = lexer.get_next_token();
        let next_token = lexer.get_next_token();

        Self {
            lexer,
            token,
            next_token,
        }
    }

    /// Update fields `current` and `next` with the next two lexer tokens.
    pub fn advance(&mut self) {
        self.token = self.next_token.to_owned();
        self.next_token = self.lexer.get_next_token();
    }

    pub fn expect_token(&self, expected: TokenKind) -> Result<Token, ParserError> {
        if self.token.kind == expected {
            Ok(self.token.clone())
        } else {
            Err(ParserError::WrongToken(expected, self.token.kind.clone()))
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    WrongToken(TokenKind, TokenKind),
}

impl std::error::Error for ParserError {}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongToken(expected, actual) => {
                write!(f, "expected next token to be {}, got {}", expected, actual)
            }
        }
    }
}
