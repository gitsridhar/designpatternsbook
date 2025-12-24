use std::collections::HashMap;

pub struct FoodOrder {
    // Map of item names to their quantities or details
    pub items: HashMap<String, u32>,
}

impl FoodOrder {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, name: &str, quantity: u32) {
        self.items.insert(name.to_string(), quantity);
    }

    pub fn remove_item(&mut self, name: &str) {
        self.items.remove(name);
    }
}
