
use crate::hotfood::HotFood;
use crate::coldfood::ColdFood;

pub struct Restaurant {
    pub hot_food: HotFood,
    pub cold_food: ColdFood,
}

impl Restaurant {
    pub fn new(hotFood: HotFood, coldFood: ColdFood) -> Self {
        Restaurant {
            hot_food: hotFood,
            cold_food: coldFood,
        }
    }

    pub fn operation(&self) -> &str {
        let mut result = String::new();
        result.push_str(self.cold_food.washAndRinse());
        result.push_str("\n");
        result.push_str(self.cold_food.wrap());
        result.push_str("\n");
        result.push_str(self.cold_food.freeze());
        result.push_str("\n");
        result.push_str(self.hot_food.unwrap());
        result.push_str("\n");
        result.push_str(self.hot_food.clean());
        result.push_str("\n");
        result.push_str(self.hot_food.cook());
        Box::leak(result.into_boxed_str())
    }
}