use consumer::consumer::Consumer;
use consumer::consumer::{AppleConsumer,BananaConsumer};
//use consumer::consumer::BananaConsumer;

pub(crate) mod consumer;

fn main() {
    println!("Hello, world!");
    let ac = AppleConsumer{};
    println!("{}",ac.consume().to_string());

    let bc = BananaConsumer{};
    println!("{}",bc.consume().to_string());
}
