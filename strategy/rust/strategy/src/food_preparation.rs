use crate::strategy_interface::StrategyInterface;

pub struct FoodPreparation<'a> {
    // Embedding the strategy interface via a trait object
    pub strategy_interface: &'a dyn StrategyInterface,
}

impl<'a> FoodPreparation<'a> {
    pub fn new(interface: &'a dyn StrategyInterface) -> Self {
        Self { strategy_interface: interface }
    }

    pub fn prepare_food(&self, a: i32, b: i32) {
        println!("Preparing food with current interface:");
        self.strategy_interface.perform_operation(a, b);
    }
}
