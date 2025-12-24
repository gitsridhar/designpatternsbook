mod restaurant;
mod visitor;

use restaurant::{Restaurant, Restaurant1, Restaurant2};
use visitor::{Visitor, Visitor1, Visitor2};

fn main() {
    let visitor_one = Visitor1;
    let visitor_two = Visitor2;

    let rest_one = Restaurant1;
    let rest_two = Restaurant2;

    let restaurants: Vec<Box<dyn Restaurant>> = vec![
        Box::new(rest_one),
        Box::new(rest_two),
    ];

    println!("--- Visitor 1's Round ---");
    for r in &restaurants {
        r.accept(&visitor_one);
        visitor_one.drink(r.as_ref());
        r.take_payment(&visitor_one);
        println!();
    }

    println!("--- Visitor 2's Round ---");
    for r in &restaurants {
        r.accept(&visitor_two);
        visitor_two.drink(r.as_ref());
        r.take_payment(&visitor_two);
        println!();
    }
}
