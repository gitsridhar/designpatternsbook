// waiter.rs
use crate::observer::Observer;
use crate::subject::Chef;

pub struct Waiter {
    pub name: String,
    pub chef: Option<Chef>, // Embedding Chef
}

impl Waiter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            chef: None,
        }
    }
}

impl Observer for Waiter {
    fn update(&self, dish_name: &str) {
        println!("Waiter {}: Picking up {} to serve!", self.name, dish_name);
    }
}
