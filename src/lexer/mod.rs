//! # Lexer
//!
//! The lexer is responsible for tokenizing the Monkey source code.
//!
//! ## Examples
//!
//! ```
//! fn main() {
//!     let input = r#"
//!         let add = fn(x, y) {
//!             x + y
//!         };
//!     "#;
//!     let lexer = monkey::Lexer::new(&input);
//!
//!     for token in lexer.iter() {
//!         println!("{:?}", token);
//!     }
//! }
//! ```

pub mod token;

use std::iter::Peekable;
use std::str::Chars;

use token::Token;

/// Responsible for tokenizing the source code.
pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl Lexer<'_> {
    pub fn iter(&self) -> LexerIterator {
        LexerIterator::new(self)
    }
}

pub struct LexerIterator<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> LexerIterator<'a> {
    pub fn new(lexer: &'a Lexer<'a>) -> Self {
        Self {
            input: lexer.input.chars().peekable(),
        }
    }
}

impl Iterator for LexerIterator<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.input.peek()? {
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::from(self.input.next().unwrap());

                while self
                    .input
                    .peek()
                    .is_some_and(|&c| c.is_ascii_alphanumeric() || c == '_')
                {
                    identifier.push(self.input.next().unwrap());
                }

                let token = Token::from(identifier);
                Some(token)
            }
            '0'..='9' => {
                let mut literal = String::from(self.input.next().unwrap());

                while self.input.peek().is_some_and(|&c| c.is_ascii_digit()) {
                    literal.push(self.input.next().unwrap());
                }

                let token = Token::from(literal);
                Some(token)
            }
            '=' | '!' | '<' | '>' => {
                let mut literal = String::from(self.input.next().unwrap());
                let token: Token;

                if let Some(&'=') = self.input.peek() {
                    literal.push(self.input.next().unwrap());
                    token = Token::from(literal);
                } else {
                    debug_assert_eq!(literal.len(), 1);
                    token = Token::from(literal.chars().nth(0).unwrap());
                }

                Some(token)
            }
            _ => {
                let token = Token::from(self.input.next().unwrap());
                Some(token)
            }
        }
    }
}

impl LexerIterator<'_> {
    /// Advances the input iterator over consecutive whitespace characters.
    ///
    /// This function repeatedly checks the next character in the input and
    /// advances the iterator as it encounters ASCII whitespace characters.
    fn skip_whitespace(&mut self) {
        while self.input.peek().is_some_and(|c| c.is_ascii_whitespace()) {
            _ = self.input.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenKind;

    #[test]
    fn test_next() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        "#;

        // Expected tokens {{{
        let expected_tokens = &[
            ///////////////////////////////////////////////////////////////////
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            ///////////////////////////////////////////////////////////////////
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            ///////////////////////////////////////////////////////////////////
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Function,
                literal: "fn".to_string(),
            },
            Token {
                kind: TokenKind::LParenthesis,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::RParenthesis,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::LBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::RBrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            ///////////////////////////////////////////////////////////////////
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "result".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::LParenthesis,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::RParenthesis,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            ///////////////////////////////////////////////////////////////////
            Token {
                kind: TokenKind::Bang,
                literal: "!".to_string(),
            },
            Token {
                kind: TokenKind::Minus,
                literal: "-".to_string(),
            },
            Token {
                kind: TokenKind::Slash,
                literal: "/".to_string(),
            },
            Token {
                kind: TokenKind::Asterisk,
                literal: "*".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            ///////////////////////////////////////////////////////////////////
            Token {
                kind: TokenKind::Integer,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::LessThan,
                literal: "<".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::GreaterThan,
                literal: ">".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            ///////////////////////////////////////////////////////////////////
            Token {
                kind: TokenKind::If,
                literal: "if".to_string(),
            },
            Token {
                kind: TokenKind::LParenthesis,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::LessThan,
                literal: "<".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::RParenthesis,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::LBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Return,
                literal: "return".to_string(),
            },
            Token {
                kind: TokenKind::True,
                literal: "true".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::RBrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Else,
                literal: "else".to_string(),
            },
            Token {
                kind: TokenKind::LBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Return,
                literal: "return".to_string(),
            },
            Token {
                kind: TokenKind::False,
                literal: "false".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::RBrace,
                literal: "}".to_string(),
            },
            ///////////////////////////////////////////////////////////////////
            Token {
                kind: TokenKind::Integer,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::Equal,
                literal: "==".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            ///////////////////////////////////////////////////////////////////
            Token {
                kind: TokenKind::Integer,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::NotEqual,
                literal: "!=".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
                literal: "9".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            ///////////////////////////////////////////////////////////////////
        ];
        // }}}

        let lexer = Lexer::new(&input);
        let mut expected_tokens_it = expected_tokens.iter();

        for token in lexer.iter() {
            let expected_token = expected_tokens_it
                .next()
                .expect("expected test to cover all of the test input");

            assert_eq!(token, *expected_token);
        }
    }
}
