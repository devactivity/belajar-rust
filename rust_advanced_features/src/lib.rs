use std::collections::HashMap;

#[derive(Debug)]
pub struct People(HashMap<i32, String>);

impl People {
    pub fn new() -> People {
        People(HashMap::new())
    }

    pub fn add_name(&mut self, name: &str) {
        self.0.insert(1, name.to_string());
    }
}