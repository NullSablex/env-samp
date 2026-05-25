use std::collections::HashMap;

#[derive(Default)]
pub struct EnvStore {
    vars: HashMap<String, String>,
}

impl EnvStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self, map: HashMap<String, String>) {
        self.vars = map;
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(String::as_str)
    }

    pub fn count(&self) -> usize {
        self.vars.len()
    }
}
