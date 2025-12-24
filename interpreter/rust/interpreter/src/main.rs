mod food_order;
mod item;

use food_order::FoodOrder;
use item::{Item, FoodItem, DrinkItem, AllFood};

fn main() {
    // 1. Create the Context (the Order)
    let mut my_order = FoodOrder::new();
    my_order.add_item("Burger", 1);
    my_order.add_item("Cola", 1);

    // 2. Define the Expressions (the Items to check for)
    let burger = FoodItem { name: "Burger".to_string() };
    let cola = DrinkItem { name: "Cola".to_string() };
    let salad = FoodItem { name: "Salad".to_string() };

    let combo = AllFood {
        food: FoodItem { name: "Burger".to_string() },
        drink: DrinkItem { name: "Cola".to_string() },
    };

    // 3. Interpret the context using the expressions
    println!("Does order have Burger? {}", burger.interpret(&my_order)); // true
    println!("Does order have Salad? {}", salad.interpret(&my_order));   // false
    println!("Does order have Cola? {}", cola.interpret(&my_order));       // true
    println!("Is the Burger + Cola combo present? {}", combo.interpret(&my_order)); // true
}

