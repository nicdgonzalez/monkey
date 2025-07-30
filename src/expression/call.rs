use crate::expression::Expression;
use crate::parser::{ParseInfix, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Call {
    token: Token,
    function: Box<Expression>,
    arguments: Vec<Box<Expression>>,
}

impl Call {
    pub const fn new(
        token: Token,
        function: Box<Expression>,
        arguments: Vec<Box<Expression>>,
    ) -> Self {
        Self {
            token,
            function,
            arguments,
        }
    }
}

impl ParseInfix for Call {
    fn parse_infix(parser: &mut Parser<'_>, left: Expression) -> Result<Expression, ParserError> {
        let token = parser.expect_token_with_kind(TokenKind::LParenthesis)?;
        let arguments = parse_call_arguments(parser)?;

        let expression = Self::new(token, Box::new(left), arguments);
        Ok(expression.into())
    }
}

fn parse_call_arguments(parser: &mut Parser<'_>) -> Result<Vec<Box<Expression>>, ParserError> {
    let mut arguments = Vec::new();

    _ = parser.expect_token_with_kind(TokenKind::LParenthesis)?;

    if parser
        .token()
        .is_some_and(|token| token.kind() == TokenKind::RParenthesis)
    {
        parser.advance();
        return Ok(arguments);
    }

    arguments.push(Box::new(Expression::parse(parser, Precedence::Lowest)?));

    while parser
        .token()
        .is_some_and(|token| token.kind() == TokenKind::Comma)
    {
        assert_eq!(
            parser.token().map(|token| token.kind()),
            Some(TokenKind::Comma)
        );
        _ = parser.advance();

        arguments.push(Box::new(Expression::parse(parser, Precedence::Lowest)?));
    }

    _ = parser.expect_token_with_kind(TokenKind::RParenthesis)?;

    Ok(arguments)
}
