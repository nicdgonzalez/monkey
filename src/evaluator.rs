use crate::{ast::Node, environment::Environment, object::Object};

pub trait Evaluate {
    fn evaluate(&self, env: &mut Environment) -> Object;
}
