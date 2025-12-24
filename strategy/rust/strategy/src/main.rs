mod strategy_interface;
mod strategy;
mod food_preparation;

use strategy_interface::{OpenPanStrategy, ClosedPanStrategy};
use strategy::{Strategy, OpenStrategy, ClosedStrategy};
use food_preparation::FoodPreparation;

fn main() {
    let a = 10;
    let b = 5;

    // 1. Using OpenPanStrategy with OpenStrategy
    let open_pan = OpenPanStrategy;
    let open_strat = OpenStrategy;
    open_strat.execute_strategy(a, b, &open_pan);

    // 2. Using FoodPreparation context with ClosedPanStrategy
    let closed_pan = ClosedPanStrategy;
    let chef = FoodPreparation::new(&closed_pan);
    chef.prepare_food(a, b);

    // 3. Demonstrating ClosedStrategy execution
    let closed_strat = ClosedStrategy;
    closed_strat.execute_strategy(a, b, &open_pan);
}
