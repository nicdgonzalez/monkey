mod boolean;
mod error;
mod integer;
mod null;
mod r#return;

use std::fmt;

pub use boolean::Boolean;
pub use error::Error;
pub use integer::Integer;
pub use null::Null;
pub use r#return::Return;

// TODO: I don't like how it feels to work with these constants...
pub const NULL: Object = Object::Null(Null::new());
pub const TRUE: Boolean = Boolean::new(true);
pub const FALSE: Boolean = Boolean::new(false);

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    Return(Return),
    Error(Error),
}

impl Object {
    pub fn as_boolean(&self) -> Boolean {
        match *self {
            Self::Integer(ref inner) => {
                if inner.value() > 0 {
                    TRUE
                } else {
                    FALSE
                }
            }
            Self::Boolean(ref inner) => {
                if inner.value() {
                    TRUE
                } else {
                    FALSE
                }
            }
            Self::Null(_) => FALSE,
            Self::Return(ref inner) => inner.value().as_boolean(),
            Self::Error(_) => FALSE,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Integer(ref inner) => inner.value().fmt(f),
            Self::Boolean(ref inner) => inner.value().fmt(f),
            Self::Null(_) => "null".fmt(f),
            Self::Return(ref inner) => (*inner.value()).fmt(f),
            Self::Error(ref inner) => write!(f, "ERROR: {}", inner.message()),
        }
    }
}

impl From<Integer> for Object {
    fn from(value: Integer) -> Self {
        Object::Integer(value)
    }
}
impl From<Boolean> for Object {
    fn from(value: Boolean) -> Self {
        Object::Boolean(value)
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
impl From<Error> for Object {
    fn from(value: Error) -> Self {
        Object::Error(value)
    }
}
