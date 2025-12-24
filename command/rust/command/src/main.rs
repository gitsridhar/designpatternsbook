// main.rs
mod customer;
mod peel;
mod waiter;

use customer::{Customer, CustomerInteraction};
use peel::Peel;
use waiter::Waiter;

fn main() {
    let mut waiter = Waiter::new();

    // Create a customer and an interaction
    let customer1 = Customer { name: String::from("Alice") };
    let interaction1 = CustomerInteraction { customer: customer1 };

    // Create the "Peel" action and add it to the waiter
    let peel_action = Peel { interaction: interaction1 };
    waiter.add_action(Box::new(peel_action));

    // Execute all queued actions
    waiter.serve_all();
}

