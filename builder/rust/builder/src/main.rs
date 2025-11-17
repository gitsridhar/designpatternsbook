pub mod director;
pub mod consumer;

use director::director::Director;
use consumer::consumer::{FruitConsumer, VegetableConsumer};

fn main() {
    let dir = Director{};
    let fruitconsumer = FruitConsumer{};
    dir.start_consuming(&fruitconsumer);

    let vegetableconsumer = VegetableConsumer{};
    dir.start_consuming(&vegetableconsumer);
}
