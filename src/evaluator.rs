// TODO: Clean up like I did with parser.

use crate::ast::Node;
use crate::expression::{self, Expression};
use crate::object::{self, Object, FALSE, NULL, TRUE};
use crate::statement::Statement;
use crate::token::TokenKind;

// pub trait Eval {
//     fn eval(&self) -> Object;
// }

pub fn eval(node: Node) -> Object {
    match node {
        Node::Program(program) => eval_statements(program.statements),
        Node::Statement(statement) => match statement {
            Statement::Expression(inner) => eval((*inner.expression).into()),
            Statement::Block(inner) => eval_statements(inner.statements),
            _ => NULL,
        },
        Node::Expression(expression) => match expression {
            Expression::IntegerLiteral(inner) => {
                let value = object::Integer { value: inner.value };
                value.into()
            }
            Expression::Prefix(inner) => {
                let right = eval((*inner.right).into());
                let value = eval_prefix_expression(&inner.token.kind, &right);
                value
            }
            Expression::Infix(inner) => {
                let left = eval((*inner.left).into());
                let right = eval((*inner.right).into());
                eval_infix_expression(&inner.token.kind, &left, &right)
            }
            Expression::If(inner) => eval_if_expression(inner),
            Expression::Boolean(inner) => {
                let value = object::Boolean { value: inner.value };
                truthy(value.into())
            }
            _ => NULL,
        },
    }
}

fn eval_statements(statements: Vec<Statement>) -> Object {
    let mut result = NULL;

    for statement in statements.iter() {
        result = eval(statement.clone().into());
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
    }
}

fn eval_prefix_expression(operator: &TokenKind, right: &Object) -> Object {
    match operator {
        TokenKind::Bang => eval_bang_operator_expression(right),
        TokenKind::Minus => eval_minus_prefix_expression(right),
        _ => NULL,
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
    }
}

fn eval_minus_prefix_expression(right: &Object) -> Object {
    if let Object::Integer(inner) = right {
        let value = object::Integer {
            value: -inner.value,
        };
        value.into()
    } else {
        NULL
    }
}

fn eval_infix_expression(operator: &TokenKind, left: &Object, right: &Object) -> Object {
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
        _ => NULL,
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

fn eval_if_expression(expr: expression::If) -> Object {
    let condition = eval((*expr.condition).into());

    if is_truthy(condition) {
        eval(expr.consequence.into())
    } else if expr.alternative.is_some() {
        eval(expr.alternative.unwrap().into())
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
