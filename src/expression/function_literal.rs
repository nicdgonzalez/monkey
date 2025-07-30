use crate::ast::Node;
use crate::expression::{Expression, Identifier};
use crate::parser::{Parse, ParsePrefix, Parser, ParserError};
use crate::statement::{Block, Statement};
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct FunctionLiteral {
    token: Token,
    parameters: Vec<Identifier>,
    body: Block,
}

impl FunctionLiteral {
    pub fn new(token: Token, parameters: Vec<Identifier>, body: Block) -> Self {
        Self {
            token,
            parameters,
            body,
        }
    }

    pub const fn token(&self) -> &Token {
        &self.token
    }

    pub fn parameters(&self) -> &[Identifier] {
        &self.parameters
    }

    pub const fn body(&self) -> &Block {
        &self.body
    }
}

impl ParsePrefix for FunctionLiteral {
    fn parse_prefix(parser: &mut Parser<'_>) -> Result<Expression, ParserError> {
        let token = parser.expect_token_with_kind(TokenKind::Function)?;
        let parameters = parse_function_literal_parameters(parser)?;
        // TODO: Refactor `Block::parse`.
        let body = match Block::parse(parser)? {
            Node::Statement(Statement::Block(inner)) => inner,
            _ => unreachable!("expected Block::parse to return a Block"),
        };

        let expression = Self::new(token, parameters, body);
        Ok(expression.into())
    }
}

fn parse_function_literal_parameters(
    parser: &mut Parser<'_>,
) -> Result<Vec<Identifier>, ParserError> {
    let mut parameters = Vec::new();

    _ = parser.expect_token_with_kind(TokenKind::LParenthesis)?;

    if parser
        .token()
        .is_some_and(|token| token.kind() == TokenKind::RParenthesis)
    {
        parser.advance();
        return Ok(parameters);
    }

    parameters.push(
        parser
            .expect_token_with_kind(TokenKind::Identifier)
            .map(Identifier::new)?,
    );

    while parser
        .token()
        .is_some_and(|token| token.kind() == TokenKind::Comma)
    {
        assert_eq!(
            parser.token().map(|token| token.kind()),
            Some(TokenKind::Comma)
        );
        _ = parser.advance();

        parameters.push(
            parser
                .expect_token_with_kind(TokenKind::Identifier)
                .map(Identifier::new)?,
        );
    }

    _ = parser.expect_token_with_kind(TokenKind::RParenthesis)?;

    Ok(parameters)
}
