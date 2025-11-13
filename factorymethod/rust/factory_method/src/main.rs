use consumer::consumer::Consumer;
use consumer::consumer::AppleConsumer;

pub mod consumer;

fn main() {
    println!("Hello, world!");
    let ac = AppleConsumer{};
    println!("{}",ac.consume().to_string());
}
