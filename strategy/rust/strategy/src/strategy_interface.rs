pub trait StrategyInterface {
    fn perform_operation(&self, a: i32, b: i32);
}

// "Extended" by OpenPanStrategy
pub struct OpenPanStrategy;
impl StrategyInterface for OpenPanStrategy {
    fn perform_operation(&self, a: i32, b: i32) {
        println!("Cooking in an open pan: result = {}", a + b);
    }
}

// "Extended" by ClosedPanStrategy
pub struct ClosedPanStrategy;
impl StrategyInterface for ClosedPanStrategy {
    fn perform_operation(&self, a: i32, b: i32) {
        println!("Cooking in a closed pan: result = {}", a * b);
    }
}
