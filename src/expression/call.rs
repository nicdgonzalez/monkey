use std::collections::HashMap;

use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::expression::Expression;
use crate::object::Object;
use crate::parser::{ParseInfix, Parser, ParserError};
use crate::precedence::Precedence;
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Call {
    function: Box<Expression>,
    arguments: Vec<Expression>,
}

impl Call {
    pub const fn new(function: Box<Expression>, arguments: Vec<Expression>) -> Self {
        Self {
            function,
            arguments,
        }
    }
}

impl ParseInfix for Call {
    fn parse_infix(parser: &mut Parser<'_>, left: Expression) -> Result<Expression, ParserError> {
        _ = parser.expect_token_with_kind(TokenKind::LParenthesis)?;
        let arguments = parse_call_arguments(parser)?;
        _ = parser.expect_token_with_kind(TokenKind::RParenthesis)?;

        let expression = Self::new(Box::new(left), arguments);
        Ok(expression.into())
    }
}

fn parse_call_arguments(parser: &mut Parser<'_>) -> Result<Vec<Expression>, ParserError> {
    let mut arguments = Vec::new();

    if parser
        .token()
        .is_some_and(|token| token.kind() == TokenKind::RParenthesis)
    {
        return Ok(arguments);
    }

    arguments.push(Expression::parse(parser, Precedence::Lowest)?);

    while let Some(TokenKind::Comma) = parser.token().map(Token::kind) {
        _ = parser.expect_token_with_kind(TokenKind::Comma)?;
        arguments.push(Expression::parse(parser, Precedence::Lowest)?);
    }

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

        let function = match function {
            Object::Function(inner) => inner,
            _ => panic!("expected Function object"),
        };

        let outer = Box::new(function.env().to_owned());
        let mut env_extended = Environment::new(HashMap::new(), Some(outer));

        for (index, parameter) in function.parameters().iter().enumerate() {
            let key = parameter.token().literal();
            let value = arguments.get(index).unwrap();

            if env_extended
                .store_mut()
                .insert(key.to_owned(), value.to_owned())
                .is_some()
            {
                panic!("variable named {key:?} already exists");
            }
        }

        let value = function.body().evaluate(&mut env_extended);

        match value {
            Object::Return(ref inner) => inner.value().to_owned(),
            _ => value,
        }
    }
}

fn evaluate_call_arguments(arguments: &[Expression], env: &mut Environment) -> Vec<Object> {
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
