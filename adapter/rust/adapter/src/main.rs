mod adaptee;
mod adapter;
mod target;

use adaptee::adaptee::Chopper;
use adapter::adapter::NewFoodProcessor;
use target::target::{FoodProcessor, Processor};

fn client(target: impl Processor) {
    println!("'{}'", target.process_food())
}

fn main() {
    println!("Hello, world!");

    let target = FoodProcessor;
    client(target);

    let adaptee = Chopper;
    println!("'{}'",adaptee.chop());

    let adapter = NewFoodProcessor::new(adaptee);
    client(adapter)
}
