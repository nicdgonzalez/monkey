use crate::environment::Environment;
use crate::object::Object;

pub trait Evaluate {
    fn evaluate(&self, env: &mut Environment) -> Object;
}
