mod coldfood;
mod hotfood;
mod restaurant;

use crate::coldfood::ColdFood;
use crate::hotfood::HotFood;
use crate::restaurant::Restaurant;

fn main() {
    let coldFood = ColdFood{};
    let hotFood = HotFood{};

    let restaurant = Restaurant{hot_food: hotFood, cold_food: coldFood};
    println!("{}",restaurant.operation());
}
