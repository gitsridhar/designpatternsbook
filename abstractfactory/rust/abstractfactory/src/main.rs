mod fruit;
mod vegetable;
mod factory;

use factory::factory::factory::Factory;

fn main() {
    println!("Hello, world!");

    let ff = factory::factory::factory::FirstFactory{};
    let fruit = ff.consume_fruit();
    let msg = fruit.consume();
    println!("Main: {}", msg);

    println!("-----------");

    let vegetable = ff.consume_vegetable();
    let msg = vegetable.cookandconsume();
    println!("Main: {}", msg);
    vegetable.serve();

    let ff = factory::factory::factory::SecondFactory{};
    let fruit = ff.consume_fruit();
    let msg = fruit.consume();
    println!("Main: {}", msg);

    println!("-----------");

    let vegetable = ff.consume_vegetable();
    let msg = vegetable.cookandconsume();
    println!("Main: {}", msg);
    vegetable.serve();
}
