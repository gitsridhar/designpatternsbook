mod eating;
mod restaurant;
mod dinner;

use dinner::{Dinner, WeekendDinner};

fn main() {
    let dinner_factory = WeekendDinner;
    
    // Create the 'Eating' object via the factory method
    let eating_instance = dinner_factory.create_dinner();
    
    println!("Menu for the weekend:");
    
    // Use the iterator provided by the Eating implementation
    for dish in eating_instance.get_dishes() {
        println!("- Serving: {}", dish.name);
    }
}

