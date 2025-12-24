mod chef;
mod basic_chef;
mod collecting_ingredients_chef;
mod boiling_chef;
mod frying_chef;
mod master_chef;

use collecting_ingredients_chef::CollectingIngredientsChef;
use boiling_chef::BoilingChef;
use frying_chef::FryingChef;
use master_chef::MasterChef;
use basic_chef::BasicChef;
use chef::Chef;

fn main() {
    let mut collector = CollectingIngredientsChef { base: BasicChef::new() };
    let mut boiler = BoilingChef { base: BasicChef::new() };
    let mut fryer = FryingChef { base: BasicChef::new() };
    let master = MasterChef { base: BasicChef::new() };

    // Build the chain: Collector -> Boiler -> Fryer -> Master
    fryer.set_next(Box::new(master));
    boiler.set_next(Box::new(fryer));
    collector.set_next(Box::new(boiler));

    println!("--- Order: Chicken Soup ---");
    collector.cook("Chicken Soup");

    println!("\n--- Order: Veggie Stir-fry ---");
    collector.cook("Veggie Stir-fry");
}
