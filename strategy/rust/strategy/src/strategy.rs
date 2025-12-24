use crate::strategy_interface::StrategyInterface;

pub trait Strategy {
    fn execute_strategy(&self, a: i32, b: i32, interface: &dyn StrategyInterface);
}

// Concrete implementation "OpenStrategy"
pub struct OpenStrategy;
impl Strategy for OpenStrategy {
    fn execute_strategy(&self, a: i32, b: i32, interface: &dyn StrategyInterface) {
        println!("Executing OpenStrategy...");
        interface.perform_operation(a, b);
    }
}

// Concrete implementation "ClosedStrategy"
pub struct ClosedStrategy;
impl Strategy for ClosedStrategy {
    fn execute_strategy(&self, a: i32, b: i32, interface: &dyn StrategyInterface) {
        println!("Executing ClosedStrategy...");
        interface.perform_operation(a, b);
    }
}
