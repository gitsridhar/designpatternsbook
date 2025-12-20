
use crate::burger::Burger;

pub struct VegBurger;

impl Burger for VegBurger {
    fn request(&self) {
        println!("VegBurger: Handling request.");
    }
}
