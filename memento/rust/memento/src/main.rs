mod dish;
mod waiter;
mod chef;

use dish::Dish;
use waiter::Waiter;
use chef::Chef;

fn main() {
    let mut dish = Dish::new("Pasta");
    let mut waiter = Waiter::new();
    let mut chef = Chef::new();

    // Initial State
    println!("Initial: {:?}, {:?}", dish, waiter);

    // Save state before changes
    chef.backup(&dish, &waiter);

    // Perform actions
    dish.stage = "Cooking".to_string();
    waiter.status = "Busy".to_string();
    println!("Modified: {:?}, {:?}", dish, waiter);

    // Undo action
    println!("Undoing changes...");
    chef.undo(&mut dish, &mut waiter);

    println!("Restored: {:?}, {:?}", dish, waiter);
}
