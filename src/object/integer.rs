use crate::expression;

#[derive(Debug)]
pub struct Integer {
    value: i64,
}

impl Integer {
    pub const fn new(value: i64) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> i64 {
        self.value
    }
}

impl From<expression::IntegerLiteral> for Integer {
    fn from(value: expression::IntegerLiteral) -> Self {
        Self {
            value: value.value(),
        }
    }
}
