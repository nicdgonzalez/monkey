use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;

pub struct Lexer<'a> {
    pub input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }
}

impl Lexer<'_> {
    pub fn next_token(&mut self) -> Token {
        // Skip over whitespaces.
        while self
            .input
            .by_ref()
            .peek()
            .is_some_and(|c| c.is_ascii_whitespace())
        {
            _ = self.input.next();
        }

        let peek = match self.input.peek() {
            Some(&c) => c,
            None => return Token::EndOfFile,
        };

        match peek {
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();

                while self
                    .input
                    .peek()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || *c == '_')
                {
                    let v = self.input.next().unwrap();
                    identifier.push(v);
                }

                Token::from(identifier)
            }
            '0'..='9' => {
                let mut literal = String::new();

                while self.input.peek().is_some_and(|c| c.is_ascii_digit()) {
                    let d = self.input.next().unwrap();
                    literal.push(d);
                }

                Token::from(literal)
            }
            '=' | '!' | '<' | '>' => {
                let c = self.input.next().unwrap();

                if let Some(&'=') = self.input.peek() {
                    let v = self.input.next().unwrap();
                    return (String::from(c) + &String::from(v)).into();
                } else {
                    Token::from(c)
                }
            }
            _ => {
                let v = self.input.next().unwrap();
                Token::from(v)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() -> () {
        let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
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
        let expected: Vec<Token> = vec![
            Token::Let,
            Token::Identifier("five".to_string()),
            Token::Assign,
            Token::Integer("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".to_string()),
            Token::Assign,
            Token::Integer("10".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParenthesis,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::RParenthesis,
            Token::LBrace,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("add".to_string()),
            Token::LParenthesis,
            Token::Identifier("five".to_string()),
            Token::Comma,
            Token::Identifier("ten".to_string()),
            Token::RParenthesis,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer("5".to_string()),
            Token::Semicolon,
            Token::Integer("5".to_string()),
            Token::LessThan,
            Token::Integer("10".to_string()),
            Token::GreaterThan,
            Token::Integer("5".to_string()),
            Token::Semicolon,
            Token::If,
            Token::LParenthesis,
            Token::Integer("5".to_string()),
            Token::LessThan,
            Token::Integer("10".to_string()),
            Token::RParenthesis,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Integer("10".to_string()),
            Token::Equal,
            Token::Integer("10".to_string()),
            Token::Semicolon,
            Token::Integer("10".to_string()),
            Token::NotEqual,
            Token::Integer("9".to_string()),
            Token::Semicolon,
            Token::EndOfFile,
        ];

        let mut lexer = Lexer::new(&input);

        for (i, test) in expected.iter().enumerate() {
            let token = lexer.next_token();

            if token != *test {
                panic!("test[{i}]: expected {}, got {:?}", test, token);
            }
        }
    }
}
