//! # Token
//!
//! This module defines the different types of tokens that can be produced by
//! the lexer.
//!
//! Tokens are the basic building blocks of the Monkey language, and are used
//! to represent keywords, identifiers, literals, operators, and other elements
//! of the language.
//!
//! The token types defined in this module are used by the lexer to categorize
//! the input source code, and are then used by the parser to construct the
//! abstract syntax tree (AST) representation of the code.
//!
//! ## Token Types
//!
//! The following token types are defined in this module:
//!
//! * Special tokens (e.g., invalid text, end of file)
//! * Identifiers (e.g. variable names, function names)
//! * Literals (e.g. integers, strings, booleans)
//! * Operators (e.g. `+`, `-`, `*`, `/`)
//! * Delimiters (e.g. `(`, `)`, `{`, `}`)
//! * Keywords (e.g. `let`, `fn`, `if`)

/// Represents the type of token produced by the lexer.
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// An invalid or unrecognized token.
    Illegal,
    /// The end of the input source code.
    EndOfFile,
    /// A variable or function name.
    Identifier,
    /// A whole number literal (e.g., `1`, `2`, `3`, etc.).
    Integer,
    /// The assignment operator (`=`).
    Assign,
    /// The addition operator (`+`).
    Plus,
    /// The subtraction operator (`-`).
    Minus,
    /// The logical NOT operator (`!`).
    Bang,
    /// The multiplication operator (`*`).
    Asterisk,
    /// The division operator (`/`).
    Slash,
    /// The less-than operator (`<`).
    LessThan,
    /// The greater-than operator (`<`).
    GreaterThan,
    /// The equality operator (`==`).
    Equal,
    /// The inequality operator (`!=`).
    NotEqual,
    /// The less-than-or-equal operator (`<=`).
    LessOrEqual,
    /// The greater-than-or-equal operator (`>=`).
    GreaterOrEqual,
    /// The comma delimiter (`,`).
    Comma,
    /// Marks the end of a statement (`;`).
    Semicolon,
    /// The opening parenthesis (`(`).
    LParenthesis,
    /// The closing parenthesis (`)`).
    RParenthesis,
    /// Marks the start of a block (`{`).
    LBrace,
    /// Marks the end of a block (`}`).
    RBrace,
    /// The `fn` keyword, used to declare functions.
    Function,
    /// The `let` keyword, used to declare variables.
    Let,
    /// The `true` keyword, representing a boolean true value.
    True,
    /// The `false` keyword, representing a boolean false value.
    False,
    /// The `if` keyword, used for conditional statements.
    If,
    /// The `else` keyword, used for conditional statements.
    Else,
    /// The `return` keyword, used to exit functions and return values.
    Return,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Illegal => write!(f, "ILLEGAL"),
            TokenKind::EndOfFile => write!(f, "EOF"),
            TokenKind::Identifier => write!(f, "IDENTIFIER"),
            TokenKind::Integer => write!(f, "INTEGER"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::GreaterThan => write!(f, ">"),
            TokenKind::Equal => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::LessOrEqual => write!(f, "<="),
            TokenKind::GreaterOrEqual => write!(f, ">="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::LParenthesis => write!(f, "("),
            TokenKind::RParenthesis => write!(f, ")"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::Function => write!(f, "fn"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Return => write!(f, "return"),
        }
    }
}

/// Represents a single token in the Monkey source code.
#[derive(Debug, PartialEq)]
pub struct Token {
    /// The type of token, such as identifier, integer, or operator.
    pub kind: TokenKind,
    /// The original text of the token, as it appeared in the source code.
    pub literal: String,
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        let literal = value.to_string();
        let kind = match value.as_ref() {
            "==" => TokenKind::Equal,
            "!=" => TokenKind::NotEqual,
            "<=" => TokenKind::LessOrEqual,
            ">=" => TokenKind::GreaterOrEqual,
            "fn" => TokenKind::Function,
            "let" => TokenKind::Let,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            _ => {
                if value.chars().all(|c| c.is_ascii_digit()) {
                    TokenKind::Integer
                } else {
                    TokenKind::Identifier
                }
            }
        };

        Self { kind, literal }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Token::from(value.as_ref())
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        let literal = value.to_string();
        let kind = match value {
            '=' => TokenKind::Assign,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '!' => TokenKind::Bang,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '<' => TokenKind::LessThan,
            '>' => TokenKind::GreaterThan,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '(' => TokenKind::LParenthesis,
            ')' => TokenKind::RParenthesis,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '\0' => TokenKind::EndOfFile,
            _ => TokenKind::Illegal,
        };

        Self { kind, literal }
    }
}
