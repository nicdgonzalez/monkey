// TODO: Clean up like I did with parser.

use crate::ast::Node;
use crate::environment::Environment;
use crate::expression::{self, Expression};
use crate::object::{self, Object, FALSE, NULL, TRUE};
use crate::statement::Statement;
use crate::token::TokenKind;

// pub trait Eval {
//     fn eval(&self) -> Object;
// }

pub fn eval(node: Node, env: &mut Environment) -> Object {
    match node {
        Node::Program(program) => eval_program(program.statements, env),
        Node::Statement(statement) => match statement {
            Statement::Let(inner) => {
                let value = eval((*inner.value).into(), env);

                if let Object::Error(_) = value {
                    return value;
                }

                if let Some(_) = env.store.insert(inner.name.token.literal, value.clone()) {
                    return object::Error {
                        message: format!("variable named {} already exists", inner.token.literal),
                    }
                    .into();
                }

                value
            }
            Statement::Expression(inner) => eval((*inner.expression).into(), env),
            Statement::Return(inner) => {
                let value = eval((*inner.value).into(), env);

                if let Object::Error(_) = value {
                    return value;
                }

                let obj = object::Return {
                    value: Box::new(value),
                };

                obj.into()
            }
            Statement::Block(inner) => eval_block_statements(inner.statements, env),
        },
        Node::Expression(expression) => match expression {
            Expression::Identifier(inner) => eval_identifier(inner, env),
            Expression::IntegerLiteral(inner) => {
                let value = object::Integer { value: inner.value };
                value.into()
            }
            Expression::Prefix(inner) => {
                let right = eval((*inner.right).into(), env);

                if let Object::Error(_) = right {
                    return right;
                }

                let value = eval_prefix_expression(&inner.token.kind, &right);
                value
            }
            Expression::Infix(inner) => {
                let left = eval((*inner.left).into(), env);

                if let Object::Error(_) = left {
                    return left;
                }

                let right = eval((*inner.right).into(), env);

                if let Object::Error(_) = right {
                    return right;
                }

                eval_infix_expression(&inner.token.kind, &left, &right)
            }
            Expression::If(inner) => eval_if_expression(inner, env),
            Expression::Boolean(inner) => {
                let value = object::Boolean { value: inner.value };
                truthy(value.into())
            }
            _ => NULL,
        },
    }
}
fn eval_identifier(expr: expression::Identifier, env: &mut Environment) -> Object {
    let identifier = &expr.token.literal;
    let result = env.store.get(identifier);

    if let None = result {
        return object::Error {
            message: format!("identifier not found: {}", identifier),
        }
        .into();
    }

    result.unwrap().clone()
}

fn eval_program(statements: Vec<Statement>, env: &mut Environment) -> Object {
    let mut result = NULL;

    for statement in statements.iter() {
        result = eval(statement.clone().into(), env);

        match result {
            Object::Return(inner) => return *inner.value,
            Object::Error(_) => return result,
            _ => (),
        }
    }

    result
}

fn eval_block_statements(statements: Vec<Statement>, env: &mut Environment) -> Object {
    let mut result = NULL;

    for statement in statements.iter() {
        result = eval(statement.clone().into(), env);

        match result {
            Object::Return(_) | Object::Error(_) => return result,
            _ => (),
        }
    }

    result
}

fn truthy(obj: Object) -> Object {
    match obj {
        Object::Integer(inner) => {
            if inner.value > 0 {
                TRUE
            } else {
                FALSE
            }
        }
        Object::Boolean(inner) => {
            if inner.value {
                TRUE
            } else {
                FALSE
            }
        }
        Object::Null(_) => FALSE,
        Object::Return(inner) => truthy(*inner.value),
        Object::Error(_) => FALSE,
    }
}

fn eval_prefix_expression(operator: &TokenKind, right: &Object) -> Object {
    match operator {
        TokenKind::Bang => eval_bang_operator_expression(right),
        TokenKind::Minus => eval_minus_prefix_expression(right),
        _ => object::Error {
            message: format!("unknown operator: {}", operator),
        }
        .into(),
    }
}

fn eval_bang_operator_expression(right: &Object) -> Object {
    match right {
        Object::Boolean(inner) => {
            let value = object::Boolean {
                value: !inner.value,
            };
            truthy(value.into())
        }
        Object::Null(_) => FALSE,
        Object::Integer(inner) => {
            let value = object::Boolean {
                value: inner.value <= 0,
            };
            truthy(value.into())
        }
        Object::Return(inner) => eval_bang_operator_expression(&*inner.value),
        Object::Error(_) => object::Error {
            message: format!("type error: TODO"),
        }
        .into(),
    }
}

fn eval_minus_prefix_expression(right: &Object) -> Object {
    if let Object::Integer(inner) = right {
        let value = object::Integer {
            value: -inner.value,
        };
        value.into()
    } else {
        object::Error {
            message: format!("unknown operator: -{}", right),
        }
        .into()
    }
}

fn eval_infix_expression(operator: &TokenKind, left: &Object, right: &Object) -> Object {
    if std::mem::discriminant(left) != std::mem::discriminant(right) {
        return object::Error {
            message: format!("type mismatch: {} {} {}", left, operator, right),
        }
        .into();
    }

    match (operator, left, right) {
        (_, Object::Integer(inner_left), Object::Integer(inner_right)) => {
            eval_integer_infix_expression(operator, inner_left, inner_right)
        }
        (TokenKind::Equal, _, _) => truthy(
            object::Boolean {
                value: left == right,
            }
            .into(),
        ),
        (TokenKind::NotEqual, _, _) => truthy(
            object::Boolean {
                value: left != right,
            }
            .into(),
        ),
        _ => object::Error {
            message: format!("unknown operator: {} {} {}", left, operator, right),
        }
        .into(),
    }
}

fn eval_integer_infix_expression(
    operator: &TokenKind,
    left: &object::Integer,
    right: &object::Integer,
) -> Object {
    match operator {
        TokenKind::Plus => object::Integer {
            value: left.value + right.value,
        }
        .into(),
        TokenKind::Minus => object::Integer {
            value: left.value - right.value,
        }
        .into(),
        TokenKind::Asterisk => object::Integer {
            value: left.value * right.value,
        }
        .into(),
        TokenKind::Slash => object::Integer {
            value: left.value / right.value,
        }
        .into(),
        TokenKind::LessThan => truthy(
            object::Boolean {
                value: left.value < right.value,
            }
            .into(),
        )
        .into(),
        TokenKind::GreaterThan => truthy(
            object::Boolean {
                value: left.value > right.value,
            }
            .into(),
        )
        .into(),
        TokenKind::Equal => truthy(
            object::Boolean {
                value: left.value == right.value,
            }
            .into(),
        )
        .into(),
        TokenKind::NotEqual => truthy(
            object::Boolean {
                value: left.value != right.value,
            }
            .into(),
        )
        .into(),
        _ => NULL,
    }
}

fn eval_if_expression(expr: expression::If, env: &mut Environment) -> Object {
    let condition = eval((*expr.condition).into(), env);

    if let Object::Error(_) = condition {
        return condition;
    }

    if is_truthy(condition) {
        eval(expr.consequence.into(), env)
    } else if expr.alternative.is_some() {
        eval(expr.alternative.unwrap().into(), env)
    } else {
        NULL
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        TRUE => true,
        FALSE => false,
        NULL => false,
        _ => true,
    }
}
