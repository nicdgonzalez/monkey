use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug, Default)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new(store: HashMap<String, Object>) -> Self {
        Self { store }
    }
}
