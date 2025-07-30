use crate::object::Object;

#[derive(Debug)]
pub struct Return {
    value: Box<Object>,
}
