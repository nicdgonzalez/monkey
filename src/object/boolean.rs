use crate::expression;

#[derive(Debug)]
pub struct Boolean {
    value: bool,
}

impl Boolean {
    pub const fn new(value: bool) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> bool {
        self.value
    }
}

impl From<expression::Boolean> for Boolean {
    fn from(value: expression::Boolean) -> Self {
        Self {
            value: value.value(),
        }
    }
}
