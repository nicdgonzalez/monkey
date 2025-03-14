use std::collections::HashMap;

use crate::object;

#[derive(Debug)]
pub struct Environment {
    pub store: HashMap<String, object::Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}
