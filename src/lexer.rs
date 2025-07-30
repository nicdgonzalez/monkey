use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
}

impl Lexer {
    pub const fn new(input: String) -> Self {
        Self { input }
    }
}

impl Lexer {
    pub fn tokens(&self) -> Tokens<'_> {
        Tokens {
            chars: self.input.chars().peekable(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tokens<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.chars.peek().is_some_and(|&c| c.is_whitespace()) {
            _ = self.chars.next();
        }

        let c = self.chars.next()?;

        match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut literal = c.to_string();

                while self
                    .chars
                    .peek()
                    .is_some_and(|&c| c.is_ascii_alphanumeric() || c == '_')
                {
                    literal.push(self.chars.next().unwrap());
                }

                Some(Token::from(literal))
            }
            '0'..='9' => {
                let mut literal = c.to_string();

                while self.chars.peek().is_some_and(|&c| c.is_ascii_digit()) {
                    literal.push(self.chars.next().unwrap());
                }

                Some(Token::from(literal))
            }
            '=' | '!' | '<' | '>' => {
                let mut literal = c.to_string();

                if let Some(&'=') = self.chars.peek() {
                    literal.push(self.chars.next().unwrap());
                }

                Some(Token::from(literal))
            }
            _ => Some(Token::from(c)),
        }
    }
}
