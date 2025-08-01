use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Illegal,
    // EndOfFile,
    Identifier,
    Integer,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Comma,
    Semicolon,
    LParenthesis,
    RParenthesis,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }

    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        let kind = match value.as_ref() {
            // "" => TokenKind::EndOfFile,
            "=" => TokenKind::Assign,
            "+" => TokenKind::Plus,
            "-" => TokenKind::Minus,
            "!" => TokenKind::Bang,
            "*" => TokenKind::Asterisk,
            "/" => TokenKind::Slash,
            "<" => TokenKind::LessThan,
            ">" => TokenKind::GreaterThan,
            "==" => TokenKind::Equal,
            "!=" => TokenKind::NotEqual,
            "<=" => TokenKind::LessThanOrEqual,
            ">=" => TokenKind::GreaterThanOrEqual,
            "," => TokenKind::Comma,
            ";" => TokenKind::Semicolon,
            "(" => TokenKind::LParenthesis,
            ")" => TokenKind::RParenthesis,
            "{" => TokenKind::LBrace,
            "}" => TokenKind::RBrace,
            "fn" => TokenKind::Function,
            "let" => TokenKind::Let,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            _ if is_valid_integer(&value) => TokenKind::Integer,
            _ if is_valid_identifier(&value) => TokenKind::Identifier,
            _ => TokenKind::Illegal,
        };

        Token::new(kind, value)
    }
}

fn is_valid_integer(value: &str) -> bool {
    value.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        Token::from(value.to_string())
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Illegal => "ILLEGAL".fmt(f),
            // Self::EndOfFile => "EOF".fmt(f),
            Self::Identifier => "IDENTIFIER".fmt(f),
            Self::Integer => "INTEGER".fmt(f),
            Self::Assign => "=".fmt(f),
            Self::Plus => "+".fmt(f),
            Self::Minus => "-".fmt(f),
            Self::Bang => "!".fmt(f),
            Self::Asterisk => "*".fmt(f),
            Self::Slash => "/".fmt(f),
            Self::LessThan => "<".fmt(f),
            Self::GreaterThan => ">".fmt(f),
            Self::Equal => "==".fmt(f),
            Self::NotEqual => "!=".fmt(f),
            Self::LessThanOrEqual => "<=".fmt(f),
            Self::GreaterThanOrEqual => ">=".fmt(f),
            Self::Comma => ",".fmt(f),
            Self::Semicolon => ";".fmt(f),
            Self::LParenthesis => "(".fmt(f),
            Self::RParenthesis => ")".fmt(f),
            Self::LBrace => "{".fmt(f),
            Self::RBrace => "}".fmt(f),
            Self::Function => "fn".fmt(f),
            Self::Let => "let".fmt(f),
            Self::True => "true".fmt(f),
            Self::False => "false".fmt(f),
            Self::If => "if".fmt(f),
            Self::Else => "else".fmt(f),
            Self::Return => "return".fmt(f),
        }
    }
}
