use crate::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    value: Box<Object>,
}

impl Return {
    pub const fn new(value: Box<Object>) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Object {
        self.value.as_ref()
    }
}
