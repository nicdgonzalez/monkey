use crate::expression;

pub const NULL: Object = Object::Null(Null {});
pub const TRUE: Object = Object::Boolean(Boolean { value: true });
pub const FALSE: Object = Object::Boolean(Boolean { value: false });

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    Return(Return),
    Error(Error),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(inner) => write!(f, "{}", inner.value),
            Self::Boolean(inner) => write!(f, "{}", inner.value),
            Self::Null(_) => write!(f, "null"),
            Self::Return(inner) => write!(f, "{}", *inner.value),
            Self::Error(inner) => write!(f, "ERROR: {}", inner.message),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub value: i64,
}

impl From<Integer> for Object {
    fn from(value: Integer) -> Self {
        Self::Integer(value)
    }
}

impl From<expression::IntegerLiteral> for Integer {
    fn from(value: expression::IntegerLiteral) -> Self {
        Self { value: value.value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub value: bool,
}

impl From<Boolean> for Object {
    fn from(value: Boolean) -> Self {
        Self::Boolean(value)
    }
}

impl From<expression::Boolean> for Boolean {
    fn from(value: expression::Boolean) -> Self {
        Self { value: value.value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Null;

impl From<Null> for Object {
    fn from(value: Null) -> Self {
        Self::Null(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub value: Box<Object>,
}

impl From<Return> for Object {
    fn from(value: Return) -> Self {
        Self::Return(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub message: String,
}

impl From<Error> for Object {
    fn from(value: Error) -> Self {
        Self::Error(value)
    }
}
