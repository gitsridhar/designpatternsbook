// main.rs
mod observer;
mod subject;
mod waiter;

use subject::{Chef, Subject};
use waiter::Waiter;

fn main() {
    let mut chef = Chef::new();

    // Create observers
    let waiter1 = Box::new(Waiter::new("Alice"));
    let waiter2 = Box::new(Waiter::new("Bob"));

    // Register observers to the Subject (Chef)
    chef.attach(waiter1);
    chef.attach(waiter2);

    // Trigger update
    chef.prepare_dish("Beef Wellington");
}
