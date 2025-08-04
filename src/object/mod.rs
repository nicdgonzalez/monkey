mod boolean;
mod error;
mod function;
mod integer;
mod null;
mod r#return;

use std::fmt;

pub use boolean::Boolean;
pub use error::Error;
pub use function::Function;
pub use integer::Integer;
pub use null::Null;
pub use r#return::Return;

pub const NULL: Object = Object::Null(Null::new());
pub const TRUE: Boolean = Boolean::new(true);
pub const FALSE: Boolean = Boolean::new(false);

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Boolean(Boolean),
    Error(Error),
    Function(Function),
    Integer(Integer),
    Null(Null),
    Return(Return),
}

impl Object {
    pub fn as_boolean(&self) -> Boolean {
        match *self {
            Self::Boolean(ref inner) => {
                if inner.value() {
                    TRUE
                } else {
                    FALSE
                }
            }
            Self::Error(_) => FALSE,
            Self::Function(_) => FALSE,
            Self::Integer(ref inner) => {
                if inner.value() > 0 {
                    TRUE
                } else {
                    FALSE
                }
            }
            Self::Null(_) => FALSE,
            Self::Return(ref inner) => inner.value().as_boolean(),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Boolean(ref inner) => inner.value().fmt(f),
            Self::Error(ref inner) => write!(f, "ERROR: {}", inner.message()),
            Self::Function(_) => "function".fmt(f),
            Self::Integer(ref inner) => inner.value().fmt(f),
            Self::Null(_) => "null".fmt(f),
            Self::Return(ref inner) => (*inner.value()).fmt(f),
        }
    }
}

impl From<Boolean> for Object {
    fn from(value: Boolean) -> Self {
        Object::Boolean(value)
    }
}

impl From<Error> for Object {
    fn from(value: Error) -> Self {
        Object::Error(value)
    }
}

impl From<Function> for Object {
    fn from(value: Function) -> Self {
        Object::Function(value)
    }
}

impl From<Integer> for Object {
    fn from(value: Integer) -> Self {
        Object::Integer(value)
    }
}

impl From<Null> for Object {
    fn from(value: Null) -> Self {
        Object::Null(value)
    }
}

impl From<Return> for Object {
    fn from(value: Return) -> Self {
        Object::Return(value)
    }
}
