
use crate::food::Food;
use crate::food_factory::FoodFactory;
use std::rc::Rc;

pub struct Restaurant {
    factory: FoodFactory,
    pub orders: Vec<Food>,
}

impl Restaurant {
    pub fn new() -> Self {
        Self {
            factory: FoodFactory::new(),
            orders: Vec::new(),
        }
    }

    pub fn place_order(&mut self, name: &str, description: &str, price: u32, serving_size: &str, table_number: u32) {
        // The factory handles getting the shared intrinsic state (FoodType)
        let food_type = self.factory.get_food_type(name, description, price);
        
        // The unique extrinsic state is stored in the specific order (Food)
        let order = Food::new(
            Rc::clone(&food_type),
            serving_size.to_string(),
            table_number,
        );
        self.orders.push(order);
    }

    pub fn fulfill_orders(&self) {
        println!("\n--- Fulfilling Orders ---");
        for order in &self.orders {
            order.serve();
        }
    }

    pub fn show_menu_stats(&self) {
        self.factory.list_cached_food_types();
    }
}
