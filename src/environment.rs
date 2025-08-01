use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug, Clone, Default)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(store: HashMap<String, Object>, outer: Option<Box<Environment>>) -> Self {
        Self { store, outer }
    }

    pub fn store(&self) -> &HashMap<String, Object> {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut HashMap<String, Object> {
        &mut self.store
    }

    pub fn outer(&self) -> Option<&Self> {
        self.outer.as_deref()
    }

    pub fn outer_mut(&mut self) -> Option<&mut Self> {
        self.outer.as_deref_mut()
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        let mut value = self.store.get(name);

        if value.is_none() && self.outer.is_some() {
            value = self.outer.as_ref().unwrap().get(name);
        }

        value
    }
}
