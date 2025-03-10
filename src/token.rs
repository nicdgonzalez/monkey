#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Illegal,
    EndOfFile,

    // Identifiers and literals
    Identifier,
    Integer,

    // Operators
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
    LessOrEqual,
    GreaterOrEqual,

    // Delimiters
    Comma,
    Semicolon,

    LParenthesis,
    RParenthesis,
    LBrace,
    RBrace,

    // Keywords
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
    pub kind: TokenKind,
    pub literal: String,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
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

        Self {
            kind,
            literal: value.to_string(),
        }
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        let kind = match value {
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

        Self {
            kind,
            literal: value.to_string(),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal)
    }
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
