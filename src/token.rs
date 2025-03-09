#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(String),
    EndOfFile,

    // Identifiers and literals
    Identifier(String),
    Integer(String),

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

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '=' => Self::Assign,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '!' => Self::Bang,
            '*' => Self::Asterisk,
            '/' => Self::Slash,
            '<' => Self::LessThan,
            '>' => Self::GreaterThan,
            ',' => Self::Comma,
            ';' => Self::Semicolon,
            '(' => Self::LParenthesis,
            ')' => Self::RParenthesis,
            '{' => Self::LBrace,
            '}' => Self::RBrace,
            '\0' => Self::EndOfFile,
            _ => Self::Illegal(value.to_string()),
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "==" => Self::Equal,
            "!=" => Self::NotEqual,
            "<=" => Self::LessOrEqual,
            ">=" => Self::GreaterOrEqual,
            "fn" => Self::Function,
            "let" => Self::Let,
            "true" => Self::True,
            "false" => Self::False,
            "if" => Self::If,
            "else" => Self::Else,
            "return" => Self::Return,
            _ => {
                if value.chars().all(|c| c.is_ascii_digit()) {
                    Self::Integer(value)
                } else {
                    Self::Identifier(value)
                }
            }
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Illegal(c) => write!(f, "Illegal: {c}"),
            Self::EndOfFile => write!(f, "EOF"),
            Self::Identifier(identifier) => write!(f, "{identifier}"),
            Self::Integer(literal) => write!(f, "{literal}"),
            Self::Assign => write!(f, "="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Bang => write!(f, "!"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::LessThan => write!(f, "<"),
            Self::GreaterThan => write!(f, ">"),
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::LessOrEqual => write!(f, "<="),
            Self::GreaterOrEqual => write!(f, ">="),
            Self::Comma => write!(f, ","),
            Self::Semicolon => write!(f, ";"),
            Self::LParenthesis => write!(f, "("),
            Self::RParenthesis => write!(f, ")"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::Function => write!(f, "fn"),
            Self::Let => write!(f, "let"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Return => write!(f, "return"),
        }
    }
}
