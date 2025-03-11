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
    pub fn get_next_token(&mut self) -> Token {
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
            None => return Token::from('\0'),
        };

        match peek {
            'a'..='z' | 'A'..='Z' | '_' => {
                let c = self.input.next().unwrap();
                let mut identifier = String::from(c);

                while self
                    .input
                    .peek()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || *c == '_')
                {
                    let d = self.input.next().unwrap();
                    identifier.push(d);
                }

                Token::from(identifier.as_ref())
            }
            '0'..='9' => {
                let c = self.input.next().unwrap();
                let mut literal = String::from(c);

                while self.input.peek().is_some_and(|c| c.is_ascii_digit()) {
                    let d = self.input.next().unwrap();
                    literal.push(d);
                }

                Token::from(literal.as_ref())
            }
            '=' | '!' | '<' | '>' => {
                let c = self.input.next().unwrap();

                if let Some(&'=') = self.input.peek() {
                    let d = self.input.next().unwrap();
                    let s = String::from(c) + &String::from(d);
                    return Token::from(s.as_ref());
                } else {
                    Token::from(c)
                }
            }
            _ => {
                let c = self.input.next().unwrap();
                Token::from(c)
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

        let expected_tokens: &[Token] = &[
            Token::from("let"),
            Token::from("five"),
            Token::from('='),
            Token::from("5"),
            Token::from(';'),
            Token::from("let"),
            Token::from("ten"),
            Token::from('='),
            Token::from("10"),
            Token::from(';'),
            Token::from("let"),
            Token::from("add"),
            Token::from('='),
            Token::from("fn"),
            Token::from('('),
            Token::from("x"),
            Token::from(','),
            Token::from("y"),
            Token::from(')'),
            Token::from('{'),
            Token::from("x"),
            Token::from('+'),
            Token::from("y"),
            Token::from(';'),
            Token::from('}'),
            Token::from(';'),
            Token::from("let"),
            Token::from("result"),
            Token::from('='),
            Token::from("add"),
            Token::from('('),
            Token::from("five"),
            Token::from(','),
            Token::from("ten"),
            Token::from(')'),
            Token::from(';'),
            Token::from('!'),
            Token::from('-'),
            Token::from('/'),
            Token::from('*'),
            Token::from("5"),
            Token::from(';'),
            Token::from("5"),
            Token::from('<'),
            Token::from("10"),
            Token::from('>'),
            Token::from("5"),
            Token::from(';'),
            Token::from("if"),
            Token::from('('),
            Token::from("5"),
            Token::from('<'),
            Token::from("10"),
            Token::from(')'),
            Token::from('{'),
            Token::from("return"),
            Token::from("true"),
            Token::from(';'),
            Token::from('}'),
            Token::from("else"),
            Token::from('{'),
            Token::from("return"),
            Token::from("false"),
            Token::from(';'),
            Token::from('}'),
            Token::from("10"),
            Token::from("=="),
            Token::from("10"),
            Token::from(';'),
            Token::from("10"),
            Token::from("!="),
            Token::from("9"),
            Token::from(';'),
            Token::from('\0'),
        ];

        let mut lexer = Lexer::new(&input);

        for expected_token in expected_tokens.iter() {
            assert_eq!(lexer.get_next_token(), *expected_token);
        }
    }
}
