use crate::statement::{self, Block};
use crate::token::{Token, TokenKind};

use super::ast::Node;
use super::parser::{
    parse_expression, Parse, ParseInfix, ParsePrefix, ParserContext, ParserError, Precedence,
    PRECEDENCES,
};

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Prefix(Prefix),
    Infix(Infix),
    Boolean(Boolean),
    If(If),
    FunctionLiteral(FunctionLiteral),
    Call(Call),
}

impl From<Node> for Expression {
    fn from(value: Node) -> Self {
        match value {
            Node::Expression(expression) => expression,
            _ => panic!("expected node to be an expression"),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(inner) => write!(f, "{}", inner),
            Self::IntegerLiteral(inner) => write!(f, "{}", inner),
            Self::Prefix(inner) => write!(f, "{}", inner),
            Self::Infix(inner) => write!(f, "{}", inner),
            Self::Boolean(inner) => write!(f, "{}", inner),
            Self::If(inner) => write!(f, "{}", inner),
            Self::FunctionLiteral(inner) => write!(f, "{}", inner),
            Self::Call(inner) => write!(f, "{}", inner),
        }
    }
}

// ┌────────────────────────────────┐
// │ Implementations for Identifier │
// └────────────────────────────────┘

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
}

impl From<Identifier> for Expression {
    fn from(value: Identifier) -> Self {
        Expression::Identifier(value)
    }
}

impl From<Identifier> for Node {
    fn from(value: Identifier) -> Self {
        Expression::Identifier(value).into()
    }
}

impl ParsePrefix for Identifier {
    fn parse_prefix(parser: &mut ParserContext<'_>) -> Result<Expression, ParserError> {
        let node = parser
            .expect_token(TokenKind::Identifier)
            .map(|token| Self { token })?
            .into();

        Ok(node)
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

// ┌─────────────────────────────────────┐
// │ Implementations for Integer Literal │
// └─────────────────────────────────────┘

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl From<IntegerLiteral> for Expression {
    fn from(value: IntegerLiteral) -> Self {
        Expression::IntegerLiteral(value)
    }
}

impl From<IntegerLiteral> for Node {
    fn from(value: IntegerLiteral) -> Self {
        Expression::IntegerLiteral(value).into()
    }
}

impl ParsePrefix for IntegerLiteral {
    fn parse_prefix(parser: &mut ParserContext<'_>) -> Result<Expression, ParserError> {
        let node = parser
            .expect_token(TokenKind::Integer)
            .map(|token| {
                let value = token.literal.clone().parse::<i64>().unwrap();
                Self { token, value }
            })?
            .into();

        Ok(node)
    }
}

impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

// ┌────────────────────────────┐
// │ Implementations for Prefix │
// └────────────────────────────┘

#[derive(Debug)]
pub struct Prefix {
    pub token: Token,
    pub right: Box<Expression>,
}

impl From<Prefix> for Expression {
    fn from(value: Prefix) -> Self {
        Expression::Prefix(value)
    }
}
impl From<Prefix> for Node {
    fn from(value: Prefix) -> Self {
        Expression::Prefix(value).into()
    }
}

impl ParsePrefix for Prefix {
    fn parse_prefix(parser: &mut ParserContext<'_>) -> Result<Expression, ParserError> {
        debug_assert!(
            parser.token.kind == TokenKind::Minus || parser.token.kind == TokenKind::Bang
        );
        let token = parser.token.clone();
        parser.advance();

        let right = parse_expression(parser, Precedence::Prefix)?;

        let expression = Self {
            token,
            right: Box::new(right),
        };
        Ok(expression.into())
    }
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.token, self.right)
    }
}

// ┌───────────────────────────┐
// │ Implementations for Infix │
// └───────────────────────────┘

#[derive(Debug)]
pub struct Infix {
    pub token: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl From<Infix> for Expression {
    fn from(value: Infix) -> Self {
        Expression::Infix(value)
    }
}

impl From<Infix> for Node {
    fn from(value: Infix) -> Self {
        Expression::Infix(value).into()
    }
}

impl ParseInfix for Infix {
    fn parse_infix(
        parser: &mut ParserContext<'_>,
        left: Expression,
    ) -> Result<Expression, ParserError> {
        let token = parser.token.clone();
        let precedence = PRECEDENCES
            .get(&parser.token.kind)
            .unwrap_or_else(|| &Precedence::Lowest);
        parser.advance();

        let right = parse_expression(parser, precedence.clone())?;

        let expression = Self {
            token,
            left: Box::new(left),
            right: Box::new(right),
        };
        Ok(expression.into())
    }
}

impl std::fmt::Display for Infix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.token, self.right)
    }
}

// ┌─────────────────────────────┐
// │ Implementations for Boolean │
// └─────────────────────────────┘

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl From<Boolean> for Expression {
    fn from(value: Boolean) -> Self {
        Expression::Boolean(value)
    }
}

impl From<Boolean> for Node {
    fn from(value: Boolean) -> Self {
        Expression::Boolean(value).into()
    }
}

impl ParsePrefix for Boolean {
    fn parse_prefix(parser: &mut ParserContext<'_>) -> Result<Expression, ParserError> {
        let token = parser.token.clone();
        let value = token.kind == TokenKind::True;
        let node = Self { token, value }.into();
        Ok(node)
    }
}

impl std::fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// ┌────────────────────────┐
// │ Implementations for If │
// └────────────────────────┘

#[derive(Debug)]
pub struct If {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: statement::Block,
    pub alternative: Option<statement::Block>,
}

impl From<If> for Expression {
    fn from(value: If) -> Self {
        Expression::If(value)
    }
}

impl From<If> for Node {
    fn from(value: If) -> Self {
        Expression::If(value).into()
    }
}

impl ParsePrefix for If {
    fn parse_prefix(parser: &mut ParserContext<'_>) -> Result<Expression, ParserError> {
        debug_assert_eq!(parser.token.kind, TokenKind::If);
        let token = parser.token.clone();

        _ = parser.expect_peek(TokenKind::LParenthesis)?;

        let condition = parse_expression(parser, Precedence::Lowest)?;

        debug_assert_eq!(parser.token.kind, TokenKind::RParenthesis);
        parser.advance();

        let consequence = statement::Block::parse(parser)?.into();

        let alternative = if parser.peek.kind == TokenKind::Else {
            parser.advance();

            _ = parser.expect_peek(TokenKind::LBrace)?;

            let alt = statement::Block::parse(parser)?;
            Some(alt.into())
        } else {
            None
        };

        let expression = Self {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        };
        Ok(expression.into())
    }
}

impl std::fmt::Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alternative = if let Some(alt) = self.alternative.as_ref() {
            format!(" else {{ {} }}", alt)
        } else {
            "".to_string()
        };

        write!(
            f,
            "{} ({}) {{ {} }}{}",
            self.token, self.condition, self.consequence, alternative
        )
    }
}

// ┌──────────────────────────────────────┐
// │ Implementations for Function Literal │
// └──────────────────────────────────────┘

#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Block,
}

impl From<FunctionLiteral> for Expression {
    fn from(value: FunctionLiteral) -> Self {
        Expression::FunctionLiteral(value)
    }
}

impl From<FunctionLiteral> for Node {
    fn from(value: FunctionLiteral) -> Self {
        Expression::FunctionLiteral(value).into()
    }
}

impl ParsePrefix for FunctionLiteral {
    fn parse_prefix(parser: &mut ParserContext<'_>) -> Result<Expression, ParserError> {
        debug_assert_eq!(parser.token.kind, TokenKind::Function);
        let token = parser.token.clone();

        _ = parser.expect_peek(TokenKind::LParenthesis)?;

        let parameters = parse_function_parameters(parser)?;
        debug_assert_eq!(parser.token.kind, TokenKind::RParenthesis);

        _ = parser.expect_peek(TokenKind::LBrace)?;

        let body = statement::Block::parse(parser)?.into();

        let expression = Self {
            token,
            parameters,
            body,
        };
        Ok(expression.into())
    }
}

fn parse_function_parameters(
    parser: &mut ParserContext<'_>,
) -> Result<Vec<Identifier>, ParserError> {
    let mut parameters = Vec::<Identifier>::new();
    debug_assert_eq!(parser.token.kind, TokenKind::LParenthesis);
    parser.advance();

    // Exit early if there are no parameters.
    if parser.token.kind == TokenKind::RParenthesis {
        return Ok(parameters);
    }

    // Parse the first parameter.
    let mut parameter = parser
        .expect_token(TokenKind::Identifier)
        .map(|token| Identifier { token })?;
    parameters.push(parameter.into());

    // Iterate over the remaining parameters.
    while parser.token.kind != TokenKind::EndOfFile {
        if parser.peek.kind != TokenKind::Comma {
            break; // Next token should be a closing parenthesis.
        } else {
            parser.advance();
        }

        parameter = parser
            .expect_peek(TokenKind::Identifier)
            .map(|token| Identifier { token })?;
        parameters.push(parameter.into());
    }

    _ = parser.expect_peek(TokenKind::RParenthesis)?;

    Ok(parameters)
}

impl std::fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}({}) {{ {} }}", self.token, parameters, self.body)
    }
}

// ┌──────────────────────────┐
// │ Implementations for Call │
// └──────────────────────────┘

#[derive(Debug)]
pub struct Call {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
}

impl From<Call> for Expression {
    fn from(value: Call) -> Self {
        Expression::Call(value)
    }
}

impl From<Call> for Node {
    fn from(value: Call) -> Self {
        Expression::Call(value).into()
    }
}

impl ParseInfix for Call {
    fn parse_infix(
        parser: &mut ParserContext<'_>,
        function: Expression,
    ) -> Result<Expression, ParserError> {
        debug_assert_eq!(parser.token.kind, TokenKind::LParenthesis);
        let token = parser.token.clone();
        let arguments = parse_call_arguments(parser)?;
        let expression = Self {
            token,
            function: Box::new(function),
            arguments,
        };
        Ok(expression.into())
    }
}

fn parse_call_arguments(
    parser: &mut ParserContext<'_>,
) -> Result<Vec<Box<Expression>>, ParserError> {
    let mut arguments = Vec::<Box<Expression>>::new();
    debug_assert_eq!(parser.token.kind, TokenKind::LParenthesis);
    parser.advance();

    // Exit early if there are no arguments.
    if parser.token.kind == TokenKind::RParenthesis {
        return Ok(arguments);
    }

    // Parse the first argument.
    let mut argument = parse_expression(parser, Precedence::Lowest)?;
    arguments.push(Box::new(argument.into()));

    // Iterate over the remaining arguments.
    while parser.token.kind != TokenKind::EndOfFile {
        if parser.peek.kind != TokenKind::Comma {
            break;
        } else {
            parser.advance();
        }

        // Advance to the next parameter.
        parser.advance();

        argument = parse_expression(parser, Precedence::Lowest)?;
        arguments.push(Box::new(argument));
    }

    _ = parser.expect_peek(TokenKind::RParenthesis)?;

    Ok(arguments)
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arguments = self
            .arguments
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}({})", self.function, arguments)
    }
}
