
use crate::food_type::FoodType;
use std::rc::Rc; // Using Rc for shared ownership of the flyweight

/// The Context (extrinsic state)
pub struct Food {
    // Reference to the shared flyweight object
    food_type: Rc<FoodType>,
    // Extrinsic state (unique to each instance)
    serving_size: String,
    table_number: u32,
}

impl Food {
    pub fn new(food_type: Rc<FoodType>, serving_size: String, table_number: u32) -> Self {
        Self { food_type, serving_size, table_number }
    }

    pub fn serve(&self) {
        // Delegate to the flyweight method, passing the extrinsic state
        self.food_type.display(&self.serving_size, self.table_number);
    }
}
