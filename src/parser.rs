use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::ast::Node;
use crate::expression;
use crate::lexer::Lexer;
use crate::statement;
use crate::token::{Token, TokenKind};

pub trait Parse {
    fn parse(parser: &mut Parser<'_>) -> Result<Node, ParserError>;
}

pub trait ParsePrefix {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<expression::Expression, ParserError>;
}

pub trait ParseInfix {
    fn parse_infix(
        parser: &mut Parser<'_>,
        left: expression::Expression,
    ) -> Result<expression::Expression, ParserError>;
}

type PrefixParseFn = fn(&mut Parser<'_>) -> Result<expression::Expression, ParserError>;
type InfixParseFn =
    fn(&mut Parser<'_>, expression::Expression) -> Result<expression::Expression, ParserError>;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    pub token: Token,
    pub peek: Token,
    prefix_fns: HashMap<TokenKind, PrefixParseFn>,
    infix_fns: HashMap<TokenKind, InfixParseFn>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        // Start the parser in a working state.
        let token = lexer.get_next_token();
        let peek = lexer.get_next_token();

        let mut parser = Self {
            lexer,
            token,
            peek,
            prefix_fns: HashMap::new(),
            infix_fns: HashMap::new(),
        };

        use TokenKind::*;
        // Register prefix parsing functions.
        parser.register_prefix_fn(Identifier, expression::Identifier::parse_prefix);
        parser.register_prefix_fn(Integer, expression::IntegerLiteral::parse_prefix);
        parser.register_prefix_fn(Bang, expression::Prefix::parse_prefix);
        parser.register_prefix_fn(Minus, expression::Prefix::parse_prefix);
        parser.register_prefix_fn(True, expression::Boolean::parse_prefix);
        parser.register_prefix_fn(False, expression::Boolean::parse_prefix);
        parser.register_prefix_fn(LParenthesis, parse_grouped_expression);
        parser.register_prefix_fn(If, expression::If::parse_prefix);
        parser.register_prefix_fn(Function, expression::FunctionLiteral::parse_prefix);

        // Register infix parsing functions.
        parser.register_infix_fn(Plus, expression::Infix::parse_infix);
        parser.register_infix_fn(Minus, expression::Infix::parse_infix);
        parser.register_infix_fn(Slash, expression::Infix::parse_infix);
        parser.register_infix_fn(Asterisk, expression::Infix::parse_infix);
        parser.register_infix_fn(Equal, expression::Infix::parse_infix);
        parser.register_infix_fn(NotEqual, expression::Infix::parse_infix);
        parser.register_infix_fn(LessThan, expression::Infix::parse_infix);
        parser.register_infix_fn(GreaterThan, expression::Infix::parse_infix);
        parser.register_infix_fn(LParenthesis, expression::Call::parse_infix);

        parser
    }

    fn register_prefix_fn(&mut self, key: TokenKind, value: PrefixParseFn) {
        debug_assert!(self.prefix_fns.insert(key, value).is_none());
    }

    fn register_infix_fn(&mut self, key: TokenKind, value: InfixParseFn) {
        debug_assert!(self.infix_fns.insert(key, value).is_none());
    }

    /// Update fields `current` and `next` with the next two lexer tokens.
    pub fn advance(&mut self) {
        self.token = self.peek.to_owned();
        self.peek = self.lexer.get_next_token();
    }

    pub fn expect_token(&self, expected: TokenKind) -> Result<Token, ParserError> {
        if self.token.kind == expected {
            Ok(self.token.clone())
        } else {
            Err(ParserError::WrongToken(expected, self.token.kind.clone()))
        }
    }

    /// If peek matches `expected`, advance the parser and return peek's token.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// - `ParserError::WrongToken`: peek did not match `expected`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use monkey::lexer::Lexer;
    /// # use monkey::parser::Parser;
    /// # use monkey::token::TokenKind;
    /// # fn main() {
    /// let input = "let five = 5;";
    /// let mut lexer = Lexer::new(&input);
    /// let mut parser = Parser::new(&mut lexer);
    ///
    /// assert_eq!(parser.token.kind, TokenKind::Let);
    /// let identifier = parser.expect_peek(TokenKind::Identifier).unwrap();
    ///
    /// assert_eq!(identifier.kind, TokenKind::Identifier);
    /// assert_eq!(identifier.literal, "five".to_string());
    /// # }
    /// ```
    pub fn expect_peek(&mut self, expected: TokenKind) -> Result<Token, ParserError> {
        if self.peek.kind == expected {
            self.advance();
            Ok(self.token.clone())
        } else {
            Err(ParserError::WrongToken(expected, self.peek.kind.clone()))
        }
    }
}

pub fn parse_statement(parser: &mut Parser<'_>) -> Result<statement::Statement, ParserError> {
    let statement = match parser.token.kind {
        TokenKind::Let => statement::Let::parse(parser),
        TokenKind::Return => statement::Return::parse(parser),
        _ => statement::Expression::parse(parser),
    }?;

    Ok(statement.into())
}

pub fn parse_expression(
    parser: &mut Parser<'_>,
    precedence: Precedence,
) -> Result<expression::Expression, ParserError> {
    let mut left = match parser.prefix_fns.get(&parser.token.kind) {
        Some(callback) => callback(parser),
        None => Err(ParserError::MissingPrefixFn(parser.token.clone())),
    }?;

    while parser.peek.kind != TokenKind::EndOfFile {
        if parser.peek.kind == TokenKind::Semicolon {
            break;
        }

        let peek_precedence = PRECEDENCES
            .get(&parser.peek.kind)
            .unwrap_or_else(|| &Precedence::Lowest);

        if *peek_precedence < precedence {
            break;
        }

        if let Some(&infix) = parser.infix_fns.get(&parser.peek.kind) {
            parser.advance();
            left = infix(parser, left)?;
        } else {
            return Ok(left);
        }
    }

    Ok(left)
}

pub fn parse_grouped_expression(
    parser: &mut Parser<'_>,
) -> Result<expression::Expression, ParserError> {
    debug_assert_eq!(parser.token.kind, TokenKind::LParenthesis);
    parser.advance();

    let expr = parse_expression(parser, Precedence::Lowest)?;

    _ = parser.expect_peek(TokenKind::RParenthesis)?;

    Ok(expr)
}

#[derive(Debug)]
pub enum ParserError {
    WrongToken(TokenKind, TokenKind),
    MissingPrefixFn(Token),
    MissingInfixFn(Token),
}

impl std::error::Error for ParserError {}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongToken(expected, actual) => {
                write!(f, "expected next token to be {}, got {}", expected, actual)
            }
            Self::MissingPrefixFn(token) => {
                write!(f, "expected {} to have a prefix parsing function", token)
            }
            Self::MissingInfixFn(token) => {
                write!(f, "expected {} to have an infix parsing function", token)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

lazy_static! {
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

// #[cfg(test)]
// mod tests {
//     use crate::ast::Program;
//
//     use super::*;
//
//     #[test]
//     fn test_let_statements() {
//         let input = r#"
//             let x = 5;
//             let y = 10;
//             let foo = 69;
//         "#;
//
//         let mut lexer = Lexer::new(&input);
//         let mut parser = Parser::new(&mut lexer);
//
//         let program: Program = Program::parse(&mut parser).unwrap().into();
//         assert_eq!(program.statements.len(), 3);
//
//         let expected_tokens: &[statement::Statement] = &[
//             Statement::Let(
//                 Token::from("let"),
//                 Expression::Identifier(Token::from("x")),
//                 Expression::IntegerLiteral(Token::from("5"), 5),
//             ),
//             Statement::Let(
//                 Token::from("let"),
//                 Expression::Identifier(Token::from("y")),
//                 Expression::IntegerLiteral(Token::from("10"), 10),
//             ),
//             Statement::Let(
//                 Token::from("let"),
//                 Expression::Identifier(Token::from("foo")),
//                 Expression::IntegerLiteral(Token::from("69"), 69),
//             ),
//         ];
//
//         for (i, expected_token) in expected_tokens.iter().enumerate() {
//             assert_eq!(program.statements.iter().nth(i), Some(expected_token));
//         }
//     }
//
//     #[test]
//     fn test_return_statements() {
//         let input = r#"
//             return 5;
//             return 10;
//             return 69;
//         "#;
//
//         let mut lexer = Lexer::new(&input);
//         let mut parser = Parser::new(&mut lexer);
//
//         let program = parser.parse_program();
//         assert_eq!(program.statements.len(), 3);
//
//         let expected_tokens: &[Statement] = &[
//             Statement::Return(
//                 Token::from("return"),
//                 Expression::IntegerLiteral(Token::from("5"), 5),
//             ),
//             Statement::Return(
//                 Token::from("return"),
//                 Expression::IntegerLiteral(Token::from("10"), 10),
//             ),
//             Statement::Return(
//                 Token::from("return"),
//                 Expression::IntegerLiteral(Token::from("69"), 69),
//             ),
//         ];
//
//         for (i, expected_token) in expected_tokens.iter().enumerate() {
//             assert_eq!(program.statements.iter().nth(i), Some(expected_token));
//         }
//     }
//
//     #[test]
//     fn test_identifier_expressions() {
//         let input = r#"
//             foo;
//         "#;
//
//         let mut lexer = Lexer::new(&input);
//         let mut parser = Parser::new(&mut lexer);
//
//         let program = parser.parse_program();
//         assert_eq!(program.statements.len(), 1);
//
//         let expected_tokens: &[Statement] = &[Statement::Expression(
//             Token::from("foo"),
//             Expression::Identifier(Token::from("foo")),
//         )];
//
//         for (i, expected_token) in expected_tokens.iter().enumerate() {
//             assert_eq!(program.statements.iter().nth(i), Some(expected_token));
//         }
//     }
//
//     #[test]
//     fn test_operator_precedence() {
//         let tests: &[(&str, &str)] = &[
//             (("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)")),
//             (("(5 + 5) * 2", "((5 + 5) * 2)")),
//             (("2 / (5 + 5)", "(2 / (5 + 5))")),
//             (("-(5 + 5)", "(-(5 + 5))")),
//             (("!(true == true)", "(!(true == true))")),
//         ];
//
//         for (input, expected) in tests.iter() {
//             let mut lexer = Lexer::new(&input);
//             let mut parser = Parser::new(&mut lexer);
//             let program = parser.parse_program();
//             assert_eq!(program.to_string(), *expected);
//         }
//     }
// }
