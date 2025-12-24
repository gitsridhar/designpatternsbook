use crate::food_order::FoodOrder;

// The Interpreter trait (Abstract Expression)
pub trait Item {
    fn interpret(&self, context: &FoodOrder) -> bool;
}

pub struct FoodItem {
    pub name: String,
}

impl Item for FoodItem {
    fn interpret(&self, context: &FoodOrder) -> bool {
        context.items.contains_key(&self.name)
    }
}

pub struct DrinkItem {
    pub name: String,
}

impl Item for DrinkItem {
    fn interpret(&self, context: &FoodOrder) -> bool {
        context.items.contains_key(&self.name)
    }
}

// Non-Terminal Expression embedding FoodItem and DrinkItem
pub struct AllFood {
    pub food: FoodItem,
    pub drink: DrinkItem,
}

impl Item for AllFood {
    fn interpret(&self, context: &FoodOrder) -> bool {
        // Interprets as true only if both food and drink are present
        self.food.interpret(context) && self.drink.interpret(context)
    }
}
