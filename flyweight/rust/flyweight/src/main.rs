
mod food;
mod food_type;
mod food_factory;
mod restaurant;

use restaurant::Restaurant;

fn main() {
    let mut restaurant = Restaurant::new();

    // Place several orders, some of the same type, but with different extrinsic states
    restaurant.place_order("Burger", "Classic beef burger", 10, "Large", 1);
    restaurant.place_order("Pizza", "Pepperoni pizza", 12, "Medium", 2);
    restaurant.place_order("Burger", "Classic beef burger", 10, "Small", 3);
    restaurant.place_order("Salad", "Garden salad", 8, "Medium", 1);
    restaurant.place_order("Pizza", "Pepperoni pizza", 12, "Large", 4);
    restaurant.place_order("Burger", "Classic beef burger", 10, "Large", 4);


    restaurant.fulfill_orders();

    println!("\n--- Memory Usage Report ---");
    restaurant.show_menu_stats();
    println!("Total orders placed: {}", restaurant.orders.len());
}
