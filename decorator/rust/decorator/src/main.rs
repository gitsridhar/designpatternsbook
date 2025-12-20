mod sauce;
mod food;
mod strawberry;
mod hotsauce;
mod chocolatesauce;

use crate::sauce::Sauce;
use crate::food::Food;
use crate::strawberry::Strawberry;
use crate::hotsauce::HotSauce;
use crate::chocolatesauce::ChocolateSauce;

fn main() {
    let sauce = Sauce::new();
    println!("{}", sauce.dip());

    let strawberry = Strawberry::new();
    println!("{}", strawberry.dip());

    let hotsauce = HotSauce::new(&strawberry);
    println!("{}", hotsauce.dip());

    let chocolatesauce = ChocolateSauce::new(&strawberry);
    println!("{}", chocolatesauce.dip());
}
