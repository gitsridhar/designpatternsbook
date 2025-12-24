
use crate::food_type::FoodType;
use std::collections::HashMap;
use std::rc::Rc;

/// The Flyweight Factory
pub struct FoodFactory {
    cache: HashMap<String, Rc<FoodType>>,
}

impl FoodFactory {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    /// Gets an existing FoodType or creates a new one if it doesn't exist
    pub fn get_food_type(&mut self, name: &str, description: &str, price: u32) -> Rc<FoodType> {
        if let Some(food_type) = self.cache.get(name) {
            // Reuse existing flyweight
            Rc::clone(food_type)
        } else {
            // Create a new flyweight and cache it
            let food_type = Rc::new(FoodType::new(
                name.to_string(),
                description.to_string(),
                price,
            ));
            self.cache.insert(name.to_string(), Rc::clone(&food_type));
            food_type
        }
    }

    pub fn list_cached_food_types(&self) {
        println!("Currently cached food types: {}", self.cache.len());
        for (name, _) in &self.cache {
            println!("- {}", name);
        }
    }
}
