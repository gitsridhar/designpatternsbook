mod pizza;
mod cheese_pizza;
mod pepperoni_pizza;

use pizza::Pizza;
use cheese_pizza::CheesePizza;
use pepperoni_pizza::PepperoniPizza;

fn main() {
    println!("--- Ordering a Cheese Pizza ---");
    let cheese = CheesePizza;
    cheese.make_pizza();

    println!("\n--- Ordering a Pepperoni Pizza ---");
    let pepperoni = PepperoniPizza;
    pepperoni.make_pizza();
}
