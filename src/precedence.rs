use std::collections::HashMap;

use crate::token::TokenKind;

lazy_static::lazy_static! {
    pub static ref PRECEDENCES: HashMap<TokenKind, Precedence> = HashMap::from([
        (TokenKind::Equal, Precedence::Equals),
        (TokenKind::NotEqual, Precedence::Equals),
        (TokenKind::LessThan, Precedence::LessGreater),
        (TokenKind::GreaterThan, Precedence::LessGreater),
        (TokenKind::Plus, Precedence::Sum),
        (TokenKind::Minus, Precedence::Sum),
        (TokenKind::Slash, Precedence::Product),
        (TokenKind::Asterisk, Precedence::Product),
        (TokenKind::LParenthesis, Precedence::Call),
    ]);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}
