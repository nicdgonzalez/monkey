use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::Expression;
use crate::object::Object;
use crate::parser::{ParseInfix, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
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

    // The token is used by `Call::parse_prefix`.
    // _ = parser.expect_token_with_kind(TokenKind::LParenthesis)?;

    if parser
        .token()
        .is_some_and(|token| token.kind() == TokenKind::RParenthesis)
    {
        parser.advance();
        return Ok(arguments);
    }

    arguments.push({
        tracing::trace!("preparing to add first argument to collection");
        let argument = Expression::parse(parser, Precedence::Lowest)?;
        tracing::trace!("adding first call argument to collection: {argument:?}");
        Box::new(argument)
    });

    // TODO: Function calls are breaking at the comma.
    while parser
        .token()
        .is_some_and(|token| token.kind() == TokenKind::Comma)
    {
        tracing::debug!("Before: {:?}", parser.token());
        assert_eq!(
            parser.token().map(|token| token.kind()),
            Some(TokenKind::Comma)
        );
        _ = parser.advance();
        tracing::debug!("After: {:?}", parser.token());

        // I don't think this part is properly advancing.
        //
        // The error says "comma is missing prefix fn", which is the first statement in
        // Expression::parse.
        //
        // TODO: Add `tracing` crate and a flag to enable logging.
        arguments.push(Box::new(Expression::parse(parser, Precedence::Lowest)?));
    }

    _ = parser.expect_token_with_kind(TokenKind::RParenthesis)?;

    Ok(arguments)
}

impl Evaluate for Call {
    fn evaluate(&self, env: &mut Environment) -> Object {
        let function = (*self.function).evaluate(env);

        if matches!(function, Object::Error(_)) {
            return function;
        }

        let arguments = evaluate_call_arguments(&self.arguments, env);

        if arguments.len() == 1 && matches!(arguments.first().unwrap(), Object::Error(_)) {
            return arguments.into_iter().next().unwrap();
        }

        // return applyFunction(function, args)

        todo!();
    }
}

fn evaluate_call_arguments(arguments: &[Box<Expression>], env: &mut Environment) -> Vec<Object> {
    let mut results = Vec::new();

    for argument in arguments {
        let result = argument.evaluate(env);

        if matches!(result, Object::Error(_)) {
            return vec![result];
        }

        results.push(result);
    }

    results
}

fn evaluate_call_apply_function(function: &Object, arguments: &[Object]) -> Object {
    // Convert function into the inner function object.

    // Extend function environment.

    // Evaluate function body.

    // Unwrap the return value.

    todo!()
}
