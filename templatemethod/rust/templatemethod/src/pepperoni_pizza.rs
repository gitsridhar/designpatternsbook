use crate::pizza::Pizza;

pub struct PepperoniPizza;

impl Pizza for PepperoniPizza {
    fn add_toppings(&self) {
        println!("Adding Mozzarella cheese and spicy pepperoni slices.");
    }
}
