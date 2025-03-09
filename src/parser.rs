use crate::{ast::Program, Lexer, Token};

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer<'a>,
    pub current: Token,
    pub next: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        // Start the parser in a working state.
        let current = lexer.next_token();
        let next = lexer.next_token();

        Self {
            lexer,
            current,
            next,
        }
    }

    pub fn parse_program(&self) -> Program {
        Program::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Statement};

    use super::*;

    #[test]
    fn test_let_statements() -> () {
        let input = r#"
            let x = 5;
            let y = 10;
            let foo = 69;
        "#;

        let mut lexer = Lexer::new(&input);
        let parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 3);

        let expected: &[Statement] = &[
            Statement::Let(
                Token::Let,
                Expression::Identifier(Token::Identifier("x".to_string()), "5".to_string()),
            ),
            Statement::Let(
                Token::Let,
                Expression::Identifier(Token::Identifier("y".to_string()), "10".to_string()),
            ),
            Statement::Let(
                Token::Let,
                Expression::Identifier(Token::Identifier("foo".to_string()), "69".to_string()),
            ),
        ];
    }
}
