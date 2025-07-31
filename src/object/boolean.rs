use std::ops;

use crate::expression;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<Boolean> for bool {
    fn from(value: Boolean) -> Self {
        value.value()
    }
}

impl ops::Not for Boolean {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(!self.value)
    }
}
