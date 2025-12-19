mod dish;
mod fruitsalad;
mod maindish;
mod saltandpepper;
mod soup;
mod serving;

use std::{cell::RefCell, rc::Rc};
use crate::dish::Dish;
use crate::fruitsalad::FruitSalad;
use crate::maindish::MainDish;
use crate::saltandpepper::SaltAndPepper;
use crate::soup::Soup;
use crate::serving::Serving;

fn rc<C: Dish + 'static>(c: C) -> Rc<RefCell<dyn Dish>> {
    Rc::new(RefCell::new(c))
}

fn main() {

    let dinner = rc(Serving::new("dinner"));

    let saltandpepper = rc(SaltAndPepper::new("Salt and Pepper"));

    let appetizer = rc(Serving::new("Appetizer"));
    appetizer
        .borrow_mut()
        .add(rc(Soup::new("Soup")));
    appetizer
        .borrow_mut()
        .add(rc(FruitSalad::new("Fruit Salad")));

    let maincourse = rc(Serving::new("Main Course"));
    maincourse
        .borrow_mut()
        .add(rc(MainDish::new("Main Dish")));

    dinner.borrow_mut().add(saltandpepper);
    dinner.borrow_mut().add(appetizer);
    dinner.borrow_mut().add(maincourse);

    // Print structure
    println!("{}", dinner.borrow().operation(0));

    // Demonstrate removal
    println!("Removing 'A2'...");
    dinner.borrow_mut().remove("Salt and Pepper");
    println!("{}", dinner.borrow().operation(0));
}