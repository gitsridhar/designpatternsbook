use crate::pizza::Pizza;

pub struct CheesePizza;

impl Pizza for CheesePizza {
    fn add_toppings(&self) {
        println!("Adding a generous layer of Mozzarella cheese.");
    }
}
