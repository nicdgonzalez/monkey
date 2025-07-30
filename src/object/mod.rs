mod boolean;
mod error;
mod integer;
mod null;
mod r#return;

pub use boolean::Boolean;
pub use error::Error;
pub use integer::Integer;
pub use null::Null;
pub use r#return::Return;

pub const NULL: Object = Object::Null(Null::new());
pub const TRUE: Object = Object::Boolean(Boolean::new(true));
pub const FALSE: Object = Object::Boolean(Boolean::new(false));

#[derive(Debug)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    Return(Return),
    Error(Error),
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
