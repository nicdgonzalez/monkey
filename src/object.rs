pub enum ObjectType {
    Integer,
    Boolean,
    Null,
}

pub trait Object {
    fn get_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn get_type(&self) -> ObjectType {
        ObjectType::Integer
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn get_type(&self) -> ObjectType {
        ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

pub struct Null;

impl Object for Null {
    fn get_type(&self) -> ObjectType {
        ObjectType::Null
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}
